//! Expansion of `#[test_trait_suite]` and the orphaned `#[test_trait]` marker.

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::punctuated::Punctuated;
use syn::{
    Attribute, Error, FnArg, Ident, Item, ItemFn, ItemMod, Result, Token, Type,
    TypeParamBound, parse_quote, parse2,
};

const MARKER: &str = "test_trait";

/// The trait a suite drives, as written in its tests' signatures.
type Bounds = Punctuated<TypeParamBound, Token![+]>;

/// `#[test_trait]` reached without having been collected.
///
/// The enclosing module attribute rewrites every marker it collects to carry an
/// argument, so a bare marker means this test sits outside a `#[test_trait_suite]`
/// module and would never have run. The rewrite — rather than stripping the marker —
/// is also what keeps the user's `use test_trait::test_trait;` a real use.
pub(crate) fn marker(args: TokenStream, input: TokenStream) -> TokenStream {
    if !args.is_empty() {
        return input;
    }

    let message = "#[test_trait] only has meaning inside a #[test_trait_suite] module; \
                   without one the test is never collected";
    let error = match parse2::<ItemFn>(input.clone()) {
        Ok(function) => Error::new_spanned(function.sig.ident, message),
        Err(_) => Error::new_spanned(input, message),
    };
    error.to_compile_error()
}

pub(crate) fn expand(input: TokenStream) -> TokenStream {
    match try_expand(input) {
        Ok(tokens) => tokens,
        Err(error) => error.to_compile_error(),
    }
}

