use super::*;

#[test]
fn test_not_found_becomes_a_tool_level_error() {
    let uuid = "0198f0b4-0000-7000-8000-000000000000";

    let response = into_tool_result(Err(McpError::NotFound(uuid.to_string())))
        .expect("a tool-level failure must never become a protocol error");

    assert_eq!(
        response.is_error,
        Some(true),
        "a McpError must be reported as a tool-level error, got is_error={:?}",
        response.is_error
    );
    let rendered = format!("{:?}", response.content);
    assert!(
        rendered.contains(uuid),
        "the caller should be told what was not found, got {rendered}"
    );
}

#[test]
fn test_internal_failures_do_not_leak_their_detail() {
    let error = McpError::SerializationError(
        serde_json::from_str::<u8>("not json").expect_err("invalid JSON"),
    );

    let response =
        into_tool_result(Err(error)).expect("a tool-level failure stays a tool result");

    let rendered = format!("{:?}", response.content);
    assert!(
        !rendered.contains("expected"),
        "the serde message must stay server-side, got {rendered}"
    );
}

#[test]
fn test_structured_fills_both_representations() {
    let response = structured(serde_json::json!({ "answer": 42 }))
        .expect("serializing a plain JSON value");

    assert!(
        response.structured_content.is_some(),
        "structured content is what a client reads as data, got {:?}",
        response.structured_content
    );
    assert!(
        !response.content.is_empty(),
        "the text rendering must be filled too, got {:?}",
        response.content
    );
}
