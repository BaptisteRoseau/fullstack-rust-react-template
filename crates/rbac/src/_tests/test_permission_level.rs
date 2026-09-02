use super::*;

#[test]
fn levels_are_ordered_from_viewer_to_manager() {
    assert!(
        PermissionLevel::Viewer < PermissionLevel::Editor,
        "viewer must rank below editor, got {:?} vs {:?}",
        PermissionLevel::Viewer,
        PermissionLevel::Editor
    );
    assert!(
        PermissionLevel::Editor < PermissionLevel::Manager,
        "editor must rank below manager, got {:?} vs {:?}",
        PermissionLevel::Editor,
        PermissionLevel::Manager
    );
}

#[test]
fn a_manager_satisfies_every_lower_requirement() {
    for required in [
        PermissionLevel::Viewer,
        PermissionLevel::Editor,
        PermissionLevel::Manager,
    ] {
        assert!(
            PermissionLevel::Manager >= required,
            "manager must satisfy {required:?}, got false"
        );
    }
}

#[test]
fn a_viewer_does_not_satisfy_editor() {
    assert!(
        PermissionLevel::Viewer < PermissionLevel::Editor,
        "a viewer must not satisfy an editor requirement, got {:?} >= {:?}",
        PermissionLevel::Viewer,
        PermissionLevel::Editor
    );
}

#[test]
fn parsing_round_trips_through_as_str() {
    for level in [
        PermissionLevel::Viewer,
        PermissionLevel::Editor,
        PermissionLevel::Manager,
    ] {
        let parsed = level.as_str().parse::<PermissionLevel>();
        assert_eq!(
            parsed,
            Ok(level),
            "{level:?} did not round-trip through {}, got {parsed:?}",
            level.as_str()
        );
    }
}

#[test]
fn parsing_an_unknown_level_fails() {
    let parsed = "owner".parse::<PermissionLevel>();
    assert_eq!(
        parsed,
        Err(UnknownPermissionLevel("owner".to_string())),
        "an unknown level must not parse, got {parsed:?}"
    );
}