fn try_expand(input: TokenStream) -> Result<TokenStream> {
    let mut module: ItemMod = parse2(input)?;

    let Some((_, items)) = &mut module.content else {
        return Err(Error::new_spanned(
            &module,
            "#[test_trait_suite] needs the module body to read its tests: \
             write `mod suite { … }`, not `mod suite;`",
        ));
    };

    let mut tests = Vec::new();
    for item in items.iter_mut() {
        let Item::Fn(function) = item else { continue };
        if take_marker(&mut function.attrs) {
            tests.push(TestFn::parse(function)?);
        }
    }

    let Some(first) = tests.first() else {
        return Err(Error::new_spanned(
            &module.ident,
            "no #[test_trait] in this module, so the suite would run nothing",
        ));
    };

    let subject = agree(&tests, "subject", |test| Some(&test.subject.bounds))?
        .unwrap_or_else(|| first.subject.bounds.clone());
    let context = agree(&tests, "context", |test| {
        test.context.as_ref().map(|param| &param.bounds)
    })?;

    items.extend(generate(&tests, &subject, context.as_ref()));
    Ok(quote!(#module))
}

/* =======================================================================================
 * PARSING
 * ===================================================================================== */

/// How a test wants its subject handed over.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Receiver {
    Shared,
    Mut,
    Owned,
}

struct Param {
    receiver: Receiver,
    bounds: Bounds,
}

struct TestFn {
    ident: Ident,
    subject: Param,
    context: Option<Param>,
}

impl TestFn {
    fn parse(function: &ItemFn) -> Result<Self> {
        let ident = function.sig.ident.clone();

        if function.sig.asyncness.is_none() {
            return Err(Error::new_spanned(
                &ident,
                "a #[test_trait] must be `async`: the collector awaits it",
            ));
        }

        let mut params = function.sig.inputs.iter();
        let Some(first) = params.next() else {
            return Err(Error::new_spanned(
                &ident,
                "a #[test_trait] takes the subject under test as its first parameter",
            ));
        };
        let subject = Param::parse(first)?;

        let context = params.next().map(Param::parse).transpose()?;
        if let Some(context) = &context
            && context.receiver != Receiver::Shared
        {
            return Err(Error::new_spanned(
                &ident,
                "the context is shared between every test, \
                 so it must be taken by shared reference",
            ));
        }

        if params.next().is_some() {
            return Err(Error::new_spanned(
                &ident,
                "a #[test_trait] takes at most two parameters: the subject and a context",
            ));
        }

        Ok(Self {
            ident,
            subject,
            context,
        })
    }
}

impl Param {
    fn parse(argument: &FnArg) -> Result<Self> {
        let FnArg::Typed(typed) = argument else {
            return Err(Error::new_spanned(
                argument,
                "a #[test_trait] is a free function, it takes no `self`",
            ));
        };

        let (receiver, inner) = match &*typed.ty {
            Type::Reference(reference) if reference.mutability.is_some() => {
                (Receiver::Mut, &*reference.elem)
            }
            Type::Reference(reference) => (Receiver::Shared, &*reference.elem),
            owned => (Receiver::Owned, owned),
        };

        // A suite exists to run against any backend, so its signatures name the trait
        // and nothing below it. A concrete type compiles and passes while quietly
        // pinning the suite to one implementation — nothing surfaces the mistake until
        // a second backend arrives and the "reusable" suite has to be rewritten.
        let bounds = match inner {
            Type::ImplTrait(impl_trait) => impl_trait.bounds.clone(),
            Type::TraitObject(trait_object) => trait_object.bounds.clone(),
            concrete => {
                return Err(Error::new_spanned(
                    concrete,
                    "a #[test_trait] takes the trait, not a backend: write \
                     `&impl Storage` rather than `&S3`. A concrete subject stops the \
                     suite being reusable across backends, which is its whole purpose",
                ));
            }
        };

        Ok(Self { receiver, bounds })
    }
}

/// Marks every `#[test_trait]` as collected, reporting whether one was there.
///
/// The attribute is rewritten rather than removed so it still resolves through the
/// user's import — an attribute the compiler never sees would make that import look
/// unused — and so a marker without the argument can be reported as orphaned.
fn take_marker(attrs: &mut [Attribute]) -> bool {
    let mut found = false;
    for attr in attrs.iter_mut() {
        if attr.path().is_ident(MARKER) {
            let path = attr.path().clone();
            *attr = parse_quote!(#[#path(collected)]);
            found = true;
        }
    }
    found
}

/// One suite drives one trait, so every test declaring a subject — or a context —
/// has to name the same one. Returns it, or `None` if no test declared it.
fn agree<'a>(
    tests: &'a [TestFn],
    what: &str,
    of: impl Fn(&'a TestFn) -> Option<&'a Bounds>,
) -> Result<Option<Bounds>> {
    let mut declared = tests.iter().filter_map(|test| Some((test, of(test)?)));
    let Some((first, bounds)) = declared.next() else {
        return Ok(None);
    };

    for (test, other) in declared {
        if other != bounds {
            return Err(Error::new_spanned(
                &test.ident,
                format!(
                    "every #[test_trait] taking a {what} must take the same one, \
                     but this differs from `{}`",
                    first.ident
                ),
            ));
        }
    }
    Ok(Some(bounds.clone()))
}

/* =======================================================================================
 * CODE GENERATION
 * ===================================================================================== */

/// The signature fragments both collectors share.
struct Shape {
    /// `S: Storage, C: ProviderAgent + Send + Sync + 'static,`
    generics: TokenStream,
    context_param: TokenStream,
    context_clone: TokenStream,
}

/// `subject_extra` is the bound the subject picks up from how the collector holds it:
/// `trials_shared` keeps it in an `Arc` across threads, `trials` builds it in place.
fn shape(
    subject: &Bounds,
    context: Option<&Bounds>,
    subject_extra: Option<TokenStream>,
) -> Shape {
    let shared = quote!(Send + Sync + 'static);
    let mut generics = vec![generic("S", subject, subject_extra)];
    generics.extend(context.map(|context| generic("C", context, Some(shared))));

    Shape {
        generics: quote!(#(#generics,)*),
        context_param: match context {
            Some(_) => quote!(, context: ::std::sync::Arc<C>),
            None => TokenStream::new(),
        },
        context_clone: match context {
            Some(_) => quote!(let context = ::std::sync::Arc::clone(&context);),
            None => TokenStream::new(),
        },
    }
}

fn generic(name: &str, bounds: &Bounds, extra: Option<TokenStream>) -> TokenStream {
    let ident = format_ident!("{name}");
    match extra {
        Some(extra) => quote!(#ident: #bounds + #extra),
        None => quote!(#ident: #bounds),
    }
}

fn generate(tests: &[TestFn], subject: &Bounds, context: Option<&Bounds>) -> Vec<Item> {
    let mut items = vec![trials(tests, subject, context)];

    // `&mut` and by-value subjects cannot come out of an `Arc`, so a suite that wants
    // one of those can only be run against a freshly built subject.
    if tests
        .iter()
        .all(|test| test.subject.receiver == Receiver::Shared)
    {
        items.push(trials_shared(tests, subject, context));
    }
    items
}

/// `trials(rt, build[, context])` — a fresh subject per trial.
fn trials(tests: &[TestFn], subject: &Bounds, context: Option<&Bounds>) -> Item {
    let Shape {
        generics,
        context_param,
        context_clone,
    } = shape(subject, context, None);

    let trials = tests.iter().map(|test| {
        let ident = &test.ident;
        let name = ident.to_string();
        let arguments = test.context.is_some().then(|| quote!(, &*context));
        let call = match test.subject.receiver {
            Receiver::Shared => quote! {
                let subject = build().await;
                #ident(&subject #arguments).await;
            },
            Receiver::Mut => quote! {
                let mut subject = build().await;
                #ident(&mut subject #arguments).await;
            },
            Receiver::Owned => quote! {
                #ident(build().await #arguments).await;
            },
        };
        let context_clone = &context_clone;
        quote! {
            {
                let rt = ::std::sync::Arc::clone(&rt);
                let build = ::std::sync::Arc::clone(&build);
                #context_clone
                ::test_trait::Trial::test(#name, move || {
                    rt.block_on(async move { #call });
                    ::std::result::Result::Ok(())
                })
            }
        }
    });

    parse_quote! {
        /// Every `#[test_trait]` in this module, each against a freshly built subject.
        #[allow(dead_code)]
        pub fn trials<#generics B, F>(
            rt: ::std::sync::Arc<::test_trait::Runtime>,
            build: B
            #context_param
        ) -> ::std::vec::Vec<::test_trait::Trial>
        where
            B: ::std::ops::Fn() -> F + ::std::marker::Send + ::std::marker::Sync + 'static,
            F: ::std::future::Future<Output = S>,
        {
            let build = ::std::sync::Arc::new(build);
            ::std::vec![#(#trials),*]
        }
    }
}

/// `trials_shared(rt, subject[, context])` — one subject for every trial.
fn trials_shared(tests: &[TestFn], subject: &Bounds, context: Option<&Bounds>) -> Item {
    let Shape {
        generics,
        context_param,
        context_clone,
    } = shape(subject, context, Some(quote!(Send + Sync + 'static)));

    let trials = tests.iter().map(|test| {
        let ident = &test.ident;
        let name = ident.to_string();
        let arguments = test.context.is_some().then(|| quote!(, &*context));
        let context_clone = &context_clone;
        quote! {
            {
                let rt = ::std::sync::Arc::clone(&rt);
                let subject = ::std::sync::Arc::clone(&subject);
                #context_clone
                ::test_trait::Trial::test(#name, move || {
                    rt.block_on(async move { #ident(&*subject #arguments).await });
                    ::std::result::Result::Ok(())
                })
            }
        }
    });

    parse_quote! {
        /// Every `#[test_trait]` in this module, all sharing one subject.
        ///
        /// Sound only while the subject is stateless from the suite's point of view:
        /// the trials run in parallel against it.
        #[allow(dead_code)]
        pub fn trials_shared<#generics>(
            rt: ::std::sync::Arc<::test_trait::Runtime>,
            subject: ::std::sync::Arc<S>
            #context_param
        ) -> ::std::vec::Vec<::test_trait::Trial> {
            ::std::vec![#(#trials),*]
        }
    }
}
