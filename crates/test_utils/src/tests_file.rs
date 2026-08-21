/// Declares the unit-test module of the current file, whose body lives in a
/// sibling `_tests/test_<name>.rs` file.
///
/// Expands to the `#[cfg(test)]` + `#[path]` + `mod tests;` triple, so the test
/// module stays a child of the current module and keeps access to its private
/// items. The path is resolved exactly like a hand-written `#[path]`: relative
/// to the directory holding the file that invokes the macro.
///
/// ```ignore
/// test_utils::tests_file!("_tests/test_scope.rs");
/// ```
///
/// Attributes that would have sat on the `mod tests;` declaration — an
/// `#[allow(...)]` scoping a lint to the whole test module, typically — go
/// before the path:
///
/// ```ignore
/// test_utils::tests_file!(
///     #[allow(clippy::field_reassign_with_default)]
///     "_tests/test_config.rs"
/// );
/// ```
#[macro_export]
macro_rules! tests_file {
    ($(#[$attribute:meta])* $path:literal) => {
        #[cfg(test)]
        #[path = $path]
        $(#[$attribute])*
        mod tests;
    };
}
