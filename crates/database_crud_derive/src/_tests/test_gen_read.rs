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
