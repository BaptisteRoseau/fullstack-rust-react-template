use proc_macro2::TokenStream;
use quote::quote;

use crate::parse::ModelInfo;
use crate::type_mapping::{create_param_to_crud_value, create_param_type};

pub fn generate(model: &ModelInfo) -> TokenStream {
    let struct_ident = &model.struct_ident;
    let table = &model.table_name;

    let field_names: Vec<String> = model
        .user_fields
        .iter()
        .map(|f| f.ident.to_string())
        .collect();
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
    fn test_create_sql_basic() {
        let model = parse_model(
            "struct User {
                id: uuid::Uuid,
                name: String,
                email: String,
                created_at: chrono::DateTime<chrono::Utc>,
                updated_at: chrono::DateTime<chrono::Utc>,
            }",
        );
        let output = generate(&model);
        let s = tokens_str(&output);
        assert!(
            s.contains("INSERT INTO users (name, email) VALUES ($1, $2) RETURNING *")
        );
        assert!(s.contains("pub async fn create"));
        assert!(s.contains("CrudExecutor"));
    }

    #[test]
    fn test_create_with_option_field() {
        let model = parse_model(
            "struct User {
                id: uuid::Uuid,
                name: String,
                address: Option<String>,
                created_at: chrono::DateTime<chrono::Utc>,
                updated_at: chrono::DateTime<chrono::Utc>,
            }",
        );
        let output = generate(&model);
        let s = tokens_str(&output);
        assert!(
            s.contains("INSERT INTO users (name, address) VALUES ($1, $2) RETURNING *")
        );
        assert!(s.contains("Option < String >"));
    }

    #[test]
    fn test_create_multiple_types() {
        let model = parse_model(
            "struct Product {
                id: uuid::Uuid,
                name: String,
                price: f64,
                count: i32,
                active: bool,
                created_at: chrono::DateTime<chrono::Utc>,
                updated_at: chrono::DateTime<chrono::Utc>,
            }",
        );
        let output = generate(&model);
        let s = tokens_str(&output);
        assert!(s.contains(
            "INSERT INTO products (name, price, count, active) VALUES ($1, $2, $3, $4) RETURNING *"
        ));
    }

    #[test]
    fn test_create_no_user_fields() {
        let model = parse_model(
            "struct Minimal {
                id: uuid::Uuid,
                created_at: chrono::DateTime<chrono::Utc>,
                updated_at: chrono::DateTime<chrono::Utc>,
            }",
        );
        let output = generate(&model);
        let s = tokens_str(&output);
        assert!(s.contains("INSERT INTO minimals () VALUES () RETURNING *"));
    }
}
