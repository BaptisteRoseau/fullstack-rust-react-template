use convert_case::{Case, Casing};

pub fn derive_table_name(ident: &str) -> String {
    let snake = ident.to_case(Case::Snake);
    format!("{snake}s")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        assert_eq!(derive_table_name("User"), "users");
    }

    #[test]
    fn test_compound() {
        assert_eq!(derive_table_name("UserProfile"), "user_profiles");
    }

    #[test]
    fn test_single_word() {
        assert_eq!(derive_table_name("Company"), "companys");
    }
}
