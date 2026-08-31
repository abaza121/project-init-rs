---
name: rust-solution-workflow-auto
description: Build or change Rust solutions through a fully automated sequence of specification, adversarial test-first development, implementation, verification, review, and improvement. Use for Rust exercises, challenge solutions, small services, libraries, and focused repository changes that need hidden-test resilience, implicit-contract analysis, or architecture explained for a C# background, without phase-by-phase approval gates.
---

# Automated Rust Solution Workflow

Deliver Rust work through this uninterrupted sequence:

`specification -> tests -> agent implementation -> tests -> review -> improve`

Invocation authorizes automatic progression through every phase for the requested Rust task. Do not pause at phase boundaries or ask for routine confirmation. Preserve the user's scope and all authorization, sandbox, safety, repository, and system boundaries; automation removes workflow approval gates, not those boundaries.

## Start by selecting supporting skills

When available, invoke `agent-skills:using-agent-skills` before beginning. Use it to select only the skills needed for the current phase. Common choices are:

- specification: `spec-driven-development`, `planning-and-task-breakdown`, and `source-driven-development` when external APIs or libraries matter;
- tests and implementation: `test-driven-development` and `incremental-implementation`;
- review and improvement: `code-review-and-quality` and, only when complexity warrants it, `code-simplification`.

Read and follow each selected skill when its phase begins. Do not load every related skill preemptively. If a supporting skill contains a routine approval gate, treat this skill's invocation as approval to continue through that gate when the next action remains in scope and authorized.

## Hidden tests and implicit contracts

Treat hidden-test resilience as contract discovery, not as a separate patching phase. Before designing tests or implementation, reconstruct the full observable contract from the user request, rubric, repository documentation, existing tests, public types, command-line or wire formats, error shapes, resource limits, and established behavior. A deliverable named in the request is part of the contract even when no public test mentions it.

Separate the result into four categories and carry them through every checkpoint:

- **Explicit behavior:** directly required outcomes and compatibility constraints.
- **Policy decisions:** ambiguous cases for which more than one coherent behavior is possible.
- **Implementation invariants:** facts that must remain true after every accepted, ignored, or rejected operation.
- **Evaluator risks:** plausible combinations or scale conditions not yet proven by tests.

Do not silently turn an assumption into architecture. When ambiguity materially changes externally visible behavior, memory growth, compatibility, or failure semantics, resolve it from repository evidence when possible. Otherwise choose the safest reversible local behavior, record the choice and rationale, and add a corresponding test. Stop only when no responsible interpretation permits progress.

For stateful or ordering problems, define the state model before enumerating examples. At minimum identify:

- identity keys, ordering keys, and whether their scope is global, per stream, or per entity;
- authoritative state, derived counters or indexes, and ownership of each stored value;
- legal transitions, idempotency, duplicate policy, and conflict precedence;
- history or deduplication retention, including how every secondary structure stays bounded;
- capacity accounting and which operations remain allowed at a full boundary;
- invalid, overflow, exhaustion, and terminal-state behavior;
- atomicity: rejected or malformed input must not partially mutate state;
- hot-path complexity in terms of the relevant local and global state, plus total output work.

Derive adversarial tests from that model. Select representative combinations across boundary neighbors, operation permutations, interleaved entities, duplicates before and after side effects, identity and ordering collisions, large gaps, cascading release, malformed input between valid operations, full-capacity recovery, numeric extremes, and runtime or memory pressure. Prefer transition tables, small reference models, parameterized cases, or property tests when they prove a family of behavior. Do not add one-off branches for anticipated hidden examples.

Maintain these default invariants unless repository evidence or the user request says otherwise:

- accepted deliverable data is never silently lost or overwritten;
- conflict and duplicate checks occur before consuming capacity or mutating counters;
- a rejected operation leaves authoritative state and derived accounting unchanged;
- aggregate counters equal the state they summarize;
- independent partitions cannot block one another accidentally;
- bounds cover secondary indexes and retained history, not only the obvious primary buffer;
- work is proportional to the affected partition and released output, without repeated whole-state scans.

