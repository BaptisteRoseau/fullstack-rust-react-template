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
