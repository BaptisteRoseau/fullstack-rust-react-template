use super::*;
use crate::parse::{FieldInfo, extract_option_inner};
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::parse_str;

fn make_field(name: &str, type_str: &str) -> FieldInfo {
    let ty: Type = parse_str(type_str).unwrap();
    let (is_option, inner_ty) = extract_option_inner(&ty);
    FieldInfo {
        ident: Ident::new(name, Span::call_site()),
        ty,
        is_option,
        inner_ty,
    }
}

fn tokens_contain(ts: &TokenStream, needle: &str) -> bool {
    ts.to_string().contains(needle)
}

// --- extract_base_type_name ---

#[test]
fn test_extract_base_simple() {
    let ty: Type = parse_str("String").unwrap();
    assert_eq!(extract_base_type_name(&ty), "String");
}

#[test]
fn test_extract_base_qualified() {
    let ty: Type = parse_str("uuid::Uuid").unwrap();
    assert_eq!(extract_base_type_name(&ty), "Uuid");
}

#[test]
fn test_extract_base_deeply_qualified() {
    let ty: Type = parse_str("chrono::DateTime<chrono::Utc>").unwrap();
    assert_eq!(extract_base_type_name(&ty), "DateTime");
}

#[test]
fn test_extract_base_option() {
    let ty: Type = parse_str("Option<String>").unwrap();
    assert_eq!(extract_base_type_name(&ty), "Option");
}

#[test]
fn test_extract_base_primitive() {
    for (input, expected) in [
        ("bool", "bool"),
        ("i32", "i32"),
        ("i64", "i64"),
        ("f64", "f64"),
    ] {
        let ty: Type = parse_str(input).unwrap();
        assert_eq!(extract_base_type_name(&ty), expected);
    }
}

// --- to_crud_value (non-option types) ---

#[test]
fn test_to_crud_value_string() {
    let field = make_field("name", "String");
    let expr = quote! { val };
    let result = to_crud_value(&field, &expr);
    assert!(tokens_contain(&result, "CrudValue :: String"));
}

#[test]
fn test_to_crud_value_uuid() {
    let field = make_field("id", "uuid::Uuid");
    let expr = quote! { val };
    let result = to_crud_value(&field, &expr);
    assert!(tokens_contain(&result, "CrudValue :: Uuid"));
}

#[test]
fn test_to_crud_value_datetime() {
    let field = make_field("ts", "chrono::DateTime<chrono::Utc>");
    let expr = quote! { val };
    let result = to_crud_value(&field, &expr);
    assert!(tokens_contain(&result, "CrudValue :: DateTime"));
}

#[test]
fn test_to_crud_value_bool() {
    let field = make_field("active", "bool");
    let expr = quote! { val };
    let result = to_crud_value(&field, &expr);
    assert!(tokens_contain(&result, "CrudValue :: Bool"));
}

#[test]
fn test_to_crud_value_i32() {
    let field = make_field("count", "i32");
    let expr = quote! { val };
    let result = to_crud_value(&field, &expr);
    assert!(tokens_contain(&result, "CrudValue :: I32"));
}

#[test]
fn test_to_crud_value_i64() {
    let field = make_field("big", "i64");
    let expr = quote! { val };
    let result = to_crud_value(&field, &expr);
    assert!(tokens_contain(&result, "CrudValue :: I64"));
}

#[test]
fn test_to_crud_value_f64() {
    let field = make_field("price", "f64");
    let expr = quote! { val };
    let result = to_crud_value(&field, &expr);
    assert!(tokens_contain(&result, "CrudValue :: F64"));
}

#[test]
fn test_to_crud_value_unsupported() {
    let field = make_field("data", "Vec<u8>");
    let expr = quote! { val };
    let result = to_crud_value(&field, &expr);
    assert!(tokens_contain(&result, "compile_error"));
    assert!(tokens_contain(&result, "Unsupported type"));
}

