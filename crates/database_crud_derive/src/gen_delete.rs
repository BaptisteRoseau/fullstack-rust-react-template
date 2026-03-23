use proc_macro2::TokenStream;
use quote::quote;

use crate::parse::ModelInfo;

pub fn generate(model: &ModelInfo) -> TokenStream {
    let table = &model.table_name;
    let id_field = model.id_field.as_ref().expect("Crud derive requires an 'id' field");
    let id_ty = &id_field.ty;

    let sql = format!("DELETE FROM {table} WHERE id = $1");

    quote! {
        pub async fn delete(
            db: &impl crate::crud::CrudExecutor,
            id: #id_ty,
        ) -> Result<u64, crate::crud::CrudError> {
            db.crud_execute(#sql, vec![crate::crud::CrudValue::Uuid(id)]).await
        }
    }
}
