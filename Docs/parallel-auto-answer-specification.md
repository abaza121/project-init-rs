# Specification: Parallel Auto Answer

## Objective

Improve cited automatic clarification quality for developers and product creators by researching up to three independent consequential questions concurrently, judging the provisional answers as one project-aware batch, and presenting truthful stage-based progress in the Ratatui workbench.

Interactive users activate the feature with `/auto-answer` or by running `project-init run ... --auto-answer` from a terminal. Existing non-interactive automation remains headless. Only validated judged answers may enter SQLite authority.

## Acceptance Criteria

1. Automatic answering uses a fixed maximum of three concurrent research workers; the count is not configurable.
2. When exactly one consequential open question is eligible, the runner selects that blocker directly without a provider coordinator call. Otherwise, a coordinator constrained to the current eligible durable IDs selects the blocker and up to two additional independent consequential open questions.
3. Each selected question is researched by one isolated Codex invocation. Explicit auto-answer delegation authorizes a concrete, evidence-informed provisional decision; where no unique answer exists, the worker selects the narrowest conservative and reversible default and labels its status in the notes.
4. A separate schema-constrained Codex judge reviews every successful candidate together and returns exactly one cited answer for each candidate. Refusal-shaped answers such as `FAIL: ...` are invalid rather than authoritative answers.
5. Failed worker research and invalid judge output are each retried once with the previous validation error included as feedback that the next response must correct.
6. Worker and judge results remain provisional until the complete judged batch validates. Accepted still-open answers are reconciled in one SQLite transaction.
7. Cancellation terminates coordinator, workers, or judge through the shared cancellation boundary and persists no provisional answer from that batch.
8. The TUI shows coordinator, three worker, and judge lanes with question identity, stage, latest sanitized activity, and evidence count where available. It does not show invented percentages or chain-of-thought.
9. `/auto-answer` starts the mode from the workbench. A project without an active run still requires an explicit approval-policy selection.
10. `run --auto-answer` and `new --run --auto-answer` open the progress TUI only when both standard input and output are terminals. Non-terminal callers retain headless behavior and final structured status output.
11. Existing `/resume`, single-question `ResearchClient`, offline execution, generation, repair, cancellation, and validation behavior remain compatible.

## Policy Decisions

- Three is a fixed concurrency ceiling, not a requirement to fill every slot.
- A single eligible question has one legal plan and bypasses provider selection; its research worker, separate judge, cancellation checks, and atomic adoption remain mandatory.
- Coordinator schemas enumerate only current eligible durable IDs and cap the batch size at the smaller of three and the eligible count. Display IDs and other questions in the project snapshot are not selection candidates.
- The coordinator must include the immediate `QuestionRequired` identity, preventing speculative work from starving the blocking workflow edge.
- Eligible questions are open questions at or above the project's consequential threshold.
- Delegated choices remain recommendations rather than user-authored facts, but they must resolve the supplied question instead of restating why it is uncertain.
- A batch failure records `research_failed`, retains the active run, and commits no answers from that batch.
- Questions no longer open when a judged batch commits are skipped. All remaining applicable answers commit atomically.
- Coordinator failure is fail-closed without retry. Worker and judge failures receive one retry as approved in the idea brief.
- A separate judge runs only after all workers finish, so no more than three Codex processes are concurrent.
- Transient parallel progress is presentation state and is not added to the authoritative activity schema in this version.

## State Model and Invariants

- Question identity is the durable question UUID; display identity is presentation-only.
- Batch and worker-slot numbers are local monotonic presentation identities for one run.
- SQLite questions, answers, evidence, requirements, and traces remain authoritative.
- Coordinator plans, worker candidates, judge results, and TUI lanes are provisional derived state.
- Every selected ID is unique, eligible, and part of the same project.
- A judged result contains exactly the candidate question IDs once each.
- A refusal or failure sentinel is not a valid researched answer even when it has structurally valid citations.
- A rejected, malformed, failed, or cancelled batch leaves authoritative answers and evidence unchanged.
- Successful batch reconciliation preserves the existing answer/evidence/requirement/trace invariants for every accepted answer.
- Progress history remains bounded, and worker activity cannot overwrite another worker's lane.
- Cancellation is checked before transactional adoption.

## Architecture

`AutoAnswerClient` is analogous to a C# orchestration interface. It exposes coordinator, worker-research, and judge operations while `CodexCliClient` supplies the production implementation. The runner owns an `Arc<dyn AutoAnswerClient>` so Tokio tasks can safely share the immutable client while each task owns its request and channel.

```text
WorkflowRunner
  -> collect eligible consequential questions
  -> select sole eligible blocker locally, otherwise AutoAnswerClient::plan
  -> up to three AutoAnswerClient::research futures
  -> AutoAnswerClient::judge
  -> validate current question state
  -> ProjectService::answer_research_batch
  -> one SQLite transaction

AutoAnswerProgress channel
  -> coordinator lane
  -> worker 1..3 lanes
  -> judge lane
  -> WorkspaceState
  -> Ratatui overlay
```

