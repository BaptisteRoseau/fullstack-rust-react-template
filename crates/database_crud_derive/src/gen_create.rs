use proc_macro2::TokenStream;
use quote::quote;

use crate::parse::ModelInfo;
use crate::type_mapping::{create_param_to_crud_value, create_param_type};

pub fn generate(model: &ModelInfo) -> TokenStream {
    let struct_ident = &model.struct_ident;
    let table = &model.table_name;

    let field_names: Vec<String> = model.user_fields.iter().map(|f| f.ident.to_string()).collect();
    let columns = field_names.join(", ");

    let placeholders: Vec<String> = (1..=model.user_fields.len())
        .map(|i| format!("${i}"))
        .collect();
    let placeholders_str = placeholders.join(", ");

    let sql = format!(
        "INSERT INTO {table} ({columns}) VALUES ({placeholders_str}) RETURNING *"
    );

    let params: Vec<TokenStream> = model
        .user_fields
        .iter()
        .map(|f| {
            let ident = &f.ident;
            let ty = create_param_type(f);
            quote! { #ident: #ty }
        })
        .collect();

    let args: Vec<TokenStream> = model
        .user_fields
        .iter()
        .map(create_param_to_crud_value)
        .collect();

    quote! {
        pub async fn create(
            db: &impl crate::crud::CrudExecutor,
            #(#params),*
        ) -> Result<#struct_ident, crate::crud::CrudError> {
            let args = vec![#(#args),*];
            db.crud_fetch_one::<#struct_ident>(#sql, args).await
        }
    }
}