// --- to_crud_value (option types) ---

#[test]
fn test_to_crud_value_option_string() {
    let field = make_field("addr", "Option<String>");
    let expr = quote! { val };
    let result = to_crud_value(&field, &expr);
    assert!(tokens_contain(&result, "CrudValue :: OptionString"));
}

#[test]
fn test_to_crud_value_option_datetime() {
    let field = make_field("deleted_at", "Option<chrono::DateTime<chrono::Utc>>");
    let expr = quote! { val };
    let result = to_crud_value(&field, &expr);
    assert!(tokens_contain(&result, "CrudValue :: OptionDateTime"));
}

#[test]
fn test_to_crud_value_option_bool() {
    let field = make_field("flag", "Option<bool>");
    let expr = quote! { val };
    let result = to_crud_value(&field, &expr);
    assert!(tokens_contain(&result, "CrudValue :: OptionBool"));
}

#[test]
fn test_to_crud_value_option_i32() {
    let field = make_field("count", "Option<i32>");
    let expr = quote! { val };
    let result = to_crud_value(&field, &expr);
    assert!(tokens_contain(&result, "CrudValue :: OptionI32"));
}

#[test]
fn test_to_crud_value_option_i64() {
    let field = make_field("big", "Option<i64>");
    let expr = quote! { val };
    let result = to_crud_value(&field, &expr);
    assert!(tokens_contain(&result, "CrudValue :: OptionI64"));
}

#[test]
fn test_to_crud_value_option_f64() {
    let field = make_field("val", "Option<f64>");
    let expr = quote! { val };
    let result = to_crud_value(&field, &expr);
    assert!(tokens_contain(&result, "CrudValue :: OptionF64"));
}

#[test]
fn test_to_crud_value_option_uuid_unsupported() {
    let field = make_field("ref_id", "Option<uuid::Uuid>");
    let expr = quote! { val };
    let result = to_crud_value(&field, &expr);
    assert!(tokens_contain(&result, "compile_error"));
}

#[test]
fn test_to_crud_value_option_unsupported() {
    let field = make_field("data", "Option<Vec<u8>>");
    let expr = quote! { val };
    let result = to_crud_value(&field, &expr);
    assert!(tokens_contain(&result, "compile_error"));
    assert!(tokens_contain(&result, "Unsupported Option inner type"));
}

// --- create_param_type ---

#[test]
fn test_create_param_type_string() {
    let field = make_field("name", "String");
    let result = create_param_type(&field);
    assert!(tokens_contain(&result, "impl Into < String >"));
}

#[test]
fn test_create_param_type_uuid() {
    let field = make_field("id", "uuid::Uuid");
    let result = create_param_type(&field);
    assert!(tokens_contain(&result, "Uuid"));
}

#[test]
fn test_create_param_type_option_string() {
    let field = make_field("addr", "Option<String>");
    let result = create_param_type(&field);
    assert!(tokens_contain(&result, "Option < String >"));
}

#[test]
fn test_create_param_type_option_non_string() {
    let field = make_field("count", "Option<i32>");
    let result = create_param_type(&field);
    assert!(tokens_contain(&result, "Option < i32 >"));
}

#[test]
fn test_create_param_type_bool() {
    let field = make_field("active", "bool");
    let result = create_param_type(&field);
    assert_eq!(result.to_string(), "bool");
}

// --- create_param_to_crud_value ---

#[test]
fn test_create_param_to_crud_value_string() {
    let field = make_field("name", "String");
    let result = create_param_to_crud_value(&field);
    assert!(tokens_contain(&result, "CrudValue :: String"));
    assert!(tokens_contain(&result, "name . into ()"));
}

#[test]
fn test_create_param_to_crud_value_uuid() {
    let field = make_field("id", "uuid::Uuid");
    let result = create_param_to_crud_value(&field);
    assert!(tokens_contain(&result, "CrudValue :: Uuid"));
}

