You are a senior Rust engineer and agentic-workflow architect.

You are working in the existing repository:

https://github.com/abaza121/project-initiation-pipeline

Your task is to build a new Rust-based version of this project that represents a meaningful improvement over the existing project-initiation pipeline.

IMPORTANT CONTEXT

The existing implementation is the BASELINE.

Do not remove it, rewrite its history, or disguise it.

The current pipeline takes a short project idea and generates a collection of project-initiation documents through a mostly fixed sequence of prompts.

The new implementation must preserve the useful parts of that idea while changing the architecture from:

    project prompt
        ↓
    fixed document-generation pipeline

into:

    project prompt
        ↓
    structured findings
        ↓
    uncertainty analysis
        ↓
    interactive user questions
        ↓
    persistent project knowledge base
        ↓
    research / evidence
        ↓
    decisions
        ↓
    traceability
        ↓
    verification
        ↓
    generated project documents

The important improvement is NOT simply "use Rust instead of PowerShell".

The important improvement is that project knowledge becomes structured, persistent, traceable, and correctable before documents are generated.

==================================================
PRIMARY PRODUCT GOAL
==================================================

Build a terminal application that helps a developer transform an incomplete project idea into a coherent, evidence-backed, implementation-ready project foundation.

The application should:

1. Accept a short natural-language project brief.
2. Analyze the brief into structured findings.
3. Distinguish:
   - confirmed facts,
   - requirements,
   - assumptions,
   - unknowns,
   - constraints,
   - risks,
   - proposed decisions,
   - research questions.
4. Persist those findings in a local database.
5. Determine which uncertainties are important enough to ask the user about.
6. Ask the user targeted questions through an interactive TUI.
7. Store every answer in the database with provenance.
8. Re-evaluate the project model after answers are received.
9. Research important remaining uncertainties where appropriate.
10. Record research evidence separately from conclusions.
11. Create project decisions that explicitly reference:
    - user requirements,
    - user answers,
    - assumptions,
    - and/or research evidence.
12. Generate the final project-initiation documents from the structured project model.
13. Verify the generated package for contradictions, unsupported decisions, missing traceability, and unresolved high-impact uncertainties.
14. Repair correctable problems and report anything requiring human clarification.

==================================================
BASELINE PRESERVATION
==================================================

Before changing anything:

1. Inspect the existing repository.
2. Document how the existing pipeline works.
3. Identify:
   - its input,
   - stages,
   - outputs,
   - assumptions,
   - current limitations.
4. Do not remove the original implementation.
5. Add a document such as:

    docs/baseline.md

that explains that the pre-existing implementation is the hackathon baseline.

Where practical, preserve a command for running the baseline independently from the new Rust implementation.

Do not artificially weaken the baseline.

==================================================
TECHNOLOGY
==================================================

Use stable Rust.

Prefer a Cargo workspace if justified by the architecture.

Primary libraries should include or consider:

- ratatui
- crossterm
- rusqlite with bundled SQLite
- lancedb
- Apache Arrow types required by LanceDB
- serde
- serde_json
- uuid
- chrono
- anyhow
- thiserror
- clap
- tracing
- tracing-subscriber

Use the current stable LanceDB Rust APIs.

Keep LanceDB-specific API usage isolated behind a repository or
adapter boundary because its APIs may evolve independently from
the domain model.

Do not allow LanceDB types to leak throughout the application.

Use Tokio only where asynchronous execution has a concrete
benefit, including where required by the selected LanceDB or
agent-client APIs.

==================================================
HIGH-LEVEL ARCHITECTURE
==================================================

Design clear modules around concepts similar to:

src/
    main.rs

    app/
        mod.rs
        state.rs

    domain/
        project.rs
        finding.rs
        requirement.rs
        question.rs
        answer.rs
        evidence.rs
        decision.rs
        trace.rs
        validation.rs

    storage/
        mod.rs
        sqlite.rs
        migrations.rs

    tui/
        mod.rs
        screens/
        components/

    workflow/
        mod.rs
        analyze.rs
        clarify.rs
        research.rs
        decide.rs
        generate.rs
        verify.rs

    agents/
        mod.rs
        client.rs
        prompts.rs
        structured_output.rs

    documents/
        mod.rs
        renderer.rs
        templates.rs

    retrieval/
        mod.rs
        index.rs
        embeddings.rs
        lancedb_store.rs
        query.rs
        sync.rs
        context_builder.rs

