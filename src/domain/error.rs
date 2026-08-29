use thiserror::Error;

/// Describes invalid domain input or a lifecycle transition that must not mutate state.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum DomainError {
    /// Indicates that a required text value contains no meaningful characters.
    #[error("{field} must not be blank")]
    BlankText { field: &'static str },
    /// Indicates that a bounded text value exceeds the supported character count.
    #[error("{field} exceeds the maximum length of {max} characters")]
    TextTooLong { field: &'static str, max: usize },
    /// Indicates that the requested project lifecycle edge is not legal.
    #[error("project cannot transition from {from} to {to}")]
    InvalidProjectTransition { from: String, to: String },
    /// Rejects a clarification threshold outside the complete question-priority range.
    #[error("clarification threshold must be between {min} and {max}, received {value}")]
    InvalidClarificationThreshold { value: u16, min: u16, max: u16 },
}

/// Trims and validates text that becomes authoritative project state.
pub(crate) fn normalize_required_text(
    value: &str,
    field: &'static str,
    max: usize,
) -> Result<String, DomainError> {
    let normalized = value.trim();
    if normalized.is_empty() {
        return Err(DomainError::BlankText { field });
    }
    if normalized.chars().count() > max {
        return Err(DomainError::TextTooLong { field, max });
    }
    Ok(normalized.to_owned())
}
