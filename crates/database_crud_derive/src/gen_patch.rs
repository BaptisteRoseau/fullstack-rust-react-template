use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::parse::ModelInfo;
use crate::type_mapping::to_crud_value;

/// Returns (patch_struct_and_impl, patch_methods_on_model)
pub fn generate(model: &ModelInfo) -> (TokenStream, TokenStream) {
    let struct_ident = &model.struct_ident;
    let patch_ident = format_ident!("{}Patch", struct_ident);
    let table = &model.table_name;

    let id_field = model
        .id_field
        .as_ref()
        .expect("Crud derive requires an 'id' field");
    let id_ty = &id_field.ty;

    // Generate patch struct fields
    let patch_fields: Vec<TokenStream> = model
        .user_fields
        .iter()
        .map(|f| {
            let ident = &f.ident;
            if f.is_option {
                let inner_ty = f.inner_ty.as_ref().unwrap();
                quote! { pub #ident: Option<Option<#inner_ty>> }
            } else {
                let ty = &f.ty;
                quote! { pub #ident: Option<#ty> }
            }
        })
        .collect();

    // Generate setter methods
    let setters: Vec<TokenStream> = model
        .user_fields
        .iter()
        .map(|f| {
            let ident = &f.ident;
            let method = format_ident!("set_{}", ident);

            if f.is_option {
                let inner_ty = f.inner_ty.as_ref().unwrap();
                let inner_name = extract_base_type_name(inner_ty);
                if inner_name == "String" {
                    quote! {
                        pub fn #method(mut self, v: Option<impl Into<String>>) -> Self {
                            self.#ident = Some(v.map(|val| val.into()));
                            self
                        }
                    }
                } else {
                    quote! {
                        pub fn #method(mut self, v: Option<#inner_ty>) -> Self {
                            self.#ident = Some(v);
                            self
                        }
                    }
                }
            } else {
                let ty = &f.ty;
                let type_name = extract_base_type_name(ty);
                if type_name == "String" {
                    quote! {
                        pub fn #method(mut self, v: impl Into<String>) -> Self {
                            self.#ident = Some(v.into());
                            self
                        }
                    }
                } else {
                    quote! {
                        pub fn #method(mut self, v: #ty) -> Self {
                            self.#ident = Some(v);
                            self
                        }
                    }
                }
            }
        })
        .collect();

    // Generate execute body arms
    let execute_arms: Vec<TokenStream> = model
        .user_fields
        .iter()
        .map(|f| {
            let ident = &f.ident;
            let field_name = ident.to_string();

            if f.is_option {
                let value_expr = quote! { inner };
                let crud_value = to_crud_value(f, &value_expr);
                quote! {
                    if let Some(inner) = self.#ident {
                        set_clauses.push(format!("{} = ${}", #field_name, idx));
                        args.push(#crud_value);
                        idx += 1;
                    }
                }
            } else {
                let value_expr = quote! { v };
                let crud_value = to_crud_value_non_option(f, &value_expr);
                quote! {
                    if let Some(v) = self.#ident {
                        set_clauses.push(format!("{} = ${}", #field_name, idx));
                        args.push(#crud_value);
                        idx += 1;
                    }
                }
            }
        })
        .collect();

    // Field initializers (all None)
    let none_inits: Vec<TokenStream> = model
        .user_fields
        .iter()
        .map(|f| {
            let ident = &f.ident;
            quote! { #ident: None }
        })
        .collect();

    let select_sql = format!("SELECT * FROM {table} WHERE id = $1");

    // The patch struct and its impl (goes outside the model's impl block)
    let patch_struct_and_impl = quote! {
        pub struct #patch_ident {
            pub id: #id_ty,
            #(#patch_fields),*
        }

        impl #patch_ident {
            #(#setters)*

            pub async fn execute(self, db: &impl crate::crud::CrudExecutor) -> Result<#struct_ident, crate::crud::CrudError> {
                let mut set_clauses: Vec<String> = Vec::new();
                let mut args: Vec<crate::crud::CrudValue> = Vec::new();
                let mut idx: u32 = 1;

                #(#execute_arms)*

                if set_clauses.is_empty() {
                    return db.crud_fetch_one::<#struct_ident>(
                        #select_sql,
                        vec![crate::crud::CrudValue::Uuid(self.id)],
                    ).await;
                }

                let query = format!(
                    "UPDATE {} SET {} WHERE id = ${} RETURNING *",
                    #table,
                    set_clauses.join(", "),
                    idx
                );
                args.push(crate::crud::CrudValue::Uuid(self.id));

                db.crud_fetch_one::<#struct_ident>(&query, args).await
            }
        }
    };

    // Methods that go on the model struct's impl block
    let patch_methods_on_model = quote! {
        pub fn build_patch(id: #id_ty) -> #patch_ident {
            #patch_ident {
                id,
                #(#none_inits),*
            }
        }

        pub fn patch(&self) -> #patch_ident {
            #patch_ident {
                id: self.id,
                #(#none_inits),*
            }
        }
    };

    (patch_struct_and_impl, patch_methods_on_model)
}

fn to_crud_value_non_option(
    field: &crate::parse::FieldInfo,
    value_expr: &TokenStream,
) -> TokenStream {
    let type_name = extract_base_type_name(&field.ty);
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

fn extract_base_type_name(ty: &syn::Type) -> String {
    if let syn::Type::Path(type_path) = ty
        && let Some(segment) = type_path.path.segments.last()
    {
        return segment.ident.to_string();
    }
    String::new()
}

#[cfg(test)]
mod tests {
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
}