Exact structure may change if you have a better design.

Explain architectural deviations before implementing them.

==================================================
THE PROJECT KNOWLEDGE MODEL
==================================================

Do NOT treat arbitrary Markdown as the central project state.

Introduce typed domain structures.

At minimum support concepts equivalent to:

Finding
Requirement
Assumption
Unknown
Constraint
Risk
Question
Answer
Evidence
Decision
TraceLink
Document
ValidationFinding

A finding should contain information similar to:

Finding {
    id
    project_id
    kind
    statement
    source_type
    source_reference
    confidence
    impact
    status
    created_at
    updated_at
}

FindingKind should support at least:

ConfirmedFact
Requirement
Assumption
Unknown
Constraint
Risk
ResearchQuestion

Use typed enums rather than magic strings wherever practical.

==================================================
PROVENANCE
==================================================

Every important project statement must have provenance.

For example:

USER_BRIEF
USER_ANSWER
RESEARCH
AGENT_INFERENCE
DERIVED
SYSTEM

The system must never silently promote an AGENT_INFERENCE into a USER_REQUIREMENT.

Example:

User says:

    "I want to make a VR fishing game."

The system may NOT silently conclude:

    "The target platform is Meta Quest 3."

Instead it should record something like:

    Unknown:
        Which VR platform is targeted?

or, when necessary:

    Assumption:
        PC VR is tentatively assumed.
        Confidence: Low
        Impact: High
        Requires confirmation: true

==================================================
DATABASE
==================================================

Design SQLite migrations for at least:

projects
findings
requirements
questions
answers
evidence
decisions
trace_links
documents
validation_runs
validation_findings

You may normalize or combine tables when justified.

Preserve enough structure to answer questions such as:

- Which requirement came directly from the user?
- Which requirement was inferred?
- Which answer changed this requirement?
- Which evidence influenced this decision?
- Which architecture decision satisfies REQ-007?
- Which assumptions remain unresolved?
- Which findings were superseded?
- Why was a generated statement included?

==================================================
PROJECT LIFECYCLE
==================================================

Represent workflow status explicitly.

For example:

Draft
Analyzing
AwaitingClarification
Researching
Planning
Generating
Validating
NeedsUserInput
Complete

The application should be resumable.

Closing the application must not lose the project state.

A user should be able to run:

    project-init open <project>

and continue from the previous state.

==================================================
INITIAL ANALYSIS
==================================================

When a project brief is submitted, run an analysis stage.

Require structured output from the analysis agent.

Do not allow the agent to return only prose.

The analysis should extract:

- explicit facts
- explicit requirements
- goals
- constraints
- assumptions
- ambiguities
- contradictions
- unknowns
- risks
- potential research questions

Persist this result before proceeding.

==================================================
QUESTION PRIORITIZATION
==================================================

Do NOT ask the user about every missing detail.

Questions should be prioritized according to something similar to:

    priority =
        impact × uncertainty × cost_of_being_wrong

Store these values.

High-impact architectural or product uncertainties should be asked first.

Low-impact details can remain assumptions or deferred decisions.

Examples of high-impact questions:

- What platform is being targeted?
- Is multiplayer required?
- Must data remain entirely local?
- Is authentication required?
- Is there a specific engine/framework requirement?
- Are there regulatory/privacy constraints?

Examples of low-impact questions that usually should not interrupt the user:

- Exact accent color
- Exact directory names
- Minor naming preferences

Make thresholds configurable.

==================================================
TUI
==================================================

Build an actual interactive TUI using ratatui.

The application should not simply print stdin prompts one after another.

Provide screens or panels approximately equivalent to:

PROJECT OVERVIEW

