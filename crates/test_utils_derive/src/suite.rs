//! Expansion of `#[trait_test_suite]` and the orphaned `#[trait_test]` marker.

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::punctuated::Punctuated;
use syn::{
    Attribute, Error, FnArg, Ident, Item, ItemFn, ItemMod, Result, Token, Type,
    TypeParamBound, parse_quote, parse2,
};

const MARKER: &str = "trait_test";

/// `#[trait_test]` reached without having been collected.
///
/// The enclosing module attribute rewrites every marker it collects to carry an
/// argument, so a bare marker means this test sits outside a `#[trait_test_suite]`
/// module and would never have run. The rewrite — rather than stripping the marker —
/// is also what keeps the user's `use test_utils::trait_test;` a real use.
pub(crate) fn marker(args: TokenStream, input: TokenStream) -> TokenStream {
    if !args.is_empty() {
        return input;
    }

    let message = "#[trait_test] only has meaning inside a #[trait_test_suite] module; \
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
            "#[trait_test_suite] needs the module body to read its tests: \
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

    if tests.is_empty() {
        return Err(Error::new_spanned(
            &module.ident,
            "no #[trait_test] in this module, so the suite would run nothing",
        ));
    }

    let subject = agree_on(&tests, |test| &test.subject, "subject")?;
    let context = tests.iter().find_map(|test| test.context.as_ref());
    if context.is_some() {
        agree_on_context(&tests)?;
    }

    items.extend(generate(&tests, subject, context)?);
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

/// What the parameter's type says the subject is.
#[derive(Clone, PartialEq)]
enum SubjectType {
    /// `impl Trait`: the collector stays generic over the backend.
    Generic(Punctuated<TypeParamBound, Token![+]>),
    /// A named type: the collector is pinned to it.
    Concrete(Type),
}

#[derive(Clone)]
struct Param {
    receiver: Receiver,
    subject: SubjectType,
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
                &function.sig.ident,
                "a #[trait_test] must be `async`: the collector awaits it",
            ));
        }

        let mut params = function.sig.inputs.iter();
        let Some(first) = params.next() else {
            return Err(Error::new_spanned(
                &function.sig.ident,
                "a #[trait_test] takes the subject under test as its first parameter",
            ));
        };
        let subject = Param::parse(first)?;

        let context = match params.next() {
            Some(second) => {
                let context = Param::parse(second)?;
                if context.receiver != Receiver::Shared {
                    return Err(Error::new_spanned(
                        second,
                        "the context parameter is shared between every test, \
                         so it must be taken by shared reference",
                    ));
                }
                Some(context)
            }
            None => None,
        };

        if params.next().is_some() {
            return Err(Error::new_spanned(
                &function.sig.ident,
                "a #[trait_test] takes at most two parameters: the subject and a context",
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
                "a #[trait_test] is a free function, it takes no `self`",
            ));
        };

        let (receiver, inner) = match &*typed.ty {
            Type::Reference(reference) => {
                let receiver = if reference.mutability.is_some() {
                    Receiver::Mut
                } else {
                    Receiver::Shared
                };
                (receiver, &*reference.elem)
            }
            owned => (Receiver::Owned, owned),
        };

        let subject = match inner {
            Type::ImplTrait(impl_trait) => {
                SubjectType::Generic(impl_trait.bounds.clone())
            }
            concrete => SubjectType::Concrete(concrete.clone()),
        };

        Ok(Self { receiver, subject })
    }
}

/// Marks every `#[trait_test]` as collected, reporting whether one was there.
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

/// Every test in a suite drives the same backend, so their parameter types have to
/// line up — the collector can only be generated for one of them.
fn agree_on<'a>(
    tests: &'a [TestFn],
    extract: impl Fn(&'a TestFn) -> &'a Param,
    what: &str,
) -> Result<&'a SubjectType> {
    let first = extract(&tests[0]);
    for test in &tests[1..] {
        if extract(test).subject != first.subject {
            return Err(Error::new_spanned(
                &test.ident,
                format!(
                    "every #[trait_test] in a suite must take the same {what} type, \
                     but this one differs from `{}`",
                    tests[0].ident
                ),
            ));
        }
    }
    Ok(&first.subject)
}

fn agree_on_context(tests: &[TestFn]) -> Result<()> {
    let mut contexts = tests.iter().filter(|test| test.context.is_some());
    let Some(first) = contexts.next() else {
        return Ok(());
    };
    let expected = &first.context.as_ref().expect("filtered").subject;

    for test in contexts {
        if &test.context.as_ref().expect("filtered").subject != expected {
            return Err(Error::new_spanned(
                &test.ident,
                format!(
                    "every #[trait_test] taking a context must take the same type, \
                     but this one differs from `{}`",
                    first.ident
                ),
            ));
        }
    }
    Ok(())
}

