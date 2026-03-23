use proc_macro2::TokenStream;
use quote::quote;
use syn::Type;

use crate::parse::FieldInfo;

/// Returns the token stream to convert a field value into a CrudValue variant.
/// `value_expr` is the expression holding the value (e.g. an ident).
pub fn to_crud_value(field: &FieldInfo, value_expr: &TokenStream) -> TokenStream {
    let type_name = extract_base_type_name(&field.ty);

    if field.is_option {
        let inner_name = field
            .inner_ty
            .as_ref()
            .map(extract_base_type_name)
            .unwrap_or_default();
        match inner_name.as_str() {
            "String" => quote! { crate::crud::CrudValue::OptionString(#value_expr) },
            "Uuid" => {
                // Option<Uuid> not in CrudValue yet, but we can handle as Option<String> via to_string
                // For now, panic at compile time — extend CrudValue if needed
                quote! { compile_error!("Option<Uuid> not yet supported in CrudValue") }
            }
            "DateTime" => quote! { crate::crud::CrudValue::OptionDateTime(#value_expr) },
            "bool" => quote! { crate::crud::CrudValue::OptionBool(#value_expr) },
            "i32" => quote! { crate::crud::CrudValue::OptionI32(#value_expr) },
            "i64" => quote! { crate::crud::CrudValue::OptionI64(#value_expr) },
            "f64" => quote! { crate::crud::CrudValue::OptionF64(#value_expr) },
            other => {
                let msg = format!("Unsupported Option inner type for CrudValue: {other}");
                let lit = proc_macro2::Literal::string(&msg);
                quote! { compile_error!(#lit) }
            }
        }
    } else {
        match type_name.as_str() {
            "String" => quote! { crate::crud::CrudValue::String(#value_expr) },
            "Uuid" => quote! { crate::crud::CrudValue::Uuid(#value_expr) },
            "DateTime" => quote! { crate::crud::CrudValue::DateTime(#value_expr) },
            "bool" => quote! { crate::crud::CrudValue::Bool(#value_expr) },
            "i32" => quote! { crate::crud::CrudValue::I32(#value_expr) },
            "i64" => quote! { crate::crud::CrudValue::I64(#value_expr) },
            "f64" => quote! { crate::crud::CrudValue::F64(#value_expr) },
            other => {
                let msg = format!("Unsupported type for CrudValue: {other}");
                let lit = proc_macro2::Literal::string(&msg);
                quote! { compile_error!(#lit) }
            }
        }
    }
}

/// Returns the parameter type for a create function argument.
/// String -> impl Into<String>, Option<String> -> Option<impl Into<String>>,
/// Uuid -> uuid::Uuid, etc.
pub fn create_param_type(field: &FieldInfo) -> TokenStream {
    if field.is_option {
        let inner_name = field
            .inner_ty
            .as_ref()
            .map(extract_base_type_name)
            .unwrap_or_default();
        match inner_name.as_str() {
            "String" => quote! { Option<String> },
            _ => {
                let inner_ty = field.inner_ty.as_ref().unwrap();
                quote! { Option<#inner_ty> }
            }
        }
    } else {
        let type_name = extract_base_type_name(&field.ty);
        match type_name.as_str() {
            "String" => quote! { impl Into<String> },
            _ => {
                let ty = &field.ty;
                quote! { #ty }
            }
        }
    }
}

/// Returns the conversion expression from parameter to CrudValue for create.
/// For String params using impl Into<String>, we call .into() first.
pub fn create_param_to_crud_value(field: &FieldInfo) -> TokenStream {
    let ident = &field.ident;

    if field.is_option {
        let inner_name = field
            .inner_ty
            .as_ref()
            .map(extract_base_type_name)
            .unwrap_or_default();
        match inner_name.as_str() {
            "String" => quote! { crate::crud::CrudValue::OptionString(#ident) },
            "DateTime" => quote! { crate::crud::CrudValue::OptionDateTime(#ident) },
            "bool" => quote! { crate::crud::CrudValue::OptionBool(#ident) },
            "i32" => quote! { crate::crud::CrudValue::OptionI32(#ident) },
            "i64" => quote! { crate::crud::CrudValue::OptionI64(#ident) },
            "f64" => quote! { crate::crud::CrudValue::OptionF64(#ident) },
            _ => quote! { compile_error!("Unsupported type") },
        }
    } else {
        let type_name = extract_base_type_name(&field.ty);
        match type_name.as_str() {
            "String" => quote! { crate::crud::CrudValue::String(#ident.into()) },
            "Uuid" => quote! { crate::crud::CrudValue::Uuid(#ident) },
            "DateTime" => quote! { crate::crud::CrudValue::DateTime(#ident) },
            "bool" => quote! { crate::crud::CrudValue::Bool(#ident) },
            "i32" => quote! { crate::crud::CrudValue::I32(#ident) },
            "i64" => quote! { crate::crud::CrudValue::I64(#ident) },
            "f64" => quote! { crate::crud::CrudValue::F64(#ident) },
            _ => quote! { compile_error!("Unsupported type") },
        }
    }
}

/// Returns the parameter type for a read_by function argument.
/// String -> &str, Option<String> -> Option<&str>, Uuid -> uuid::Uuid (Copy), etc.
pub fn read_param_type(field: &FieldInfo) -> TokenStream {
    if field.is_option {
        let inner_name = field
            .inner_ty
            .as_ref()
            .map(extract_base_type_name)
            .unwrap_or_default();
        match inner_name.as_str() {
            "String" => quote! { Option<&str> },
            _ => {
                let inner_ty = field.inner_ty.as_ref().unwrap();
                quote! { Option<#inner_ty> }
            }
        }
    } else {
        let type_name = extract_base_type_name(&field.ty);
        match type_name.as_str() {
            "String" => quote! { &str },
            _ => {
                let ty = &field.ty;
                quote! { #ty }
            }
        }
    }
}

/// Returns the conversion expression from read parameter to CrudValue.
pub fn read_param_to_crud_value(field: &FieldInfo) -> TokenStream {
    let ident = &field.ident;

    if field.is_option {
        let inner_name = field
            .inner_ty
            .as_ref()
            .map(extract_base_type_name)
            .unwrap_or_default();
        match inner_name.as_str() {
            "String" => quote! { crate::crud::CrudValue::OptionString(#ident.map(|s| s.to_string())) },
            "DateTime" => quote! { crate::crud::CrudValue::OptionDateTime(#ident) },
            "bool" => quote! { crate::crud::CrudValue::OptionBool(#ident) },
            "i32" => quote! { crate::crud::CrudValue::OptionI32(#ident) },
            "i64" => quote! { crate::crud::CrudValue::OptionI64(#ident) },
            "f64" => quote! { crate::crud::CrudValue::OptionF64(#ident) },
            _ => quote! { compile_error!("Unsupported type") },
        }
    } else {
        let type_name = extract_base_type_name(&field.ty);
        match type_name.as_str() {
            "String" => quote! { crate::crud::CrudValue::String(#ident.to_string()) },
            "Uuid" => quote! { crate::crud::CrudValue::Uuid(#ident) },
            "DateTime" => quote! { crate::crud::CrudValue::DateTime(#ident) },
            "bool" => quote! { crate::crud::CrudValue::Bool(#ident) },
            "i32" => quote! { crate::crud::CrudValue::I32(#ident) },
            "i64" => quote! { crate::crud::CrudValue::I64(#ident) },
            "f64" => quote! { crate::crud::CrudValue::F64(#ident) },
            _ => quote! { compile_error!("Unsupported type") },
        }
    }
}

/// Extract the last segment of a type path (e.g. "uuid::Uuid" -> "Uuid", "String" -> "String")
pub(crate) fn extract_base_type_name(ty: &Type) -> String {
    if let Type::Path(type_path) = ty
        && let Some(segment) = type_path.path.segments.last()
    {
        return segment.ident.to_string();
    }
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::{extract_option_inner, FieldInfo};
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
        for (input, expected) in [("bool", "bool"), ("i32", "i32"), ("i64", "i64"), ("f64", "f64")]
        {
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
}