Project:
Calm Fishing VR

Status:
Awaiting Clarification

Progress:
Brief analyzed
12 facts
8 requirements
5 assumptions
4 important unknowns


CLARIFICATION

Question 2 of 4

Which VR platform should be considered the primary target?

> Meta Quest
  PC VR
  PlayStation VR2
  Multiple platforms
  Not decided yet
  Custom answer

Why this matters:
Platform choice affects rendering constraints,
SDK choices, input APIs and performance targets.

Impact: HIGH


FINDINGS

[F] Confirmed
[A] Assumption
[?] Unknown
[R] Requirement
[!] Risk

Allow the user to inspect findings before continuing.

Useful controls could include:

Enter   Select
E       Edit
C       Confirm
D       Defer
Esc     Back
Q       Quit safely

The exact UX is up to you, but it should feel intentional and usable.

==================================================
USER ANSWERS
==================================================

User answers must not merely be appended to a text prompt.

Store them as first-class records.

An answer should include:

- question ID
- answer text/value
- timestamp
- answer source
- whether it resolves the question
- optional notes

After an answer is recorded, run a reconciliation step.

That step may:

- confirm an assumption,
- reject an assumption,
- create a requirement,
- modify a constraint,
- close an unknown,
- create new unknowns,
- invalidate an existing decision.

Keep history.

Do not destructively overwrite previous reasoning without traceability.

==================================================
RESEARCH
==================================================

Research should happen BEFORE major decisions that depend on it.

Do not reproduce the weakness:

    architecture
        ↓
    research

Instead use:

    uncertainty
        ↓
    research
        ↓
    evidence
        ↓
    decision

Only research questions whose answers can materially influence the project.

Persist evidence independently.

Evidence should contain fields similar to:

Evidence {
    id
    project_id
    research_question_id
    claim
    source
    source_title
    retrieved_at
    reliability
    notes
}

A decision may reference multiple pieces of evidence.

Do not treat research output as automatically true.

==================================================
DECISIONS
==================================================

Create first-class decisions.

For example:

Decision {
    id
    title
    statement
    status
    rationale
}

DecisionStatus:

Proposed
Accepted
Rejected
Superseded
NeedsConfirmation

Architecture decisions must be traceable.

Example:

ADR-004
Use SQLite for local project state.

Supports:
REQ-012
REQ-015

Based on:
ANSWER-004
EVIDENCE-009

Assumes:
ASM-002

==================================================
TRACEABILITY
==================================================

Create generic trace links.

For example:

Requirement -> UserAnswer
Requirement -> Finding
Decision -> Requirement
Decision -> Evidence
Decision -> Assumption
DocumentSection -> Decision
DocumentSection -> Requirement

Prefer a generic trace-link structure capable of representing relationships such as:

SUPPORTS
DERIVED_FROM
ANSWERS
IMPLEMENTS
CONSTRAINS
CONTRADICTS
SUPERSEDES
CITES

The final project should be queryable as a graph even though storage is relational.

==================================================
REQUIREMENTS
==================================================

Give requirements stable IDs:

REQ-001
REQ-002
...

Each important requirement should include:

- statement
- source
- priority
- status
- rationale when useful
- acceptance criteria
- relevant traces

Acceptance criteria should be concrete enough for a developer or later agent to understand what "done" means.

==================================================
CONTRADICTION DETECTION
==================================================

Detect contradictions.

Example input:

    All journal data must remain exclusively on the device.

and:

    Journals must automatically synchronize across all user devices.

Do NOT silently design around this.

Create something similar to:

CONFLICT-001

Statements:
REQ-003
REQ-008

Severity:
HIGH

Resolution:
Requires user clarification.

The workflow should transition to NeedsUserInput when an unresolved contradiction blocks consequential decisions.

==================================================
DOCUMENT GENERATION
==================================================

Only generate final project documentation after sufficient clarification and evidence collection.

Generate useful documents such as:

README.md
docs/Requirements.md
docs/MissionVision.md
docs/SWOT.md
docs/TechnicalArchitecture.md
docs/VisualIdentity.md
docs/BrandPrompt.md
docs/Research/
docs/DecisionLog.md
docs/Assumptions.md
docs/OpenQuestions.md
docs/Traceability.md
docs/ValidationReport.md

Adapt filenames to the existing repository where maintaining compatibility is useful.

Documents must be generated FROM the database.

Do not create independent documents that can contradict the project model.

Include IDs such as REQ-004, ADR-002 and EVIDENCE-007 where useful.

==================================================
VERIFICATION LOOP
==================================================

After documents are generated:

    Generate
       ↓
    Verify
       ↓
    PASS ─────────────→ Complete
       │
       FAIL
       ↓
    Repair
       ↓
    Verify again

Limit automatic repair attempts.

Do not endlessly self-correct.

High-impact failures that require new product decisions should return to the user rather than be guessed.

The verifier should check at minimum:

- explicit brief requirements were preserved
- user answers were respected
- unsupported assumptions were not promoted to facts
- project naming is consistent
- requirements contain acceptance criteria
- important requirements have architecture/product traces
- important decisions have provenance
- research-based claims reference evidence
- conflicting decisions are detected
- required artifacts exist
- generated links are valid
- unresolved HIGH-severity findings are visible

==================================================
CLI
==================================================

Provide a CLI around the TUI.

Potential interface:

project-init new
project-init new --brief project-prompt.txt
project-init open <project-id>
project-init list
project-init inspect <project-id>
project-init generate <project-id>
project-init validate <project-id>
project-init export <project-id>

project-init index status <project-id>

project-init index rebuild <project-id>

project-init index clear <project-id>

project-init inspect-related <entity-id>

project-init search <project-id> "<semantic query>"

project-init new --retrieval relational

project-init new --retrieval semantic

Running:

    project-init new --brief project-prompt.txt

should launch the TUI after initial analysis.

==================================================
AGENT INTEGRATION
==================================================

Isolate model/agent execution behind a trait.

For example:

trait AgentClient {
    fn execute(...) -> Result<...>;
}

The domain and persistence layers must not depend directly on one specific AI provider.

If the existing repository is designed around Codex CLI, implement a Codex CLI adapter first.

Make its command invocation visible and reproducible.

Store representative agent trajectories/logs suitable for the hackathon submission, while ensuring credentials and private information are never written to logs.

==================================================
STRUCTURED AGENT RESPONSES
==================================================

For workflow-critical stages, require JSON or another schema-valid structured response.

Validate it before inserting it into the database.

An invalid model response must not corrupt project state.

Use serde structs as the validation boundary.

==================================================
HUMAN CONTROL
==================================================

The user is authoritative.

The agent may:

- propose
- research
- infer
- recommend
- flag

The agent may NOT silently make consequential user decisions when an important ambiguity exists.

Give the user mechanisms to:

- accept a proposal
- reject it
- edit it
- defer it
- mark it undecided

==================================================
AUDITABILITY
==================================================

For a given final statement, we should ideally be able to answer:

    Why is this here?

Example:

TechnicalArchitecture.md:
"The application will use SQLite."

Trace:

Document section
    ↓
ADR-003
    ↓
REQ-011
    ↓
ANSWER-004
    ↓
Question:
"Must project state survive between sessions?"

This traceability is a major goal of the system.

==================================================
HACKATHON EXPERIMENTATION SUPPORT
==================================================

Make the implementation useful for comparing the baseline against the new system.

Add an evaluation directory similar to:

evaluation/
    cases/
    baseline/
    improved/
    reports/

Do not fabricate evaluation results.

Provide tooling so the same project brief can be given to both systems.

Record:

- model configuration where available
- execution time
- token usage where available
- number of user questions
- number of agent calls
- number of verification/repair passes
- output directory

==================================================
TESTING
==================================================

Add unit and integration tests.

Important test areas:

1. SQLite migrations.
2. Project persistence/resume.
3. Finding provenance.
4. Assumption confirmation/rejection.
5. Question prioritization.
6. Answer reconciliation.
7. Trace creation.
8. Contradiction representation.
9. Structured agent-response validation.
10. Document generation from known fixture data.
11. Validation calculations.
12. No project data is lost after application restart.