#[test]
fn test_create_param_to_crud_value_option_string() {
    let field = make_field("addr", "Option<String>");
    let result = create_param_to_crud_value(&field);
    assert!(tokens_contain(&result, "CrudValue :: OptionString"));
}

#[test]
fn test_create_param_to_crud_value_unsupported() {
    let field = make_field("data", "Vec<u8>");
    let result = create_param_to_crud_value(&field);
    assert!(tokens_contain(&result, "compile_error"));
}

#[test]
fn test_create_param_to_crud_value_option_unsupported() {
    let field = make_field("data", "Option<Vec<u8>>");
    let result = create_param_to_crud_value(&field);
    assert!(tokens_contain(&result, "compile_error"));
}

// --- read_param_type ---

#[test]
fn test_read_param_type_string() {
    let field = make_field("name", "String");
    let result = read_param_type(&field);
    assert_eq!(result.to_string(), "& str");
}

#[test]
fn test_read_param_type_uuid() {
    let field = make_field("id", "uuid::Uuid");
    let result = read_param_type(&field);
    assert!(tokens_contain(&result, "Uuid"));
}

#[test]
fn test_read_param_type_option_string() {
    let field = make_field("addr", "Option<String>");
    let result = read_param_type(&field);
    assert!(tokens_contain(&result, "Option < & str >"));
}

#[test]
fn test_read_param_type_option_i32() {
    let field = make_field("count", "Option<i32>");
    let result = read_param_type(&field);
    assert!(tokens_contain(&result, "Option < i32 >"));
}

// --- read_param_to_crud_value ---

#[test]
fn test_read_param_to_crud_value_string() {
    let field = make_field("name", "String");
    let result = read_param_to_crud_value(&field);
    assert!(tokens_contain(&result, "CrudValue :: String"));
    assert!(tokens_contain(&result, "to_string"));
}

#[test]
fn test_read_param_to_crud_value_uuid() {
    let field = make_field("id", "uuid::Uuid");
    let result = read_param_to_crud_value(&field);
    assert!(tokens_contain(&result, "CrudValue :: Uuid"));
}

#[test]
fn test_read_param_to_crud_value_option_string() {
    let field = make_field("addr", "Option<String>");
    let result = read_param_to_crud_value(&field);
    assert!(tokens_contain(&result, "CrudValue :: OptionString"));
    assert!(tokens_contain(&result, "to_string"));
}

#[test]
fn test_read_param_to_crud_value_option_i64() {
    let field = make_field("big", "Option<i64>");
    let result = read_param_to_crud_value(&field);
    assert!(tokens_contain(&result, "CrudValue :: OptionI64"));
}

#[test]
fn test_read_param_to_crud_value_unsupported() {
    let field = make_field("data", "Vec<u8>");
    let result = read_param_to_crud_value(&field);
    assert!(tokens_contain(&result, "compile_error"));
}

#[test]
fn test_read_param_to_crud_value_option_unsupported() {
    let field = make_field("data", "Option<Vec<u8>>");
    let result = read_param_to_crud_value(&field);
    assert!(tokens_contain(&result, "compile_error"));
}

// --- all numeric option variants for create_param_to_crud_value ---

#[test]
fn test_create_param_to_crud_value_option_datetime() {
    let field = make_field("ts", "Option<chrono::DateTime<chrono::Utc>>");
    let result = create_param_to_crud_value(&field);
    assert!(tokens_contain(&result, "CrudValue :: OptionDateTime"));
}

#[test]
fn test_create_param_to_crud_value_option_bool() {
    let field = make_field("flag", "Option<bool>");
    let result = create_param_to_crud_value(&field);
    assert!(tokens_contain(&result, "CrudValue :: OptionBool"));
}

#[test]
fn test_create_param_to_crud_value_option_i32() {
    let field = make_field("n", "Option<i32>");
    let result = create_param_to_crud_value(&field);
    assert!(tokens_contain(&result, "CrudValue :: OptionI32"));
}