## Rust commenting strategy

- Add a Rust documentation comment (`///`) to every function, struct, and enum, including private and test items. Each comment must explain what the item represents or what behavior it provides, rather than merely restating its name or syntax.
- Within a function body, use inline comments only when the algorithm is complex or the function is longer than 18 lines. Keep them focused on non-obvious intent, invariants, or reasoning rather than narrating individual statements.
- Preserve useful existing comments and update them whenever the associated behavior changes.
- Treat missing, inaccurate, or unnecessary comments as review findings and resolve them before completion.

## Automated checkpoint protocol

At the end of every phase:

1. Record a compact checkpoint containing the artifacts produced, decisions made, evidence gathered, and unresolved tradeoffs.
2. State what the next phase will do and which files it may change.
3. Continue immediately into the next phase without requesting approval or ending the turn.

Revise earlier decisions automatically when later evidence invalidates them. Keep the final response self-contained by consolidating the important checkpoint information rather than requiring the user to reconstruct it from intermediate updates.

## Phase 1: Specification

Inspect the repository and relevant authoritative requirements without modifying code. Surface assumptions and conflicts.

The checkpoint must cover:

- objective, acceptance criteria, and non-goals;
- the smallest viable architecture and data flow;
- modules, public boundaries, and core data types;
- error-handling strategy and likely failure cases;
- explicit behavior, ambiguous policy decisions, state invariants, and evaluator risks;
- compatibility surfaces, deliverable files, and implicit contracts inferred from existing behavior;
- an ordered implementation plan with small verification points;
- dependencies, with a reason for every non-standard dependency;
- open questions that materially change the solution.

Explain architecture for a C# programmer. Introduce the familiar concept first, then the Rust concept, for example:

- a crate is roughly a .NET project and assembly, while a Rust module is closer to a namespace with real privacy boundaries;
- a trait resembles an interface, but also participates in generic constraints and static dispatch;
- a Rust enum with data-bearing variants is a closed discriminated union, not a C# numeric enum;
- `Result<T, E>` makes recoverable failure part of the return type instead of relying on exceptions;
- borrowing passes temporary access without transferring ownership; call out where this differs materially from managed references.

Use analogies to orient the reader, not to erase important Rust semantics. Prefer a small architecture diagram or comparison table only when it materially improves clarity.

**Checkpoint:** Record the specification and proceed directly to tests.

## Phase 2: Tests

Turn the acceptance criteria into behavior-focused tests before production implementation.

- Cover the normal path, boundary cases, and meaningful error paths.
- Build tests from the state transitions and invariants, then combine dimensions likely to interact instead of testing every concern only in isolation.
- Include exact limit boundaries, post-error recovery, conflict precedence, and counter/state consistency where applicable.
- Use a small reference model or property-based testing when many permutations share one oracle and the added complexity is justified.
- Name tests as behavior statements and assert outputs or state rather than internal call sequences.
- Prefer unit tests for pure logic and add integration tests only at real boundaries.
- Avoid mocks when real deterministic code or a small fake is practical.
- Run the narrow test command and confirm the new tests fail for the intended missing behavior. A compile failure is acceptable only when the new public contract does not exist yet; prefer a behavioral failure when a minimal stub is reasonable.
- Do not weaken, skip, or rewrite a valid test merely to make later implementation easier.

The checkpoint must include the test names, files changed, mapped acceptance criteria, and the observed red-state evidence. Also identify which hidden-test families are proven, which remain untested, and why the selected cases generalize beyond their concrete values.

**Checkpoint:** Record red-state evidence and proceed directly to implementation.

## Phase 3: Agent implementation

Implement the smallest clear Rust change that satisfies the tests.