Create synthetic fixtures so tests do not depend on a live model.

==================================================
SAMPLE END-TO-END SCENARIO
==================================================

Use the existing fishing VR project prompt as one demonstration.

The expected behavior should look approximately like:

User brief
    ↓
Analyzer extracts:
    FACT-001 VR game
    FACT-002 Calm environment
    REQ-001 Fishing gameplay
    REQ-002 Exploration helps identify fishing locations
    UNKNOWN-001 Target VR platform
    UNKNOWN-002 Engine constraints
    ASSUMPTION-001 Single-player unless stated otherwise
    ...
    ↓
Question prioritizer
    ↓
TUI asks:
    "Which VR platform is the primary target?"
    ↓
User:
    "Meta Quest 3"
    ↓
ANSWER-001 stored
    ↓
UNKNOWN-001 resolved
    ↓
REQ/CONSTRAINT records updated
    ↓
Relevant research created
    ↓
Evidence recorded
    ↓
Architecture decisions made
    ↓
Documents generated
    ↓
Verifier checks them
    ↓
Final package

==================================================
IMPLEMENTATION PROCESS
==================================================

Do not try to implement everything blindly in one pass.

First:

1. Inspect the current repository.
2. Write a short architecture proposal.
3. Identify what remains from the baseline.
4. Propose the database schema.
5. Propose the domain model.
6. Propose the workflow state machine.
7. Then implement incrementally.

Suggested development order:

Milestone 1
Rust project + SQLite + project/finding domain model.

Milestone 2
Brief ingestion + structured analysis.

Milestone 3
Question prioritization + TUI.

Milestone 4
Answers + reconciliation + persistence.

Milestone 5
Requirements/evidence/decisions/traceability.

Milestone 6
Document generation.

Milestone 7
Verification/repair loop.

Milestone 8
Evaluation harness and documentation.

After each milestone:

- cargo fmt --check
- cargo check
- cargo test
- cargo clippy -- -D warnings

Fix failures before progressing.

==================================================
QUALITY BAR
==================================================

The final result should feel like a tool another developer could actually use.

Avoid:

- giant untyped JSON blobs
- one enormous Rust module
- prompts hidden inside source code without organization
- fake research
- fabricated citations
- silent assumptions
- meaningless multi-agent complexity
- agent loops without limits
- documents that duplicate conflicting state
- fake evaluation results

Prefer:

- explicit types
- provenance
- small modules
- deterministic checks where possible
- human clarification for consequential ambiguity
- inspectable agent trajectories
- repeatable commands
- clean Rust architecture

==================================================
FINAL DELIVERABLE
==================================================

When implementation is complete, provide:

1. Updated repository structure.
2. README with setup and usage.
3. Baseline documentation.
4. Database schema documentation.
5. Architecture documentation.
6. Example project run.
7. Example generated artifact package.
8. Test instructions.
9. Baseline vs improved evaluation instructions.
10. Improvement changelog.
11. Representative agent trajectory documentation.
12. List of known limitations.

Do not claim that the improved workflow is better until it has been evaluated.

The implementation should make that claim measurable.

==================================================
SEMANTIC RETRIEVAL TESTING
==================================================

Add tests covering:

1. LanceDB index creation.
2. Entity indexing.
3. Content-hash based re-indexing.
4. No re-index when semantic content is unchanged.
5. Superseded records excluded from active retrieval.
6. SQLite record remains valid when LanceDB indexing fails.
7. Rebuilding LanceDB entirely from SQLite.
8. Project isolation:
   project A results must never leak into project B.
9. Entity type filtering.
10. Duplicate-question candidate retrieval.
11. Answer retrieval for semantically equivalent questions.
12. Research evidence retrieval.
13. Contradiction candidate discovery.
14. Stale index detection.
15. Embedding dimension/schema validation.
16. Context top-K limits.
17. Semantic retrieval disabled mode.
18. Application operation when LanceDB is unavailable.

