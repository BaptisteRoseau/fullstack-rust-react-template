use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::parse::ModelInfo;
use crate::type_mapping::{read_param_to_crud_value, read_param_type};

pub fn generate(model: &ModelInfo) -> TokenStream {
    let struct_ident = &model.struct_ident;
    let table = &model.table_name;

    let methods: Vec<TokenStream> = model
        .all_fields
        .iter()
        .map(|field| {
            let field_name = field.ident.to_string();
            let method_name = format_ident!("read_by_{}", field_name);
            let param_type = read_param_type(field);
            let param_ident = &field.ident;
            let crud_value = read_param_to_crud_value(field);

            let sql = format!("SELECT * FROM {table} WHERE {field_name} = $1");

            if field_name == "id" {
                // read_by_id returns a single result
                quote! {
                    pub async fn #method_name(
                        db: &impl crate::crud::CrudExecutor,
                        #param_ident: #param_type,
                    ) -> Result<#struct_ident, crate::crud::CrudError> {
                        db.crud_fetch_one::<#struct_ident>(#sql, vec![#crud_value]).await
                    }
                }
            } else {
                // Other read_by_X return Vec
                quote! {
                    pub async fn #method_name(
                        db: &impl crate::crud::CrudExecutor,
                        #param_ident: #param_type,
                    ) -> Result<Vec<#struct_ident>, crate::crud::CrudError> {
                        db.crud_fetch_all::<#struct_ident>(#sql, vec![#crud_value]).await
                    }
                }
            }
        })
        .collect();

    quote! { #(#methods)* }
}
