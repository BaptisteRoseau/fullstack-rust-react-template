use proc_macro2::TokenStream;
use quote::quote;

use crate::parse::ModelInfo;

pub fn generate(model: &ModelInfo) -> TokenStream {
    let table = &model.table_name;
    let id_field = model.id_field.as_ref().expect("Crud derive requires an 'id' field");
    let id_ty = &id_field.ty;

    let sql = format!("DELETE FROM {table} WHERE id = $1");

    quote! {
        pub async fn delete(
            db: &impl crate::crud::CrudExecutor,
            id: #id_ty,
        ) -> Result<u64, crate::crud::CrudError> {
            db.crud_execute(#sql, vec![crate::crud::CrudValue::Uuid(id)]).await
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::ModelInfo;
    use syn::{parse_str, DeriveInput};

    fn parse_model(code: &str) -> ModelInfo {
        let input: DeriveInput = parse_str(code).unwrap();
        ModelInfo::from_derive_input(&input)
    }

    #[test]
    fn test_delete_sql() {
        let model = parse_model(
            "struct User {
                id: uuid::Uuid,
                name: String,
                created_at: chrono::DateTime<chrono::Utc>,
                updated_at: chrono::DateTime<chrono::Utc>,
            }",
        );
        let output = generate(&model);
        let s = output.to_string();
        assert!(s.contains("DELETE FROM users WHERE id = $1"));
        assert!(s.contains("pub async fn delete"));
        assert!(s.contains("crud_execute"));
        assert!(s.contains("CrudValue :: Uuid"));
    }

    #[test]
    #[should_panic(expected = "Crud derive requires an 'id' field")]
    fn test_delete_panics_without_id() {
        let model = parse_model(
            "struct NoId {
                name: String,
            }",
        );
        generate(&model);
    }
}