/* =======================================================================================
 * CODE GENERATION
 * ===================================================================================== */

/// A subject or context type rendered for the generated signature: either a fresh
/// generic parameter carrying the `impl Trait` bounds, or the concrete type itself.
struct Rendered {
    /// `S` / `C`, or the concrete type.
    ty: TokenStream,
    /// The generic parameter declaration, if one was introduced.
    generic: Option<TokenStream>,
}

fn render(subject: &SubjectType, name: &str, extra: Option<TokenStream>) -> Rendered {
    match subject {
        SubjectType::Generic(bounds) => {
            let ident = format_ident!("{name}");
            let bounds = match &extra {
                Some(extra) => quote!(#bounds + #extra),
                None => quote!(#bounds),
            };
            Rendered {
                ty: quote!(#ident),
                generic: Some(quote!(#ident: #bounds)),
            }
        }
        SubjectType::Concrete(ty) => Rendered {
            ty: quote!(#ty),
            generic: None,
        },
    }
}

fn generate(
    tests: &[TestFn],
    subject: &SubjectType,
    context: Option<&Param>,
) -> Result<Vec<Item>> {
    let context = context.map(|param| &param.subject);

    let mut items = vec![trials(tests, subject, context)];
    if tests
        .iter()
        .all(|test| test.subject.receiver == Receiver::Shared)
    {
        items.push(trials_shared(tests, subject, context));
    }
    Ok(items)
}

/// The pieces a generated collector needs about its subject and context types.
struct Shape {
    generics: TokenStream,
    subject_ty: TokenStream,
    context_param: TokenStream,
    context_clone: TokenStream,
}

/// Builds the signature fragments shared by both collectors. `subject_extra` is the
/// bound the subject picks up from how the collector holds it — `trials_shared` keeps
/// it in an `Arc` across threads, `trials` builds it inside the trial.
fn shape(
    subject: &SubjectType,
    context: Option<&SubjectType>,
    subject_extra: Option<TokenStream>,
) -> Shape {
    let shared = quote!(Send + Sync + 'static);
    let subject = render(subject, "S", subject_extra);
    let context = context.map(|context| render(context, "C", Some(shared)));

    let mut generics: Vec<TokenStream> = Vec::new();
    generics.extend(subject.generic);
    generics.extend(context.as_ref().and_then(|context| context.generic.clone()));

    let context_param = context
        .as_ref()
        .map(|context| {
            let ty = &context.ty;
            quote!(, context: ::std::sync::Arc<#ty>)
        })
        .unwrap_or_default();

    Shape {
        generics: quote!(#(#generics,)*),
        subject_ty: subject.ty,
        context_param,
        context_clone: if context.is_some() {
            quote!(let context = ::std::sync::Arc::clone(&context);)
        } else {
            TokenStream::new()
        },
    }
}

/// `trials(rt, build[, context])` — a fresh subject per trial.
fn trials(
    tests: &[TestFn],
    subject: &SubjectType,
    context: Option<&SubjectType>,
) -> Item {
    let Shape {
        generics,
        subject_ty,
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
                ::test_utils::Trial::test(#name, move || {
                    rt.block_on(async move { #call });
                    ::std::result::Result::Ok(())
                })
            }
        }
    });

    parse_quote! {
        /// Every `#[trait_test]` in this module, each against a freshly built subject.
        #[allow(dead_code)]
        pub fn trials<#generics B, F>(
            rt: ::std::sync::Arc<::test_utils::Runtime>,
            build: B
            #context_param
        ) -> ::std::vec::Vec<::test_utils::Trial>
        where
            B: ::std::ops::Fn() -> F + ::std::marker::Send + ::std::marker::Sync + 'static,
            F: ::std::future::Future<Output = #subject_ty>,
        {
            let build = ::std::sync::Arc::new(build);
            ::std::vec![#(#trials),*]
        }
    }
}

/// `trials_shared(rt, subject[, context])` — one subject for every trial.
fn trials_shared(
    tests: &[TestFn],
    subject: &SubjectType,
    context: Option<&SubjectType>,
) -> Item {
    let Shape {
        generics,
        subject_ty,
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
                ::test_utils::Trial::test(#name, move || {
                    rt.block_on(async move { #ident(&*subject #arguments).await });
                    ::std::result::Result::Ok(())
                })
            }
        }
    });

    parse_quote! {
        /// Every `#[trait_test]` in this module, all sharing one subject.
        ///
        /// Sound only while the subject is stateless from the suite's point of view:
        /// the trials run in parallel against it.
        #[allow(dead_code)]
        pub fn trials_shared<#generics>(
            rt: ::std::sync::Arc<::test_utils::Runtime>,
            subject: ::std::sync::Arc<#subject_ty>
            #context_param
        ) -> ::std::vec::Vec<::test_utils::Trial> {
            ::std::vec![#(#trials),*]
        }
    }
}
