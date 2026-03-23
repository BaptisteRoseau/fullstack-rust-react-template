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

        let id_field = all_fields
            .iter()
            .find(|f| f.ident == "id")
            .cloned();

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

fn extract_option_inner(ty: &Type) -> (bool, Option<Type>) {
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
