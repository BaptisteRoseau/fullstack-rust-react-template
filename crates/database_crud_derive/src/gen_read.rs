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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::ModelInfo;
    use syn::{DeriveInput, parse_str};

    fn parse_model(code: &str) -> ModelInfo {
        let input: DeriveInput = parse_str(code).unwrap();
        ModelInfo::from_derive_input(&input)
    }

    fn tokens_str(ts: &TokenStream) -> String {
        ts.to_string()
    }

    #[test]
    fn test_read_by_id_returns_single() {
        let model = parse_model(
            "struct User {
                id: uuid::Uuid,
                name: String,
                created_at: chrono::DateTime<chrono::Utc>,
                updated_at: chrono::DateTime<chrono::Utc>,
            }",
        );
        let output = generate(&model);
        let s = tokens_str(&output);
        assert!(s.contains("fn read_by_id"));
        assert!(s.contains("SELECT * FROM users WHERE id = $1"));
        // read_by_id returns Result<User, ...> not Vec
        assert!(s.contains("crud_fetch_one"));
    }

    #[test]
    fn test_read_by_other_returns_vec() {
        let model = parse_model(
            "struct User {
                id: uuid::Uuid,
                email: String,
                created_at: chrono::DateTime<chrono::Utc>,
                updated_at: chrono::DateTime<chrono::Utc>,
            }",
        );
        let output = generate(&model);
        let s = tokens_str(&output);
        assert!(s.contains("fn read_by_email"));
        assert!(s.contains("SELECT * FROM users WHERE email = $1"));
        assert!(s.contains("crud_fetch_all"));
    }

    #[test]
    fn test_generates_method_for_all_fields() {
        let model = parse_model(
            "struct User {
                id: uuid::Uuid,
                name: String,
                email: String,
                address: Option<String>,
                created_at: chrono::DateTime<chrono::Utc>,
                updated_at: chrono::DateTime<chrono::Utc>,
            }",
        );
        let output = generate(&model);
        let s = tokens_str(&output);
        assert!(s.contains("fn read_by_id"));
        assert!(s.contains("fn read_by_name"));
        assert!(s.contains("fn read_by_email"));
        assert!(s.contains("fn read_by_address"));
        assert!(s.contains("fn read_by_created_at"));
        assert!(s.contains("fn read_by_updated_at"));
    }

    #[test]
    fn test_read_by_string_takes_str_ref() {
        let model = parse_model(
            "struct User {
                id: uuid::Uuid,
                name: String,
                created_at: chrono::DateTime<chrono::Utc>,
                updated_at: chrono::DateTime<chrono::Utc>,
            }",
        );
        let output = generate(&model);
        let s = tokens_str(&output);
        // read_by_name should take &str
        assert!(s.contains("name : & str"));
    }

    #[test]
    fn test_read_by_option_string_takes_option_str_ref() {
        let model = parse_model(
            "struct User {
                id: uuid::Uuid,
                address: Option<String>,
                created_at: chrono::DateTime<chrono::Utc>,
                updated_at: chrono::DateTime<chrono::Utc>,
            }",
        );
        let output = generate(&model);
        let s = tokens_str(&output);
        assert!(s.contains("address : Option < & str >"));
    }
}