#[test]
fn test_create_param_to_crud_value_option_i64() {
    let field = make_field("n", "Option<i64>");
    let result = create_param_to_crud_value(&field);
    assert!(tokens_contain(&result, "CrudValue :: OptionI64"));
}

#[test]
fn test_create_param_to_crud_value_option_f64() {
    let field = make_field("f", "Option<f64>");
    let result = create_param_to_crud_value(&field);
    assert!(tokens_contain(&result, "CrudValue :: OptionF64"));
}

// --- all non-option variants for create_param_to_crud_value ---

#[test]
fn test_create_param_to_crud_value_datetime() {
    let field = make_field("ts", "chrono::DateTime<chrono::Utc>");
    let result = create_param_to_crud_value(&field);
    assert!(tokens_contain(&result, "CrudValue :: DateTime"));
}

#[test]
fn test_create_param_to_crud_value_bool() {
    let field = make_field("flag", "bool");
    let result = create_param_to_crud_value(&field);
    assert!(tokens_contain(&result, "CrudValue :: Bool"));
}

#[test]
fn test_create_param_to_crud_value_i32() {
    let field = make_field("n", "i32");
    let result = create_param_to_crud_value(&field);
    assert!(tokens_contain(&result, "CrudValue :: I32"));
}

#[test]
fn test_create_param_to_crud_value_i64() {
    let field = make_field("n", "i64");
    let result = create_param_to_crud_value(&field);
    assert!(tokens_contain(&result, "CrudValue :: I64"));
}

#[test]
fn test_create_param_to_crud_value_f64() {
    let field = make_field("f", "f64");
    let result = create_param_to_crud_value(&field);
    assert!(tokens_contain(&result, "CrudValue :: F64"));
}

// --- all read_param_to_crud_value variants ---

#[test]
fn test_read_param_to_crud_value_datetime() {
    let field = make_field("ts", "chrono::DateTime<chrono::Utc>");
    let result = read_param_to_crud_value(&field);
    assert!(tokens_contain(&result, "CrudValue :: DateTime"));
}

#[test]
fn test_read_param_to_crud_value_bool() {
    let field = make_field("flag", "bool");
    let result = read_param_to_crud_value(&field);
    assert!(tokens_contain(&result, "CrudValue :: Bool"));
}

#[test]
fn test_read_param_to_crud_value_i32() {
    let field = make_field("n", "i32");
    let result = read_param_to_crud_value(&field);
    assert!(tokens_contain(&result, "CrudValue :: I32"));
}

#[test]
fn test_read_param_to_crud_value_f64() {
    let field = make_field("f", "f64");
    let result = read_param_to_crud_value(&field);
    assert!(tokens_contain(&result, "CrudValue :: F64"));
}

#[test]
fn test_read_param_to_crud_value_option_datetime() {
    let field = make_field("ts", "Option<chrono::DateTime<chrono::Utc>>");
    let result = read_param_to_crud_value(&field);
    assert!(tokens_contain(&result, "CrudValue :: OptionDateTime"));
}

#[test]
fn test_read_param_to_crud_value_option_bool() {
    let field = make_field("flag", "Option<bool>");
    let result = read_param_to_crud_value(&field);
    assert!(tokens_contain(&result, "CrudValue :: OptionBool"));
}

#[test]
fn test_read_param_to_crud_value_option_i32() {
    let field = make_field("n", "Option<i32>");
    let result = read_param_to_crud_value(&field);
    assert!(tokens_contain(&result, "CrudValue :: OptionI32"));
}

#[test]
fn test_read_param_to_crud_value_option_f64() {
    let field = make_field("f", "Option<f64>");
    let result = read_param_to_crud_value(&field);
    assert!(tokens_contain(&result, "CrudValue :: OptionF64"));
}
