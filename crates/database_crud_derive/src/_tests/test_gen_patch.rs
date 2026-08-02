use super::*;
use crate::parse::ModelInfo;
use syn::{DeriveInput, parse_str};

fn parse_model(code: &str) -> ModelInfo {
    let input: DeriveInput = parse_str(code).unwrap();
    ModelInfo::from_derive_input(&input)
}

#[test]
fn test_patch_struct_generated() {
    let model = parse_model(
        "struct User {
            id: uuid::Uuid,
            name: String,
            email: String,
            created_at: chrono::DateTime<chrono::Utc>,
            updated_at: chrono::DateTime<chrono::Utc>,
        }",
    );
    let (patch_struct, _) = generate(&model);
    let s = patch_struct.to_string();
    assert!(s.contains("pub struct UserPatch"));
    assert!(s.contains("pub name : Option < String >"));
    assert!(s.contains("pub email : Option < String >"));
}

#[test]
fn test_patch_option_field_double_option() {
    let model = parse_model(
        "struct User {
            id: uuid::Uuid,
            address: Option<String>,
            created_at: chrono::DateTime<chrono::Utc>,
            updated_at: chrono::DateTime<chrono::Utc>,
        }",
    );
    let (patch_struct, _) = generate(&model);
    let s = patch_struct.to_string();
    // Option<String> field should become Option<Option<String>> in patch
    // quote! serializes as "Option < Option < String >>"
    assert!(s.contains("pub address : Option < Option < String >>"));
}

#[test]
fn test_patch_setters_generated() {
    let model = parse_model(
        "struct User {
            id: uuid::Uuid,
            name: String,
            count: i32,
            address: Option<String>,
            created_at: chrono::DateTime<chrono::Utc>,
            updated_at: chrono::DateTime<chrono::Utc>,
        }",
    );
    let (patch_struct, _) = generate(&model);
    let s = patch_struct.to_string();
    assert!(s.contains("fn set_name"));
    assert!(s.contains("fn set_count"));
    assert!(s.contains("fn set_address"));
}

#[test]
fn test_patch_string_setter_uses_into() {
    let model = parse_model(
        "struct User {
            id: uuid::Uuid,
            name: String,
            created_at: chrono::DateTime<chrono::Utc>,
            updated_at: chrono::DateTime<chrono::Utc>,
        }",
    );
    let (patch_struct, _) = generate(&model);
    let s = patch_struct.to_string();
    assert!(s.contains("impl Into < String >"));
}

#[test]
fn test_patch_option_string_setter() {
    let model = parse_model(
        "struct User {
            id: uuid::Uuid,
            address: Option<String>,
            created_at: chrono::DateTime<chrono::Utc>,
            updated_at: chrono::DateTime<chrono::Utc>,
        }",
    );
    let (patch_struct, _) = generate(&model);
    let s = patch_struct.to_string();
    // Option<String> setter takes Option<impl Into<String>>
    assert!(s.contains("Option < impl Into < String >>"));
}

#[test]
fn test_patch_non_string_option_setter() {
    let model = parse_model(
        "struct User {
            id: uuid::Uuid,
            count: Option<i32>,
            created_at: chrono::DateTime<chrono::Utc>,
            updated_at: chrono::DateTime<chrono::Utc>,
        }",
    );
    let (patch_struct, _) = generate(&model);
    let s = patch_struct.to_string();
    assert!(s.contains("fn set_count (mut self , v : Option < i32 >)"));
}

#[test]
fn test_patch_execute_has_update_sql() {
    let model = parse_model(
        "struct User {
            id: uuid::Uuid,
            name: String,
            created_at: chrono::DateTime<chrono::Utc>,
            updated_at: chrono::DateTime<chrono::Utc>,
        }",
    );
    let (patch_struct, _) = generate(&model);
    let s = patch_struct.to_string();
    assert!(s.contains("UPDATE {} SET {} WHERE id = ${} RETURNING *"));
    assert!(s.contains("users"));
}

#[test]
fn test_patch_execute_fallback_select() {
    let model = parse_model(
        "struct User {
            id: uuid::Uuid,
            name: String,
            created_at: chrono::DateTime<chrono::Utc>,
            updated_at: chrono::DateTime<chrono::Utc>,
        }",
    );
    let (patch_struct, _) = generate(&model);
    let s = patch_struct.to_string();
    // When no fields set, falls back to SELECT
    assert!(s.contains("SELECT * FROM users WHERE id = $1"));
}

#[test]
fn test_build_patch_method() {
    let model = parse_model(
        "struct User {
            id: uuid::Uuid,
            name: String,
            created_at: chrono::DateTime<chrono::Utc>,
            updated_at: chrono::DateTime<chrono::Utc>,
        }",
    );
    let (_, model_methods) = generate(&model);
    let s = model_methods.to_string();
    assert!(s.contains("pub fn build_patch"));
    assert!(s.contains("UserPatch"));
    assert!(s.contains("name : None"));
}

#[test]
fn test_patch_method() {
    let model = parse_model(
        "struct User {
            id: uuid::Uuid,
            name: String,
            created_at: chrono::DateTime<chrono::Utc>,
            updated_at: chrono::DateTime<chrono::Utc>,
        }",
    );
    let (_, model_methods) = generate(&model);
    let s = model_methods.to_string();
    assert!(s.contains("pub fn patch (& self)"));
    assert!(s.contains("id : self . id"));
}

