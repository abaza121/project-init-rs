use serde::{Deserialize, Serialize};

use super::Impact;

/// Represents how likely available project knowledge is incomplete or wrong.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Uncertainty {
    /// The current statement is strongly supported.
    Low,
    /// Important details remain ambiguous.
    Medium,
    /// The system lacks a reliable answer.
    High,
}

impl Uncertainty {
    /// Returns the ordinal used in the deterministic priority product.
    const fn weight(self) -> u16 {
        match self {
            Self::Low => 1,
            Self::Medium => 3,
            Self::High => 5,
        }
    }
}

/// Represents the consequence of proceeding with an incorrect answer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CostOfBeingWrong {
    /// Correction would be localized and inexpensive.
    Low,
    /// Correction would cause meaningful rework.
    Medium,
    /// Correction could invalidate product scope or architecture.
    High,
}

impl CostOfBeingWrong {
    /// Returns the ordinal used in the deterministic priority product.
    const fn weight(self) -> u16 {
        match self {
            Self::Low => 1,
            Self::Medium => 3,
            Self::High => 5,
        }
    }
}

/// Stores a comparable question score derived from impact, uncertainty, and rework cost.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct QuestionPriority(u16);

impl QuestionPriority {
    /// Calculates the bounded multiplicative priority for a clarification candidate.
    pub const fn new(
        impact: Impact,
        uncertainty: Uncertainty,
        cost_of_being_wrong: CostOfBeingWrong,
    ) -> Self {
        Self(impact.weight() * uncertainty.weight() * cost_of_being_wrong.weight())
    }

    /// Returns the numeric score used for ordering and persistence.
    pub const fn score(self) -> u16 {
        self.0
    }

    /// Determines whether the score meets an inclusive interruption threshold.
    pub const fn requires_attention(self, threshold: u16) -> bool {
        self.0 >= threshold
    }
}