Use fake/deterministic embedding providers for most tests.

Unit and integration tests should not require paid embedding
API access.

==================================================
PERSISTENCE AND RETRIEVAL ARCHITECTURE
==================================================

Use two local persistence layers with clearly separated responsibilities:

1. SQLite
   - authoritative source of truth
   - structured project state
   - relationships
   - provenance
   - requirements
   - questions
   - answers
   - evidence
   - decisions
   - trace links
   - validation state

2. LanceDB
   - derived semantic search index
   - embeddings
   - semantic retrieval
   - candidate similarity discovery

The architecture is:

                     USER / AGENTS
                          │
                          ▼
                 ┌─────────────────┐
                 │ Domain Services │
                 └────────┬────────┘
                          │
                          ▼
                 ┌─────────────────┐
                 │     SQLite      │
                 │ Source of Truth │
                 └────────┬────────┘
                          │
                    changed entities
                          │
                          ▼
                 ┌─────────────────┐
                 │ Embedding Layer │
                 └────────┬────────┘
                          │
                          ▼
                 ┌─────────────────┐
                 │     LanceDB     │
                 │ Semantic Index  │
                 └─────────────────┘

LanceDB must NEVER replace explicit relational relationships.

Examples:

REQ-004 -> ADR-002

must be represented by a real TraceLink in SQLite.

Do not infer this relationship from vector similarity.

Likewise:

- user answers
- provenance
- decision status
- requirement status
- conflicts
- validation failures
- evidence linkage

must remain explicitly represented in SQLite.

The principle is:

    SQLite answers:
        "What do we know?"

    LanceDB answers:
        "What existing information might be relevant?"

    Agents answer:
        "What does that information mean?"

==================================================
SEMANTIC RETRIEVAL ABSTRACTION
==================================================

Do not couple the workflow directly to LanceDB.

Create abstractions similar to:

trait EmbeddingProvider {
    async fn embed(&self, text: &str) -> Result<Vec<f32>>;

    async fn embed_batch(
        &self,
        texts: &[String]
    ) -> Result<Vec<Vec<f32>>>;
}

trait SemanticIndex {
    async fn index(
        &self,
        records: &[SemanticRecord]
    ) -> Result<()>;

    async fn search(
        &self,
        query: SemanticQuery
    ) -> Result<Vec<SemanticMatch>>;

    async fn delete_entity(
        &self,
        project_id: ProjectId,
        entity_id: EntityId
    ) -> Result<()>;

    async fn rebuild_project(
        &self,
        project_id: ProjectId
    ) -> Result<()>;
}

Implement:

LanceDbSemanticIndex

as the initial SemanticIndex implementation.

The rest of the application should depend on SemanticIndex,
not LanceDB directly.

==================================================
LANCEDB SEMANTIC RECORD MODEL
==================================================

Create a dedicated LanceDB table for semantic project knowledge.

A semantic record should contain fields equivalent to:

SemanticRecord {
    row_id
    project_id
    entity_id
    entity_type
    text
    vector
    status
    content_hash
    embedding_model
    embedding_version
    created_at
    updated_at
}

Additional metadata may be added when useful.

entity_type may include:

Finding
Requirement
Assumption
Unknown
Question
Answer
Evidence
Decision
Constraint
Risk
DocumentChunk

Do NOT serialize entire application entities into the vector
database and treat that as persistence.

The vector record exists to locate an authoritative SQLite
entity.

Example:

LanceDB result:

    entity_type = Requirement
    entity_id   = REQ-014
    similarity  = ...

Then:

    load REQ-014 from SQLite

before using it as authoritative context.

Never trust a stale LanceDB text copy over the SQLite record.

==================================================
SEMANTIC TEXT REPRESENTATION
==================================================

Do not embed arbitrary JSON blobs.

Create an explicit textual representation for each entity.

Example requirement:

ENTITY TYPE: Requirement
ENTITY ID: REQ-014
STATUS: Active

STATEMENT:
The game must run on standalone VR hardware.

