use super::*;

use database::models::NewFile;
use database::testing::MockDatabase;

#[tokio::test]
async fn the_owner_of_a_directory_manages_it() {
    let mut db = MockDatabase::default();
    let owner = Uuid::new_v4();
    let directory = create_directory(&mut db, owner, None).await;

    let level = effective_level(&db, owner, ResourceRef::Directory(directory))
        .await
        .expect("effective_level failed");

    assert_eq!(
        level,
        Some(PermissionLevel::Manager),
        "the owner must manage its own directory, got {level:?}"
    );
}

#[tokio::test]
async fn the_owner_of_a_file_manages_it() {
    let mut db = MockDatabase::default();
    let owner = Uuid::new_v4();
    let file = create_file(&mut db, owner, None).await;

    let level = effective_level(&db, owner, ResourceRef::File(file))
        .await
        .expect("effective_level failed");

    assert_eq!(
        level,
        Some(PermissionLevel::Manager),
        "the owner must manage its own file, got {level:?}"
    );
}

#[tokio::test]
async fn a_stranger_reaches_nothing() {
    let mut db = MockDatabase::default();
    let owner = Uuid::new_v4();
    let stranger = Uuid::new_v4();
    let directory = create_directory(&mut db, owner, None).await;
    let file = create_file(&mut db, owner, Some(directory)).await;

    let on_directory = effective_level(&db, stranger, ResourceRef::Directory(directory))
        .await
        .expect("effective_level failed");
    let on_file = effective_level(&db, stranger, ResourceRef::File(file))
        .await
        .expect("effective_level failed");

    assert_eq!(
        on_directory, None,
        "a stranger must hold no level on the directory, got {on_directory:?}"
    );
    assert_eq!(
        on_file, None,
        "a stranger must hold no level on the file, got {on_file:?}"
    );
}

#[tokio::test]
async fn a_direct_grant_on_a_file_is_honoured() {
    let mut db = MockDatabase::default();
    let owner = Uuid::new_v4();
    let guest = Uuid::new_v4();
    let file = create_file(&mut db, owner, None).await;
    grant_on_file(&mut db, file, guest, "editor", owner).await;

    let level = effective_level(&db, guest, ResourceRef::File(file))
        .await
        .expect("effective_level failed");

    assert_eq!(
        level,
        Some(PermissionLevel::Editor),
        "the direct grant must apply, got {level:?}"
    );
}

#[tokio::test]
async fn a_grant_on_an_ancestor_directory_reaches_a_nested_file() {
    let mut db = MockDatabase::default();
    let owner = Uuid::new_v4();
    let guest = Uuid::new_v4();
    let root = create_directory(&mut db, owner, None).await;
    let middle = create_directory(&mut db, owner, Some(root)).await;
    let leaf = create_directory(&mut db, owner, Some(middle)).await;
    let file = create_file(&mut db, owner, Some(leaf)).await;
    grant_on_directory(&mut db, root, guest, "viewer", owner).await;

    let on_leaf = effective_level(&db, guest, ResourceRef::Directory(leaf))
        .await
        .expect("effective_level failed");
    let on_file = effective_level(&db, guest, ResourceRef::File(file))
        .await
        .expect("effective_level failed");

    assert_eq!(
        on_leaf,
        Some(PermissionLevel::Viewer),
        "the grant on the root must reach the leaf directory, got {on_leaf:?}"
    );
    assert_eq!(
        on_file,
        Some(PermissionLevel::Viewer),
        "the grant on the root must reach the nested file, got {on_file:?}"
    );
}

#[tokio::test]
async fn the_highest_applicable_level_wins() {
    let mut db = MockDatabase::default();
    let owner = Uuid::new_v4();
    let guest = Uuid::new_v4();
    let directory = create_directory(&mut db, owner, None).await;
    let file = create_file(&mut db, owner, Some(directory)).await;
    grant_on_directory(&mut db, directory, guest, "viewer", owner).await;
    grant_on_file(&mut db, file, guest, "manager", owner).await;

    let level = effective_level(&db, guest, ResourceRef::File(file))
        .await
        .expect("effective_level failed");

    assert_eq!(
        level,
        Some(PermissionLevel::Manager),
        "the file grant outranks the inherited one and must win, got {level:?}"
    );
}

#[tokio::test]
async fn a_deeper_grant_does_not_lower_an_inherited_one() {
    let mut db = MockDatabase::default();
    let owner = Uuid::new_v4();
    let guest = Uuid::new_v4();
    let root = create_directory(&mut db, owner, None).await;
    let child = create_directory(&mut db, owner, Some(root)).await;
    grant_on_directory(&mut db, root, guest, "manager", owner).await;
    grant_on_directory(&mut db, child, guest, "viewer", owner).await;

    let level = effective_level(&db, guest, ResourceRef::Directory(child))
        .await
        .expect("effective_level failed");

    assert_eq!(
        level,
        Some(PermissionLevel::Manager),
        "the highest of the applicable levels must win, got {level:?}"
    );
}

