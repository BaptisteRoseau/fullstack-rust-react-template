use super::*;
use syn::parse_str;

fn parse_model(code: &str) -> ModelInfo {
    let input: DeriveInput = parse_str(code).unwrap();
    ModelInfo::from_derive_input(&input)
}

#[test]
fn test_basic_struct_parsing() {
    let model = parse_model(
        "struct User {
            id: uuid::Uuid,
            name: String,
            created_at: chrono::DateTime<chrono::Utc>,
            updated_at: chrono::DateTime<chrono::Utc>,
        }",
    );
    assert_eq!(model.struct_ident, "User");
    assert_eq!(model.table_name, "users");
    assert_eq!(model.all_fields.len(), 4);
    // user_fields excludes id, created_at, updated_at
    assert_eq!(model.user_fields.len(), 1);
    assert_eq!(model.user_fields[0].ident, "name");
    assert!(model.id_field.is_some());
}

#[test]
fn test_compound_struct_name() {
    let model = parse_model(
        "struct UserProfile {
            id: uuid::Uuid,
            bio: String,
            created_at: chrono::DateTime<chrono::Utc>,
            updated_at: chrono::DateTime<chrono::Utc>,
        }",
    );
    assert_eq!(model.table_name, "user_profiles");
}

#[test]
fn test_option_field_detection() {
    let model = parse_model(
        "struct Item {
            id: uuid::Uuid,
            name: String,
            description: Option<String>,
            count: i32,
            label: Option<i64>,
            created_at: chrono::DateTime<chrono::Utc>,
            updated_at: chrono::DateTime<chrono::Utc>,
        }",
    );
    let desc = model
        .all_fields
        .iter()
        .find(|f| f.ident == "description")
        .unwrap();
    assert!(desc.is_option);
    assert!(desc.inner_ty.is_some());

    let label = model
        .all_fields
        .iter()
        .find(|f| f.ident == "label")
        .unwrap();
    assert!(label.is_option);

    let name = model.all_fields.iter().find(|f| f.ident == "name").unwrap();
    assert!(!name.is_option);
    assert!(name.inner_ty.is_none());

    let count = model
        .all_fields
        .iter()
        .find(|f| f.ident == "count")
        .unwrap();
    assert!(!count.is_option);
}

#[test]
fn test_auto_fields_filtered() {
    let model = parse_model(
        "struct Product {
            id: uuid::Uuid,
            name: String,
            price: f64,
            active: bool,
            created_at: chrono::DateTime<chrono::Utc>,
            updated_at: chrono::DateTime<chrono::Utc>,
        }",
    );
    let user_field_names: Vec<String> = model
        .user_fields
        .iter()
        .map(|f| f.ident.to_string())
        .collect();
    assert_eq!(user_field_names, vec!["name", "price", "active"]);
    assert!(!user_field_names.contains(&"id".to_string()));
    assert!(!user_field_names.contains(&"created_at".to_string()));
    assert!(!user_field_names.contains(&"updated_at".to_string()));
}

#[test]
fn test_no_id_field() {
    let model = parse_model(
        "struct NoId {
            name: String,
            value: i32,
        }",
    );
    assert!(model.id_field.is_none());
    // all fields are user fields since none are auto
    assert_eq!(model.user_fields.len(), 2);
}

#[test]
fn test_all_fields_are_auto() {
    let model = parse_model(
        "struct Timestamps {
            id: uuid::Uuid,
            created_at: chrono::DateTime<chrono::Utc>,
            updated_at: chrono::DateTime<chrono::Utc>,
        }",
    );
    assert_eq!(model.user_fields.len(), 0);
    assert_eq!(model.all_fields.len(), 3);
}

#[test]
#[should_panic(expected = "Crud derive only supports structs")]
fn test_enum_rejected() {
    let input: DeriveInput = parse_str("enum Foo { A, B }").unwrap();
    ModelInfo::from_derive_input(&input);
}

#[test]
#[should_panic(expected = "Crud derive only supports structs with named fields")]
fn test_tuple_struct_rejected() {
    let input: DeriveInput = parse_str("struct Foo(String, i32);").unwrap();
    ModelInfo::from_derive_input(&input);
}

#[test]
fn test_extract_option_inner_with_string() {
    let ty: Type = parse_str("Option<String>").unwrap();
    let (is_opt, inner) = extract_option_inner(&ty);
    assert!(is_opt);
    assert!(inner.is_some());
}

#[test]
fn test_extract_option_inner_with_non_option() {
    let ty: Type = parse_str("String").unwrap();
    let (is_opt, inner) = extract_option_inner(&ty);
    assert!(!is_opt);
    assert!(inner.is_none());
}

#[test]
fn test_extract_option_inner_with_qualified_path() {
    let ty: Type = parse_str("std::option::Option<i32>").unwrap();
    // Our implementation only checks the last segment name
    let (is_opt, inner) = extract_option_inner(&ty);
    assert!(is_opt);
    assert!(inner.is_some());
}

#[test]
fn test_many_field_types() {
    let model = parse_model(
        "struct AllTypes {
            id: uuid::Uuid,
            s: String,
            b: bool,
            n32: i32,
            n64: i64,
            f: f64,
            opt_s: Option<String>,
            opt_b: Option<bool>,
            dt: chrono::DateTime<chrono::Utc>,
            created_at: chrono::DateTime<chrono::Utc>,
            updated_at: chrono::DateTime<chrono::Utc>,
        }",
    );
    assert_eq!(model.all_fields.len(), 11);
    // user_fields: s, b, n32, n64, f, opt_s, opt_b, dt = 8
    assert_eq!(model.user_fields.len(), 8);
}
