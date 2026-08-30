use std::time::Duration;

use async_trait::async_trait;
use clap::ValueEnum;
use tokio::sync::mpsc;

use super::{
    ActivityEvent, AgentClient, AgentError, AgentExecution, AnalysisRequest, AutoAnswerClient,
    CancellationToken, CodexCliClient, DocumentationClient, DocumentationRequest,
    JudgedResearchBatch, LocalHttpProvider, ResearchBatchPlan, ResearchClient,
    ResearchJudgmentRequest, ResearchPlanRequest, ResearchRequest, ResearchedAnswer,
};

/// Selects one explicitly supported provider without changing workflow capability contracts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum ProviderKind {
    /// Uses the installed Codex CLI and its configured tools.
    Codex,
    /// Uses a managed loopback OpenAI-compatible inference service.
    Local,
}

/// Dispatches each capability to one concrete configured provider.
#[derive(Debug, Clone)]
pub enum ConfiguredProvider {
    /// Wraps the existing isolated Codex subprocess adapter.
    Codex(CodexCliClient),
    /// Wraps the local loopback HTTP and managed runtime adapter.
    Local(LocalHttpProvider),
}

#[async_trait]
impl AgentClient for ConfiguredProvider {
    /// Delegates authoritative initial analysis without changing its provider-neutral request.
    async fn analyze(
        &self,
        request: AnalysisRequest,
        activity: mpsc::Sender<ActivityEvent>,
        cancellation: CancellationToken,
    ) -> Result<AgentExecution, AgentError> {
        match self {
            Self::Codex(client) => client.analyze(request, activity, cancellation).await,
            Self::Local(client) => client.analyze(request, activity, cancellation).await,
        }
    }

    /// Returns the selected provider's absolute analysis limit.
    fn timeout(&self) -> Duration {
        match self {
            Self::Codex(client) => client.timeout(),
            Self::Local(client) => client.timeout(),
        }
    }
}

#[async_trait]
impl ResearchClient for ConfiguredProvider {
    /// Delegates one cited research request to the selected provider.
    async fn research(
        &self,
        request: ResearchRequest,
        activity: mpsc::Sender<ActivityEvent>,
        cancellation: CancellationToken,
    ) -> Result<ResearchedAnswer, AgentError> {
        match self {
            Self::Codex(client) => {
                ResearchClient::research(client, request, activity, cancellation).await
            }
            Self::Local(client) => {
                ResearchClient::research(client, request, activity, cancellation).await
            }
        }
    }
}

#[async_trait]
impl AutoAnswerClient for ConfiguredProvider {
    /// Delegates automatic-answer batch planning to the selected provider.
    async fn plan(
        &self,
        request: ResearchPlanRequest,
        activity: mpsc::Sender<ActivityEvent>,
        cancellation: CancellationToken,
    ) -> Result<ResearchBatchPlan, AgentError> {
        match self {
            Self::Codex(client) => client.plan(request, activity, cancellation).await,
            Self::Local(client) => client.plan(request, activity, cancellation).await,
        }
    }

    /// Delegates one automatic-answer research worker to the selected provider.
    async fn research(
        &self,
        request: ResearchRequest,
        activity: mpsc::Sender<ActivityEvent>,
        cancellation: CancellationToken,
    ) -> Result<ResearchedAnswer, AgentError> {
        match self {
            Self::Codex(client) => {
                AutoAnswerClient::research(client, request, activity, cancellation).await
            }
            Self::Local(client) => {
                AutoAnswerClient::research(client, request, activity, cancellation).await
            }
        }
    }

    /// Delegates project-wide judgment to the selected provider.
    async fn judge(
        &self,
        request: ResearchJudgmentRequest,
        activity: mpsc::Sender<ActivityEvent>,
        cancellation: CancellationToken,
    ) -> Result<JudgedResearchBatch, AgentError> {
        match self {
            Self::Codex(client) => client.judge(request, activity, cancellation).await,
            Self::Local(client) => client.judge(request, activity, cancellation).await,
        }
    }
}

#[async_trait]
impl DocumentationClient for ConfiguredProvider {
    /// Delegates staged generation or repair without granting additional filesystem authority.
    async fn execute(
        &self,
        request: DocumentationRequest,
        activity: Option<mpsc::Sender<ActivityEvent>>,
        cancellation: CancellationToken,
    ) -> Result<(), AgentError> {
        match self {
            Self::Codex(client) => client.execute(request, activity, cancellation).await,
            Self::Local(client) => client.execute(request, activity, cancellation).await,
        }
    }
}
