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
fn extract_base_type_name(ty: &Type) -> String {
    if let Type::Path(type_path) = ty
        && let Some(segment) = type_path.path.segments.last()
    {
        return segment.ident.to_string();
    }
    String::new()
}