#[test]
#[should_panic(expected = "Crud derive requires an 'id' field")]
fn test_patch_panics_without_id() {
    let model = parse_model(
        "struct NoId {
            name: String,
        }",
    );
    generate(&model);
}

#[test]
fn test_patch_non_string_setter() {
    let model = parse_model(
        "struct User {
            id: uuid::Uuid,
            count: i32,
            created_at: chrono::DateTime<chrono::Utc>,
            updated_at: chrono::DateTime<chrono::Utc>,
        }",
    );
    let (patch_struct, _) = generate(&model);
    let s = patch_struct.to_string();
    // Non-string setter takes the type directly
    assert!(s.contains("fn set_count (mut self , v : i32)"));
}

#[test]
fn test_patch_execute_arms_for_option_field() {
    let model = parse_model(
        "struct User {
            id: uuid::Uuid,
            address: Option<String>,
            created_at: chrono::DateTime<chrono::Utc>,
            updated_at: chrono::DateTime<chrono::Utc>,
        }",
    );
    let (patch_struct, _) = generate(&model);
    let s = patch_struct.to_string();
    // Option field uses OptionString in the execute arms
    assert!(s.contains("CrudValue :: OptionString"));
}

#[test]
fn test_patch_execute_arms_for_non_option_field() {
    let model = parse_model(
        "struct User {
            id: uuid::Uuid,
            name: String,
            created_at: chrono::DateTime<chrono::Utc>,
            updated_at: chrono::DateTime<chrono::Utc>,
        }",
    );
    let (patch_struct, _) = generate(&model);
    let s = patch_struct.to_string();
    // Non-option field uses String (not OptionString) in execute arms
    assert!(s.contains("CrudValue :: String (v)"));
}

#[test]
fn test_patch_execute_arms_i64_field() {
    let model = parse_model(
        "struct Item {
            id: uuid::Uuid,
            big: i64,
            created_at: chrono::DateTime<chrono::Utc>,
            updated_at: chrono::DateTime<chrono::Utc>,
        }",
    );
    let (patch_struct, _) = generate(&model);
    let s = patch_struct.to_string();
    assert!(s.contains("CrudValue :: I64 (v)"));
}

#[test]
fn test_patch_execute_arms_f64_field() {
    let model = parse_model(
        "struct Item {
            id: uuid::Uuid,
            price: f64,
            created_at: chrono::DateTime<chrono::Utc>,
            updated_at: chrono::DateTime<chrono::Utc>,
        }",
    );
    let (patch_struct, _) = generate(&model);
    let s = patch_struct.to_string();
    assert!(s.contains("CrudValue :: F64 (v)"));
}

#[test]
fn test_patch_execute_arms_bool_field() {
    let model = parse_model(
        "struct Item {
            id: uuid::Uuid,
            active: bool,
            created_at: chrono::DateTime<chrono::Utc>,
            updated_at: chrono::DateTime<chrono::Utc>,
        }",
    );
    let (patch_struct, _) = generate(&model);
    let s = patch_struct.to_string();
    assert!(s.contains("CrudValue :: Bool (v)"));
}

#[test]
fn test_patch_execute_arms_uuid_field() {
    let model = parse_model(
        "struct Item {
            id: uuid::Uuid,
            ref_id: uuid::Uuid,
            created_at: chrono::DateTime<chrono::Utc>,
            updated_at: chrono::DateTime<chrono::Utc>,
        }",
    );
    let (patch_struct, _) = generate(&model);
    let s = patch_struct.to_string();
    assert!(s.contains("CrudValue :: Uuid (v)"));
}

#[test]
fn test_patch_execute_arms_datetime_field() {
    let model = parse_model(
        "struct Item {
            id: uuid::Uuid,
            deadline: chrono::DateTime<chrono::Utc>,
            created_at: chrono::DateTime<chrono::Utc>,
            updated_at: chrono::DateTime<chrono::Utc>,
        }",
    );
    let (patch_struct, _) = generate(&model);
    let s = patch_struct.to_string();
    // The execute arm for a non-option DateTime field
    assert!(s.contains("CrudValue :: DateTime (v)"));
}

#[test]
fn test_patch_execute_arms_i32_field() {
    let model = parse_model(
        "struct Item {
            id: uuid::Uuid,
            count: i32,
            created_at: chrono::DateTime<chrono::Utc>,
            updated_at: chrono::DateTime<chrono::Utc>,
        }",
    );
    let (patch_struct, _) = generate(&model);
    let s = patch_struct.to_string();
    assert!(s.contains("CrudValue :: I32 (v)"));
}

#[test]
fn test_patch_option_non_string_fields() {
    let model = parse_model(
        "struct Item {
            id: uuid::Uuid,
            opt_i32: Option<i32>,
            opt_i64: Option<i64>,
            opt_f64: Option<f64>,
            opt_bool: Option<bool>,
            opt_dt: Option<chrono::DateTime<chrono::Utc>>,
            created_at: chrono::DateTime<chrono::Utc>,
            updated_at: chrono::DateTime<chrono::Utc>,
        }",
    );
    let (patch_struct, _) = generate(&model);
    let s = patch_struct.to_string();
    assert!(s.contains("CrudValue :: OptionI32"));
    assert!(s.contains("CrudValue :: OptionI64"));
    assert!(s.contains("CrudValue :: OptionF64"));
    assert!(s.contains("CrudValue :: OptionBool"));
    assert!(s.contains("CrudValue :: OptionDateTime"));
}
