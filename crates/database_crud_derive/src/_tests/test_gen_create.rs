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
    assert!(s.contains("INSERT INTO users (name, email) VALUES ($1, $2) RETURNING *"));
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
    assert!(s.contains("INSERT INTO users (name, address) VALUES ($1, $2) RETURNING *"));
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
