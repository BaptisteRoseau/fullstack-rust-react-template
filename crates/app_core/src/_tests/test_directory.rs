use super::*;

use database::testing::MockDatabase;

#[test]
fn a_name_is_trimmed() {
    let name = validate_name("  holiday photos \n".to_string())
        .expect("a padded name must be accepted");

    assert_eq!(
        name, "holiday photos",
        "the surrounding whitespace must be trimmed, got {name:?}"
    );
}

#[test]
fn an_empty_name_is_refused() {
    for candidate in ["", "   ", "\t\n"] {
        let refused = validate_name(candidate.to_string());
        assert!(
            matches!(refused, Err(CoreError::InvalidRequest(_))),
            "{candidate:?} must be refused as empty, got {refused:?}"
        );
    }
}

#[test]
fn a_name_longer_than_the_column_is_refused() {
    let refused = validate_name("a".repeat(MAX_NAME_LENGTH + 1));

    assert!(
        matches!(refused, Err(CoreError::InvalidRequest(_))),
        "a name of {} characters must be refused, got {refused:?}",
        MAX_NAME_LENGTH + 1
    );
}

#[test]
fn a_name_of_exactly_the_column_length_is_accepted() {
    let name = "a".repeat(MAX_NAME_LENGTH);

    let accepted = validate_name(name.clone());

    assert_eq!(
        accepted.ok().as_deref(),
        Some(name.as_str()),
        "a name of exactly {MAX_NAME_LENGTH} characters must be accepted"
    );
}

#[test]
fn a_name_carrying_a_path_separator_is_refused() {
    for candidate in ["../etc/passwd", "a/b", "a\\b"] {
        let refused = validate_name(candidate.to_string());
        assert!(
            matches!(refused, Err(CoreError::InvalidRequest(_))),
            "{candidate:?} must be refused as a path, got {refused:?}"
        );
    }
}

#[tokio::test]
async fn a_directory_cannot_be_moved_inside_its_own_subtree() {
    let mut db = MockDatabase::default();
    let owner = Uuid::new_v4();
    let root = create(&mut db, owner, None).await;
    let child = create(&mut db, owner, Some(root)).await;
    let grandchild = create(&mut db, owner, Some(child)).await;

    let refused =
        update_directory(&mut db, owner, root, None, Some(Some(grandchild))).await;

    assert!(
        matches!(refused, Err(CoreError::InvalidRequest(_))),
        "moving a directory below itself must be refused, got {refused:?}"
    );
}

#[tokio::test]
async fn a_directory_cannot_be_moved_into_itself() {
    let mut db = MockDatabase::default();
    let owner = Uuid::new_v4();
    let directory = create(&mut db, owner, None).await;

    let refused =
        update_directory(&mut db, owner, directory, None, Some(Some(directory))).await;

    assert!(
        matches!(refused, Err(CoreError::InvalidRequest(_))),
        "moving a directory into itself must be refused, got {refused:?}"
    );
}

#[tokio::test]
async fn a_directory_moves_to_a_sibling_branch() {
    let mut db = MockDatabase::default();
    let owner = Uuid::new_v4();
    let source = create(&mut db, owner, None).await;
    let destination = create(&mut db, owner, None).await;
    let moved = create(&mut db, owner, Some(source)).await;

    let updated =
        update_directory(&mut db, owner, moved, None, Some(Some(destination)))
            .await
            .expect("update_directory failed");

    assert_eq!(
        updated.parent_id,
        Some(destination),
        "the directory must sit under the destination, got {:?}",
        updated.parent_id
    );
}

#[tokio::test]
async fn listing_a_directory_a_stranger_cannot_see_answers_not_found() {
    let mut db = MockDatabase::default();
    let owner = Uuid::new_v4();
    let stranger = Uuid::new_v4();
    let directory = create(&mut db, owner, None).await;

    let refused = list_entries(&db, stranger, Some(directory)).await;

    assert!(
        matches!(refused, Err(CoreError::NotFound(_))),
        "a stranger must be answered NotFound, got {refused:?}"
    );
}

#[tokio::test]
async fn listing_the_root_shows_only_the_callers_own_entries() {
    let mut db = MockDatabase::default();
    let owner = Uuid::new_v4();
    let stranger = Uuid::new_v4();
    create(&mut db, owner, None).await;
    create(&mut db, stranger, None).await;

    let listing = list_entries(&db, owner, None)
        .await
        .expect("list_entries failed");

    assert_eq!(
        listing.directories.len(),
        1,
        "the root must hold only the caller's directory, got {} entries",
        listing.directories.len()
    );
    assert!(
        listing.directory.is_none(),
        "the root is not a row and must not be reported as one"
    );
}

#[tokio::test]
async fn an_editor_creates_a_child_but_does_not_delete_the_directory() {
    let mut db = MockDatabase::default();
    let owner = Uuid::new_v4();
    let editor = Uuid::new_v4();
    let directory = create(&mut db, owner, None).await;
    grant(&mut db, directory, editor, "editor", owner).await;

    let created =
        create_directory(&mut db, editor, "notes".to_string(), Some(directory)).await;
    let refused = delete_directory(&mut db, editor, directory).await;

    assert!(
        created.is_ok(),
        "an editor must be able to create a child, got {created:?}"
    );
    assert!(
        matches!(refused, Err(CoreError::NotFound(_))),
        "an editor must not be able to delete the directory, got {refused:?}"
    );
}

async fn create(db: &mut impl Database, owner: Uuid, parent_id: Option<Uuid>) -> Uuid {
    db.create_directory(owner, parent_id, format!("dir-{}", Uuid::new_v4()))
        .await
        .expect("create_directory failed")
        .id
}

async fn grant(
    db: &mut impl Database,
    directory_id: Uuid,
    grantee: Uuid,
    level: &str,
    granted_by: Uuid,
) {
    db.upsert_directory_permission(directory_id, grantee, level, granted_by)
        .await
        .expect("upsert_directory_permission failed");
}