ACCEPTANCE CRITERIA:
The primary gameplay loop operates without requiring a
connected gaming PC.

SOURCE:
ANSWER-004

PRIORITY:
High

Example decision:

ENTITY TYPE: Decision
ENTITY ID: ADR-003

TITLE:
Use SQLite for local project storage.

DECISION:
Store authoritative project state in a local SQLite database.

RATIONALE:
The project must be resumable and usable without requiring
external infrastructure.

STATUS:
Accepted

The text representation should emphasize semantic meaning.

Do not include meaningless metadata simply to increase the
amount of indexed text.

==================================================
SQLITE <-> LANCEDB SYNCHRONIZATION
==================================================

LanceDB is a derived index and must be rebuildable entirely
from SQLite.

Never create information that exists only in LanceDB.

Every semantically indexable SQLite entity should have a
stable content hash.

When an entity changes:

    SQLite entity updated
        ↓
    calculate semantic representation
        ↓
    calculate content hash
        ↓
    compare with indexed hash
        ↓
    unchanged?
       /     \
     YES      NO
      ↓        ↓
    skip      regenerate embedding
                  ↓
               upsert LanceDB record

Track semantic index state in SQLite if useful.

For example:

semantic_index_state

- project_id
- entity_id
- entity_type
- content_hash
- indexed_at
- embedding_model
- embedding_version

If indexing fails:

- the SQLite operation must remain valid;
- mark the semantic index entry stale;
- log the failure;
- allow retry/rebuild.

Semantic retrieval is an optimization.

Failure of LanceDB must not corrupt authoritative project
state.

==================================================
STALE AND SUPERSEDED KNOWLEDGE
==================================================

Project knowledge will evolve.

Example:

ASM-004:
Target platform is probably PC VR.

Later:

ANSWER-009:
Primary platform is Meta Quest 3.

Do not allow semantic retrieval to keep presenting ASM-004
as active project truth.

When records become:

- superseded
- rejected
- resolved
- deleted
- inactive

update their semantic metadata or remove them from the active
LanceDB index.

Historical retrieval may still be supported explicitly.

Default agent context retrieval should normally prefer:

status = Active

Historical knowledge should only be returned when requested
or when useful for explaining why a decision changed.

==================================================
SEMANTIC QUESTION DEDUPLICATION
==================================================

Before showing a generated clarification question to the user,
perform semantic retrieval.

Example candidate:

    "Which VR headset should be the primary target?"

Search active:

- UserAnswers
- ConfirmedFacts
- Requirements
- Constraints
- PreviousQuestions

within the current project.

If LanceDB retrieves something such as:

ANSWER-009:
"We are targeting Meta Quest 3 only."

do not immediately ask the user again.

Instead pass the candidate question and retrieved context to
the clarification evaluator.

It must classify the question as:

AlreadyAnswered
PartiallyAnswered
NeedsUserInput

Only NeedsUserInput should normally appear in the TUI.

If PartiallyAnswered, the question should be rewritten around
the unresolved part.

Example:

Existing answer:
"We want standalone VR."

Do NOT ask:
"What kind of VR system are you targeting?"

Instead potentially ask:
"Which standalone VR headset is the primary target?"

Record:

- original candidate question
- retrieved semantic matches
- classification
- final question if shown

so the behavior can later be evaluated.

==================================================
AGENT CONTEXT BUILDER
==================================================

Never automatically send the entire project database to every
agent invocation.

Build focused context.

For each workflow operation:

1. Determine the task.
2. Determine required deterministic context.
3. Query LanceDB for potentially relevant additional context.
4. Resolve retrieved IDs against SQLite.
5. Remove:
   - stale entries
   - duplicates
   - superseded records
6. Rank and limit context.
7. Send the focused context to the agent.

Example:

Task:

    Decide project persistence architecture.

Deterministic context:

    relevant architecture constraints

Semantic query:

    "requirements, user decisions and evidence related to
     persistence, offline operation, synchronization,
     storage and privacy"

Filters:

    project_id = current_project
    status = Active

Entity types:

    Requirement
    Constraint
    Answer
    Evidence
    Decision
    Assumption