#[tokio::test]
async fn owning_an_ancestor_directory_reaches_someone_elses_file() {
    let mut db = MockDatabase::default();
    let owner = Uuid::new_v4();
    let contributor = Uuid::new_v4();
    let directory = create_directory(&mut db, owner, None).await;
    let file = create_file(&mut db, contributor, Some(directory)).await;

    let level = effective_level(&db, owner, ResourceRef::File(file))
        .await
        .expect("effective_level failed");

    assert_eq!(
        level,
        Some(PermissionLevel::Manager),
        "the owner of the containing directory must manage what it holds, got {level:?}"
    );
}

#[tokio::test]
async fn a_missing_resource_yields_no_level_rather_than_an_error() {
    let db = MockDatabase::default();
    let unknown = Uuid::new_v4();

    let on_directory = effective_level(&db, Uuid::new_v4(), ResourceRef::Directory(unknown))
        .await
        .expect("effective_level failed");
    let on_file = effective_level(&db, Uuid::new_v4(), ResourceRef::File(unknown))
        .await
        .expect("effective_level failed");

    assert_eq!(
        on_directory, None,
        "an unknown directory must yield no level, got {on_directory:?}"
    );
    assert_eq!(
        on_file, None,
        "an unknown file must yield no level, got {on_file:?}"
    );
}

#[tokio::test]
async fn has_access_compares_against_the_minimum() {
    let mut db = MockDatabase::default();
    let owner = Uuid::new_v4();
    let guest = Uuid::new_v4();
    let directory = create_directory(&mut db, owner, None).await;
    grant_on_directory(&mut db, directory, guest, "editor", owner).await;

    let resource = ResourceRef::Directory(directory);
    let as_viewer = has_access(&db, guest, resource, PermissionLevel::Viewer)
        .await
        .expect("has_access failed");
    let as_editor = has_access(&db, guest, resource, PermissionLevel::Editor)
        .await
        .expect("has_access failed");
    let as_manager = has_access(&db, guest, resource, PermissionLevel::Manager)
        .await
        .expect("has_access failed");

    assert!(as_viewer, "an editor must satisfy a viewer minimum, got false");
    assert!(as_editor, "an editor must satisfy an editor minimum, got false");
    assert!(
        !as_manager,
        "an editor must not satisfy a manager minimum, got true"
    );
}

#[tokio::test]
async fn require_answers_not_found_when_the_level_is_too_low() {
    let mut db = MockDatabase::default();
    let owner = Uuid::new_v4();
    let guest = Uuid::new_v4();
    let directory = create_directory(&mut db, owner, None).await;
    grant_on_directory(&mut db, directory, guest, "viewer", owner).await;

    let refused = require(
        &db,
        guest,
        ResourceRef::Directory(directory),
        PermissionLevel::Manager,
    )
    .await;

    match refused {
        Err(CoreError::NotFound(id)) => assert_eq!(
            id,
            directory.to_string(),
            "the error must name the resource, got {id}"
        ),
        other => panic!("expected NotFound so existence does not leak, got {other:?}"),
    }
}

async fn create_directory(
    db: &mut impl Database,
    owner: Uuid,
    parent_id: Option<Uuid>,
) -> Uuid {
    db.create_directory(owner, parent_id, format!("dir-{}", Uuid::new_v4()))
        .await
        .expect("create_directory failed")
        .id
}

async fn create_file(
    db: &mut impl Database,
    owner: Uuid,
    parent_id: Option<Uuid>,
) -> Uuid {
    let id = Uuid::now_v7();
    db.create_file(NewFile {
        id,
        owner,
        parent_id,
        name: format!("file-{id}"),
        storage_key: format!("files/{id}/content"),
        mime_type: "text/plain".to_string(),
        size_bytes: 0,
        stored_size_bytes: 0,
        is_compressed: false,
        encrypted_dek: Vec::new(),
        dek_nonce: Vec::new(),
        content_nonce: Vec::new(),
        thumbnail_storage_key: None,
        thumbnail_nonce: None,
    })
    .await
    .expect("create_file failed")
    .id
}

async fn grant_on_directory(
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

async fn grant_on_file(
    db: &mut impl Database,
    file_id: Uuid,
    grantee: Uuid,
    level: &str,
    granted_by: Uuid,
) {
    db.upsert_file_permission(file_id, grantee, level, granted_by)
        .await
        .expect("upsert_file_permission failed");
}
