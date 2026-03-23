use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::parse::ModelInfo;
use crate::type_mapping::to_crud_value;

pub fn generate(model: &ModelInfo) -> TokenStream {
    let struct_ident = &model.struct_ident;
    let patch_ident = format_ident!("{}Patch", struct_ident);
    let table = &model.table_name;

    let id_field = model.id_field.as_ref().expect("Crud derive requires an 'id' field");
    let id_ty = &id_field.ty;

    // Generate patch struct fields
    let patch_fields: Vec<TokenStream> = model
        .user_fields
        .iter()
        .map(|f| {
            let ident = &f.ident;
            if f.is_option {
                // Nullable field: Option<Option<T>>
                let inner_ty = f.inner_ty.as_ref().unwrap();
                quote! { pub #ident: Option<Option<#inner_ty>> }
            } else {
                // Non-nullable field: Option<T>
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

    // Generate the dynamic execute body:
    // For each user field, emit an if-let block that pushes a SET clause and CrudValue
    let execute_arms: Vec<TokenStream> = model
        .user_fields
        .iter()
        .map(|f| {
            let ident = &f.ident;
            let field_name = ident.to_string();

            if f.is_option {
                // For Option<T> model fields, patch field is Option<Option<T>>
                // When Some(inner), we push the inner value (which may be None for NULL)
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
                // For non-Option model fields, patch field is Option<T>
                // When Some(v), we push v directly
                let value_expr = quote! { v };
                // We need a non-option CrudValue here, so create a temp FieldInfo-like mapping
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

    // Generate field initializers for build_patch and patch (all None)
    let none_inits: Vec<TokenStream> = model
        .user_fields
        .iter()
        .map(|f| {
            let ident = &f.ident;
            quote! { #ident: None }
        })
        .collect();

    let select_sql = format!("SELECT * FROM {table} WHERE id = $1");

    quote! {
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

        // Methods on the model struct for creating patches
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
    }
}

/// For non-option fields unwrapped from Some(v), produce the right CrudValue
fn to_crud_value_non_option(field: &crate::parse::FieldInfo, value_expr: &TokenStream) -> TokenStream {
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
    if let syn::Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident.to_string();
        }
    }
    String::new()
}