Return a configurable top-K set.

The context builder should retain entity IDs so agent output
can reference exact authoritative records.

==================================================
SEMANTIC CONTRADICTION CANDIDATE DISCOVERY
==================================================

Use LanceDB to help locate statements that discuss the same
concept.

Do NOT use vector similarity to determine whether statements
agree.

Example:

REQ-004:
"All personal data must remain on the user's device."

ADR-008:
"Persist user profiles in the hosted cloud database."

These may appear semantically similar because both discuss
data persistence.

LanceDB should identify the pair as worth inspecting.

Then a verifier determines whether their relationship is:

Supports
Compatible
Unrelated
Ambiguous
Contradicts

If Contradicts:

persist an explicit ValidationFinding or Conflict in SQLite.

The architecture is:

semantic similarity
       ↓
candidate pair
       ↓
reasoning / deterministic inspection
       ↓
explicit structured finding
       ↓
SQLite

==================================================
SEMANTIC RESEARCH REUSE
==================================================

Before starting new external research:

1. Convert the research question into a semantic query.
2. Search existing Evidence within the current project.
3. Retrieve relevant evidence.
4. Determine:
   - sufficient evidence already exists,
   - evidence is related but incomplete,
   - new research is required.

Do not reuse evidence solely because similarity is high.

Consider:

- source relevance
- source authority
- evidence age
- whether the underlying topic is time-sensitive
- whether the evidence directly supports the claim

If existing evidence is sufficient:

reuse it by explicit Evidence IDs.

Avoid unnecessarily repeating external research.

==================================================
SEMANTICALLY FOCUSED DOCUMENT GENERATION
==================================================

Generate documents section-by-section from structured state.

For each section, combine:

A. deterministic records required by the section

and

B. semantically retrieved supporting context

Example:

TechnicalArchitecture.md
Section:
Persistence

Retrieve semantic information related to:

- storage
- offline requirements
- synchronization
- user data
- privacy
- persistence
- backups

Do not generate the section from arbitrary global context.

Every important generated claim must still trace to
authoritative SQLite records.

==================================================
RETRIEVAL MODES
==================================================

Support at least:

relational
semantic

Relational mode:

- SQLite only
- no LanceDB semantic retrieval
- deterministic context selection

Semantic mode:

- SQLite remains authoritative
- LanceDB assists context retrieval

The same project brief and model configuration should be
runnable under both modes.

This enables an ablation experiment that determines whether
LanceDB actually improves the workflow.

==================================================
SEMANTIC RETRIEVAL TESTING
==================================================

Add tests covering:

1. LanceDB index creation.
2. Entity indexing.
3. Content-hash based re-indexing.
4. No re-index when semantic content is unchanged.
5. Superseded records excluded from active retrieval.
6. SQLite record remains valid when LanceDB indexing fails.
7. Rebuilding LanceDB entirely from SQLite.
8. Project isolation:
   project A results must never leak into project B.
9. Entity type filtering.
10. Duplicate-question candidate retrieval.
11. Answer retrieval for semantically equivalent questions.
12. Research evidence retrieval.
13. Contradiction candidate discovery.
14. Stale index detection.
15. Embedding dimension/schema validation.
16. Context top-K limits.
17. Semantic retrieval disabled mode.
18. Application operation when LanceDB is unavailable.

Use fake/deterministic embedding providers for most tests.

Unit and integration tests should not require paid embedding
API access.

==================================================
SEMANTIC RETRIEVAL OBSERVABILITY
==================================================

Record retrieval behavior for evaluation.

For each semantic query, log structured metadata such as:

- operation
- query purpose
- project ID
- entity types searched
- top_k requested
- number of candidates returned
- selected entity IDs
- similarity/distance values where appropriate
- records discarded as stale
- records actually supplied to the agent
- retrieval duration

Do not log private credentials.

Allow verbose retrieval inspection through a development flag.

Example:

project-init --trace-retrieval ...

This is important for representative agent trajectories and
for understanding whether semantic retrieval actually helped.