- Use stable Rust unless the specification explicitly requires nightly.
- Prefer the standard library and existing dependencies.
- Model valid states with types when that reduces real error handling; avoid elaborate type machinery for hypothetical cases.
- Use idiomatic `Result` and `?` for recoverable failures. Avoid `unwrap`, `expect`, and panics in production paths unless an invariant is locally proven and documented.
- Keep ownership simple: borrow by default when the caller retains data, move when ownership naturally transfers, and clone only when its cost and purpose are clear.
- Prefer explicit, readable control flow over clever iterator chains or premature abstraction.
- Add async, concurrency, generics, traits, or macros only when the requirements earn their complexity.
- Preserve unrelated user changes and repository conventions.
- Implement one coherent transition model. Perform validation, duplicate/conflict classification, overflow checks, and capacity checks before mutation when possible; otherwise make rollback explicit and test it.
- Keep authoritative state in one place and make indexes or counters mechanically consistent with it.
- Preserve public and protocol compatibility unless the specification explicitly changes it.

Use targeted red-green iterations while implementing. Do not expand scope or perform unrelated cleanup. Explain non-obvious Rust choices using concise C# comparisons in the checkpoint, not tutorial comments in otherwise clear code.

The checkpoint must include changed files, important design choices, targeted test evidence, and any deviation from the specification.

**Checkpoint:** Record the green implementation evidence and proceed directly to full verification.

## Phase 4: Tests and verification

Run the repository's established quality commands. For a conventional Cargo project, use this baseline unless project instructions specify otherwise:

```text
cargo fmt --check
cargo check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

If formatting fails and the requested work authorizes repository edits, run `cargo fmt`, inspect the resulting diff for unrelated churn, and rerun the check. Do not hide warnings, skip failing tests, or claim success from partial output. Add an end-to-end or boundary check when the acceptance criteria are not fully proven by the automated suite.

Before declaring verification complete, exercise the highest-risk combinations identified in Phase 1 at realistic scale. Check exact boundaries on both sides, a long cascading or repeated transition, recovery after rejection or malformed input, and independent-state interleaving when those concepts exist. Measure or inspect complexity and memory behavior when the rubric makes performance part of correctness; a passing small test alone is insufficient evidence.

The checkpoint must list every command, its result, test counts where available, and any remaining verification gap.

**Checkpoint:** Record verification evidence and proceed directly to code review.

## Phase 5: Review

Review the change against a fresh diff before editing it further. Evaluate:

- correctness against the specification and tests;
- implicit contracts, deliverable completeness, and compatibility with existing callers or protocol clients;
- hidden-test families reconstructed fresh from the state model, especially combinations absent from the authored tests;
- readability and Rust idioms;
- architecture and unnecessary abstractions;
- input, resource, and dependency risks appropriate to the task;
- performance problems supported by evidence rather than speculation;
- dead code and accidental scope expansion.

Label findings by severity, give exact file locations, and distinguish required fixes from optional preferences. If there are no findings, say so and identify any residual risks or untested assumptions.

The checkpoint must contain the verdict and improvement set. Automatically accept and apply required correctness, security, compatibility, and scope findings. Apply optional improvements only when they are low-risk, clearly beneficial, and do not expand scope.

**Checkpoint:** Record the verdict and proceed directly to improvement.

## Phase 6: Improve

Apply the selected review findings. Keep behavior changes test-first and keep refactors behavior-preserving.

- Make the narrowest improvements that resolve the findings.
- Add a regression test for every correctness or state-invariant defect before fixing it.
- Re-run affected targeted tests after each meaningful change.
- Re-run the full Phase 4 suite after the final edit.
- Summarize the final architecture, files changed, verification evidence, and explicitly deferred items.

Do not commit, push, publish, deploy, install system-wide dependencies, mutate external systems, or perform other separately authorized actions unless the user requested that action or repository instructions require it.

**Completion:** Present the completed result once the full workflow is finished. Do not ask for intermediate acceptance.

## Stop conditions

Stop only when requirements irreconcilably conflict, a test fails for an unexpected reason that cannot be safely resolved in scope, verification tooling is unavailable with no equivalent local check, user-owned changes overlap unsafely, or the next action needs authority not granted by the request. Exhaust safe in-scope diagnostics and alternatives first. Provide the evidence and the smallest decision or authorization needed from the user.
