use super::*;
use crate::parse::ModelInfo;
use syn::{DeriveInput, parse_str};

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
