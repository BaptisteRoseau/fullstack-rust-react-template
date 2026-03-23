use proc_macro2::Ident;
use syn::{Data, DeriveInput, Fields, GenericArgument, PathArguments, Type};

use crate::table_name::derive_table_name;

const AUTO_FIELDS: &[&str] = &["id", "created_at", "updated_at"];

pub struct ModelInfo {
    pub struct_ident: Ident,
    pub table_name: String,
    pub all_fields: Vec<FieldInfo>,
    pub user_fields: Vec<FieldInfo>,
    pub id_field: Option<FieldInfo>,
}

#[derive(Clone)]
pub struct FieldInfo {
    pub ident: Ident,
    pub ty: Type,
    pub is_option: bool,
    pub inner_ty: Option<Type>,
}

impl ModelInfo {
    pub fn from_derive_input(input: &DeriveInput) -> Self {
        let struct_ident = input.ident.clone();
        let table_name = derive_table_name(&struct_ident.to_string());

        let fields = match &input.data {
            Data::Struct(data) => match &data.fields {
                Fields::Named(named) => &named.named,
                _ => panic!("Crud derive only supports structs with named fields"),
            },
            _ => panic!("Crud derive only supports structs"),
        };

        let all_fields: Vec<FieldInfo> = fields
            .iter()
            .map(|f| {
                let ident = f.ident.clone().unwrap();
                let ty = f.ty.clone();
                let (is_option, inner_ty) = extract_option_inner(&ty);
                FieldInfo {
                    ident,
                    ty,
                    is_option,
                    inner_ty,
                }
            })
            .collect();

        let id_field = all_fields.iter().find(|f| f.ident == "id").cloned();

        let user_fields: Vec<FieldInfo> = all_fields
            .iter()
            .filter(|f| !AUTO_FIELDS.contains(&f.ident.to_string().as_str()))
            .cloned()
            .collect();

        ModelInfo {
            struct_ident,
            table_name,
            all_fields,
            user_fields,
            id_field,
        }
    }
}

pub(crate) fn extract_option_inner(ty: &Type) -> (bool, Option<Type>) {
    let Type::Path(type_path) = ty else {
        return (false, None);
    };
    let Some(segment) = type_path.path.segments.last() else {
        return (false, None);
    };
    if segment.ident != "Option" {
        return (false, None);
    }
    let PathArguments::AngleBracketed(args) = &segment.arguments else {
        return (false, None);
    };
    if let Some(GenericArgument::Type(inner)) = args.args.first() {
        return (true, Some(inner.clone()));
    }
    (false, None)
}

#[cfg(test)]
mod tests {
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
}
