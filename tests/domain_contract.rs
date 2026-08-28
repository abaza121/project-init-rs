use project_init::domain::{
    CostOfBeingWrong, Impact, Project, ProjectStatus, QuestionPriority, RetrievalMode, Uncertainty,
};

/// Rejects briefs that cannot produce any authoritative project knowledge.
#[test]
fn project_rejects_blank_briefs_without_constructing_state() {
    let result = Project::new("Calm Fishing VR", "  \n\t ", RetrievalMode::Relational);

    assert!(result.is_err());
}

/// Preserves a normalized brief and begins every project in the draft state.
#[test]
fn project_owns_a_trimmed_brief_and_starts_as_draft() {
    let project = Project::new(
        "Calm Fishing VR",
        "  A calm fishing experience.  ",
        RetrievalMode::Relational,
    )
    .expect("a meaningful brief should create a project");

    assert_eq!(project.name(), "Calm Fishing VR");
    assert_eq!(project.brief(), "A calm fishing experience.");
    assert_eq!(project.status(), ProjectStatus::Draft);
}

/// Allows the documented lifecycle path while rejecting consequential stage skips.
#[test]
fn lifecycle_rejects_skipping_required_analysis_and_clarification() {
    let mut project = Project::new(
        "Calm Fishing VR",
        "A calm fishing experience.",
        RetrievalMode::Relational,
    )
    .expect("the fixture brief is valid");

    assert!(project.transition_to(ProjectStatus::Planning).is_err());
    assert_eq!(project.status(), ProjectStatus::Draft);

    project
        .transition_to(ProjectStatus::Analyzing)
        .expect("draft projects may begin analysis");
    project
        .transition_to(ProjectStatus::AwaitingClarification)
        .expect("analyzed projects may await user clarification");

    assert_eq!(project.status(), ProjectStatus::AwaitingClarification);
}

/// Multiplies all three prioritization dimensions so interactions remain explicit.
#[test]
fn question_priority_combines_impact_uncertainty_and_cost_of_being_wrong() {
    let high = QuestionPriority::new(Impact::High, Uncertainty::High, CostOfBeingWrong::High);
    let lower = QuestionPriority::new(Impact::High, Uncertainty::Medium, CostOfBeingWrong::High);

    assert_eq!(high.score(), 125);
    assert_eq!(lower.score(), 75);
    assert!(high > lower);
}

/// Includes an exact threshold boundary rather than delaying an equally important question.
#[test]
fn attention_threshold_includes_the_exact_priority_boundary() {
    let priority = QuestionPriority::new(Impact::High, Uncertainty::Medium, CostOfBeingWrong::High);

    assert!(priority.requires_attention(75));
    assert!(!priority.requires_attention(76));
}
