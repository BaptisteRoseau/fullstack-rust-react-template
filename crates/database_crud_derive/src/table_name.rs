use convert_case::{Case, Casing};

pub fn derive_table_name(ident: &str) -> String {
    let snake = ident.to_case(Case::Snake);
    format!("{snake}s")
}

test_utils::tests_file!("_tests/test_table_name.rs");
