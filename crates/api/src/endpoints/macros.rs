//! Helpers shared across endpoint modules.

/// Declares the OpenAPI tag (Swagger category) of an endpoint module.
///
/// Generates, in the invoking module:
/// - a `TAG` constant, meant to be used as `tag = TAG` in every
///   `#[utoipa::path]` of the module so its operations are grouped together;
/// - a `tag()` function returning the described [`Tag`], collected by
///   [`crate::routes::openapi`] to give the category its description.
///
/// This keeps a category's name and description next to the endpoints it
/// documents instead of in a central list.
///
/// [`Tag`]: utoipa::openapi::Tag
macro_rules! declare_tag {
    ($name:literal, $description:literal) => {
        pub(crate) const TAG: &str = $name;

        pub(crate) fn tag() -> utoipa::openapi::Tag {
            utoipa::openapi::tag::TagBuilder::new()
                .name($name)
                .description(Some($description))
                .build()
        }
    };
}

pub(crate) use declare_tag;
