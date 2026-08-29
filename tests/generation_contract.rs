use project_init::documents::PackageRenderer;
use project_init::storage::SqliteStore;
use project_init::workflow::{ProjectService, validate_snapshot};
use tempfile::tempdir;

/// Generates a traceable package from authoritative state after clarification is complete.
#[test]
fn generated_package_comes_from_the_reconciled_project_snapshot() {
    let store = SqliteStore::open_in_memory().expect("the database should open");
    let mut service = ProjectService::new(store);
    let project = service
        .initialize_project(
            "Calm Fishing VR",
            "I want a calm VR fishing game with environmental fishing cues.",
        )
        .expect("the project should initialize");
    for question in service
        .inspect_project(project.id())
        .expect("questions should load")
        .questions
    {
        let answer = if question.prompt.to_ascii_lowercase().contains("platform") {
            "Meta Quest 3"
        } else {
            "Unity"
        };
        service
            .answer_question(&question.id, answer, None)
            .expect("the answer should reconcile");
    }
    let snapshot = service
        .inspect_project(project.id())
        .expect("the reconciled snapshot should load");
    let output = tempdir().expect("an output directory should be available");

    let artifacts = PackageRenderer::render(&snapshot, output.path())
        .expect("the package should render from the snapshot");
    let report = validate_snapshot(&snapshot, output.path());

    assert!(report.passed, "{:?}", report.findings);
    for expected in [
        "README.md",
        "Requirements.md",
        "Assumptions.md",
        "OpenQuestions.md",
        "Traceability.md",
        "ValidationReport.md",
    ] {
        assert!(output.path().join(expected).is_file(), "missing {expected}");
    }
    assert_eq!(artifacts.len(), 17);
    let requirements = std::fs::read_to_string(output.path().join("Requirements.md"))
        .expect("requirements should be readable");
    assert!(requirements.contains("REQ-001"));
    assert!(requirements.contains("Meta Quest 3"));
}

/// Keeps unresolved high-impact uncertainty visible as a blocking validation result.
#[test]
fn validation_blocks_completion_when_high_impact_questions_remain_open() {
    let store = SqliteStore::open_in_memory().expect("the database should open");
    let mut service = ProjectService::new(store);
    let project = service
        .initialize_project("Calm Fishing VR", "I want a calm VR fishing game.")
        .expect("the project should initialize");
    let snapshot = service
        .inspect_project(project.id())
        .expect("the snapshot should load");
    let output = tempdir().expect("an output directory should be available");
    PackageRenderer::render(&snapshot, output.path()).expect("the draft package should render");

    let report = validate_snapshot(&snapshot, output.path());

    assert!(!report.passed);
    assert!(report.findings.iter().any(|finding| {
        finding.code == "UNRESOLVED_HIGH_QUESTION" && finding.severity == "high"
    }));
}

/// Renders the full fixed project-initiation package including strategic and research artifacts.
#[test]
fn complete_package_contains_every_required_artifact() {
    let store = SqliteStore::open_in_memory().expect("the database should open");
    let mut service = ProjectService::new(store);
    let project = service
        .initialize_from_analysis_json(
            "Complete package",
            "Build a local planning tool for independent game developers.",
            r#"{"findings":[{"kind":"confirmed_fact","statement":"The product is local.","impact":"medium","source_type":"user_brief"}]}"#,
        )
        .expect("the project should initialize");
    let snapshot = service
        .inspect_project(project.id())
        .expect("the project should remain inspectable");
    let directory = tempfile::tempdir().expect("an output directory should exist");

    let paths = PackageRenderer::render(&snapshot, directory.path())
        .expect("the complete package should render");
    let names = paths
        .iter()
        .filter_map(|path| path.file_name().and_then(|name| name.to_str()))
        .collect::<Vec<_>>();

    for required in [
        "README.md",
        "Requirements.md",
        "Assumptions.md",
        "OpenQuestions.md",
        "SWOT.md",
        "MissionVision.md",
        "VisualIdentity.md",
        "BrandPrompt.md",
        "TechnicalArchitecture.md",
        "Research-01-Audience.md",
        "Research-02-Experience.md",
        "Research-03-Market.md",
        "Research-04-Technology.md",
        "Research-05-Delivery.md",
        "Traceability.md",
        "ValidationReport.md",
    ] {
        assert!(names.contains(&required), "missing {required}");
    }
    assert!(
        names
            .iter()
            .any(|name| name.starts_with("log-") && name.ends_with(".md")),
        "the package should contain a timestamped session log"
    );
}