Rust's `Arc` is comparable to an immutable, thread-safe shared reference with reference counting. It does not make mutable state implicitly safe: requests, responses, and activity lanes remain independently owned. `Result<T, E>` carries every recoverable provider, validation, and persistence failure explicitly rather than using exceptions.

## Public Boundaries and Types

- `AutoAnswerClient`: provider-neutral coordinator, research, and judge trait.
- `ResearchPlanRequest` / `ResearchBatchPlan`: bounded coordinator contract.
- `ResearchJudgmentRequest` / `JudgedResearchBatch`: bounded judge contract.
- `AutoAnswerProgress`, `AutoAnswerActor`, and `AutoAnswerStage`: structured transient progress contract.
- `WorkflowRunner::with_auto_answer_client`: additive opt-in builder preserving `with_research_client`.
- `ProjectService::answer_questions_from_research`: atomic batch adoption boundary.
- `WorkspaceCommand::AutoAnswer`: explicit interactive activation.

## Error Handling

- Invalid coordinator IDs, duplicates, or oversized plans return `AgentError::InvalidResponse` before workers start.
- Worker failures retry only the failed question once and receive the first validation error as bounded feedback. A second failure cancels remaining provisional batch work and returns the error.
- Judge output with refusal text, missing, extra, or duplicate question IDs is invalid and retries once with bounded validation feedback.
- User cancellation is reported as `WorkflowRunStop::Cancelled`; provider or validation failures pause with `research_failed`.
- Channel closure never promotes incomplete progress or provider output to authority.
- Batch storage validates every applicable answer before mutation and rolls back on any persistence error.

## Commands

```text
Targeted runner tests: cargo test --test runner_contract parallel_auto_answer
Targeted TUI tests: cargo test --test workspace_tui_contract auto_answer
Agent tests: cargo test --test agent_contract
Format: cargo fmt --check
Check: cargo check
Full tests: cargo test
Lint: cargo clippy --all-targets --all-features -- -D warnings
```

## Project Structure

- `src/agents/mod.rs`: provider-neutral request, response, validation, and trait contracts.
- `src/agents/codex.rs`: schemas, prompts, and Codex implementations.
- `src/workflow/runner.rs`: concurrent orchestration, retry, progress, and cancellation.
- `src/workflow/mod.rs` and `src/storage/workflow.rs`: atomic authoritative adoption.
- `src/tui/workspace.rs` and `src/tui/mod.rs`: activation, transient lane state, rendering, and runtime wiring.
- `src/main.rs`: terminal-aware CLI routing.
- `tests/`: behavior and integration contracts.

## Code Style

Every Rust function, struct, and enum receives a behavior-focused `///` documentation comment. Recoverable paths return typed `Result` values, production code avoids `unwrap` and `expect`, and inline comments explain only non-obvious invariants in long or complex functions.

## Testing Strategy

- Pure contract tests validate bounded plans, exact judge membership, duplicates, and malformed JSON.
- Runner tests use deterministic fake clients to prove concurrency, fixed capacity, retry limits, judge ordering, cancellation, and no partial adoption.
- Single-question tests bypass an unreliable selector but still require research and judgment, including cancellation before adoption. Multi-question tests reject extra ineligible IDs before research or persistence.
- Storage/workflow tests prove atomic multi-answer reconciliation and recovery after rejection.
- Ratatui `TestBackend` tests prove `/auto-answer` parsing and lane rendering without a real terminal.
- CLI parser and routing helpers prove interactive versus non-interactive selection without launching Codex.
- Existing full-suite tests provide compatibility coverage.

## Boundaries

- **Always:** preserve SQLite authority, validate untrusted Codex output, keep progress bounded and sanitized, and re-run full Rust quality checks.
- **Ask first:** add dependencies, change database schema, make concurrency configurable, or alter approval semantics.
- **Never:** persist raw chain-of-thought, bypass Codex sandboxing, accept partial malformed batches, or overwrite unrelated worktree changes.

## Non-Goals

- Multiple workers researching the same question.
- Configurable worker count or model selection.
- Concurrent documentation generation or repair.
- Durable provisional worker transcripts.
- Background continuation after the owning TUI exits.
- Percentage completion estimates.
- A new dependency or database migration.

## Evaluator Risks

- The planner may select dependent questions despite its prompt; exact-ID validation cannot prove semantic independence.
- Three simultaneous Codex processes may hit host or account limits.
- A judge can make a well-cited worker answer worse; quality requires evaluation beyond structural tests.
- External processes can answer a question between research and adoption; the transaction must skip stale identities safely.
- Cancellation can race with a completed judge response; the final pre-adoption check is mandatory.

## Success Criteria

- All acceptance criteria have behavior-focused automated coverage.
- The TUI remains responsive and cancellable with three active workers.
- No failed or cancelled batch adds answers or evidence.
- Existing quality commands pass without warnings.
- A documented evaluation harness can compare judged and legacy answer quality, even though live paid evaluation is not part of deterministic tests.

## Open Questions

No question blocks implementation. The quality threshold that justifies additional Codex cost remains a post-implementation product evaluation decision.
