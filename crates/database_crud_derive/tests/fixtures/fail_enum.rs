use database_crud_derive::Crud;

#[derive(Crud)]
enum Status {
    Active,
    Inactive,
}

fn main() {}
