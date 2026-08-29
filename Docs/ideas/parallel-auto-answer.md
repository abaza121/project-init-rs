# Parallel Auto Answer

## Problem Statement

How might we improve automatic clarification quality by researching several independent project questions concurrently, while showing honest progress in the TUI and preserving authoritative workflow consistency?

## Recommended Direction

Introduce a fixed three-worker research pool. A coordinator selects up to three independent open questions, each worker researches one question, and a separate Codex judge reviews the completed batch. The judge returns one final cited answer per question after checking evidence quality, answer completeness, and consistency with the project.

The workbench exposes this as `/auto-answer`. Running `run --auto-answer` in an interactive terminal opens the same progress TUI; non-interactive automation remains headless and preserves stable output behavior.

No answer becomes authoritative while research is running. The judged batch is validated first, then reconciled in deterministic order. Questions made obsolete by an earlier answer are skipped rather than answered with stale research.

## TUI Experience

The progress overlay shows observable stages, never invented percentages:

```text
AUTO ANSWER - BATCH 2

Worker 1  Q-004  Researching     00:18
          Comparing desktop deployment options...

Worker 2  Q-007  Evidence ready  00:14
          4 cited sources

Worker 3  Q-009  Retrying        00:27
          First response failed citation validation

Judge     Waiting for Worker 3

Esc: cancel all work and preserve current authoritative state
```

Each lane reports:

- Worker number and question display ID.
- Short question summary.
- State: queued, researching, validating, ready, retrying, failed, or skipped.
- Elapsed time and latest sanitized Codex activity.
- Citation count once a candidate has validated.
- Judge state: waiting, reviewing, validating, committing, or rejected.

## User Value

This is valuable if current single-pass auto answers are inconsistent, weakly supported, or locally reasonable but contradictory as a set. The visible board also fixes the current mismatch where `--auto-answer` suppresses the interactive experience.

Parallelism alone does not improve individual answers. The quality gain must come from the judge checking citations, project-wide consistency, and unsupported inference. A blind evaluation against the current single-worker mode must demonstrate better citation correctness and project consistency.

## Key Assumptions to Validate

- [ ] A judge materially improves answers over direct worker output. Compare blind scores for citation support, relevance, and cross-answer consistency.
- [ ] Three independent eligible questions commonly exist. Measure real project queues before optimizing around batches of three.
- [ ] Concurrent Codex execution does not cause unacceptable rate-limit failures or cost. Record batch latency and token usage.
- [ ] Users can understand stage-based progress without seeing raw model reasoning. Test the overlay with representative runs.
- [ ] Provisional answers can be revalidated after earlier answers change project state. Test superseded and dependent-question scenarios.

## MVP Scope

- A `/auto-answer` command in the interactive workbench.
- TTY-aware `run --auto-answer` progress TUI.
- Exactly three concurrent worker slots, not configurable.
- Dependency-aware selection of up to three questions.
- One research candidate per selected question.
- One post-research judge invocation for the batch.
- Strict structured validation of worker and judge responses.
- One automatic retry for a failed worker.
- Atomic acceptance of valid, still-applicable judged answers.
- Immediate `Esc` cancellation of coordinator, workers, and judge.
- Deterministic fake clients and subprocess fixtures for tests.
- Quality evaluation against the current single-worker baseline.

If a retry still fails, the batch pauses visibly without committing partial answers.

## Failure Policy

- One worker failure: retry that question once.
- Repeated worker failure: pause the batch.
- Invalid judge output: retry the judge once, then pause.
- Cancellation: terminate every child and commit nothing.
- Superseded question: mark it skipped in the TUI.
- Partial reconciliation failure: roll back the entire batch.
- Fewer than three eligible questions: run the available workers without filling empty slots.

## Not Doing

- **Multiple workers answering the same question** - different-question concurrency is the chosen model.
- **Configurable concurrency** - three workers is a fixed product decision.
- **Percentage progress bars** - Codex cannot supply honest completion percentages.
- **Raw chain-of-thought display** - show sanitized operational activity only.
- **Background continuation after leaving the TUI** - cancellation and ownership remain explicit.
- **Accepting the first completed answer** - every candidate must pass judgment and validation.
- **Changing package-generation concurrency** - this feature applies only to clarification answering.
- **Persisting provisional worker output as authoritative history** - only judged answers enter project state.

## Success Criteria

Compared with the current single-worker mode:

- Higher blind reviewer scores for evidence support and project fit.
- Fewer contradictory answers within the same project.
- No increase in unsupported or malformed accepted answers.
- No partial authoritative mutation after failure or cancellation.
- Progress remains responsive throughout concurrent execution.

## Open Questions

- What minimum blind-evaluation improvement justifies the additional Codex cost?
- Should automation receive transient progress events in a future structured streaming format?
