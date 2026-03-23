mod gen_create;
mod gen_delete;
mod gen_patch;
mod gen_read;
mod parse;
mod table_name;
mod type_mapping;

use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

use parse::ModelInfo;

#[proc_macro_derive(Crud)]
pub fn derive_crud(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let model = ModelInfo::from_derive_input(&input);

    let struct_ident = &model.struct_ident;

    let create_fn = gen_create::generate(&model);
    let read_fns = gen_read::generate(&model);
    let delete_fn = gen_delete::generate(&model);
    let (patch_struct_and_impl, patch_methods_on_model) = gen_patch::generate(&model);

    let output = quote! {
        impl #struct_ident {
            #create_fn
            #read_fns
            #delete_fn
            #patch_methods_on_model
        }

        #patch_struct_and_impl
    };

    output.into()
}
