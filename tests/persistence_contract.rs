use project_init::domain::{FindingKind, NewFinding, NewProject, RetrievalMode};
use project_init::storage::SqliteStore;
use tempfile::tempdir;

/// Creates every authoritative table required by the product contract.
#[test]
fn sqlite_migrations_create_all_authoritative_tables() {
    let store = SqliteStore::open_in_memory().expect("the in-memory database should open");
    let tables = store
        .table_names()
        .expect("migration table names should be queryable");

    for expected in [
        "agent_runs",
        "answers",
        "decisions",
        "documents",
        "entity_sequences",
        "evidence",
        "findings",
        "projects",
        "questions",
        "requirements",
        "retrieval_events",
        "semantic_index_state",
        "trace_links",
        "validation_findings",
        "validation_runs",
    ] {
        assert!(tables.contains(&expected.to_owned()), "missing {expected}");
    }
}

/// Reloads authoritative project state after every database handle has been dropped.
#[test]
fn project_round_trips_after_database_reopen() {
    let directory = tempdir().expect("a temporary test directory should be available");
    let database_path = directory.path().join("project-init.sqlite3");

    let project_id = {
        let mut store = SqliteStore::open(&database_path).expect("the database should open");
        let input = NewProject::new(
            "Calm Fishing VR",
            "A calm VR game with environmental fishing cues.",
            RetrievalMode::Relational,
        )
        .expect("the fixture project is valid");
        store
            .create_project(input)
            .expect("the project should persist")
            .id()
            .clone()
    };

    let store = SqliteStore::open(&database_path).expect("the database should reopen");
    let restored = store
        .get_project(&project_id)
        .expect("the project query should succeed")
        .expect("the project should still exist");

    assert_eq!(restored.name(), "Calm Fishing VR");
    assert_eq!(
        restored.brief(),
        "A calm VR game with environmental fishing cues."
    );
}

/// Rejects duplicate knowledge before consuming the next stable display identifier.
#[test]
fn duplicate_findings_do_not_consume_display_ids_or_mutate_state() {
    let mut store = SqliteStore::open_in_memory().expect("the database should open");
    let project = store
        .create_project(
            NewProject::new(
                "Calm Fishing VR",
                "A calm VR game with environmental fishing cues.",
                RetrievalMode::Relational,
            )
            .expect("the fixture project is valid"),
        )
        .expect("the fixture project should persist");

    let first = NewFinding::from_user_brief(
        project.id().clone(),
        FindingKind::ConfirmedFact,
        "The project is a VR game.",
    )
    .expect("the first fact is valid");
    let duplicate = first.clone();

    let stored_first = store
        .add_finding(first)
        .expect("the first fact should persist");
    assert_eq!(stored_first.display_id(), "FACT-001");
    assert!(store.add_finding(duplicate).is_err());

    let second = NewFinding::from_user_brief(
        project.id().clone(),
        FindingKind::ConfirmedFact,
        "The desired atmosphere is calm.",
    )
    .expect("the second fact is valid");
    let stored_second = store
        .add_finding(second)
        .expect("the second distinct fact should persist");

    assert_eq!(stored_second.display_id(), "FACT-002");
    assert_eq!(
        store
            .list_findings(project.id())
            .expect("the findings should be queryable")
            .len(),
        2
    );
}

/// Keeps identically worded findings isolated between independent projects.
#[test]
fn independent_projects_do_not_block_or_leak_findings() {
    let mut store = SqliteStore::open_in_memory().expect("the database should open");
    let first_project = store
        .create_project(
            NewProject::new("First", "First project brief.", RetrievalMode::Relational)
                .expect("the first project is valid"),
        )
        .expect("the first project should persist");
    let second_project = store
        .create_project(
            NewProject::new("Second", "Second project brief.", RetrievalMode::Relational)
                .expect("the second project is valid"),
        )
        .expect("the second project should persist");

    for project_id in [first_project.id(), second_project.id()] {
        let finding = NewFinding::from_user_brief(
            project_id.clone(),
            FindingKind::ConfirmedFact,
            "The project is local-first.",
        )
        .expect("the fact is valid");
        let stored = store
            .add_finding(finding)
            .expect("the fact should persist independently");
        assert_eq!(stored.display_id(), "FACT-001");
    }

    assert_eq!(
        store
            .list_findings(first_project.id())
            .expect("the first project's findings should load")
            .len(),
        1
    );
    assert_eq!(
        store
            .list_findings(second_project.id())
            .expect("the second project's findings should load")
            .len(),
        1
    );
}
