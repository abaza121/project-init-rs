Project init is meant as an experiment to understand how Rust and agent work, it provides a more complex implementation against a skill that already existed in my github here [abaza121/project-initiation-pipeline: A group of prompts used to initiate a project, to steer agents around same motivation.](https://github.com/abaza121/project-initiation-pipeline)

The idea was creating different elements that jumps start a project from a simple prompt to different artifacts that would be used by both humans and agents to steer the project direction.

1. From that data, the script builds one large instruction payload that tells Codex to create a staged documentation set in `Docs`, including:
    - `Docs\README.md`
    - `Docs\requirements.md`
    - `Docs\SWOT.md`
    - `Docs\MissionVision.md`
    - `Docs\VisualIdentity.md`
    - `Docs\BrandPrompt.md`
    - `Docs\TechnicalArchitecture.md`
    - `Docs\Research-01-<topic>.md` through `Docs\Research-05-<topic>.md`
    - `Docs\log-yyyy-MM-dd-HH-mm-ss.md`

The problem is that the user had no control into what kind of result it would be, this Rust based agent implements a more elaborate interface where the user get asked questions inferred from the prompt and then generates concise documents according to the answer, let's dive into how this works.
# The idea

Sometimes you have the thought of this idea but in order to start a project, you there is this gap between what you have in mind and what appears from the agents, or you want more information to think of other thoughts around it, I decided to use Rust because I wanted to get to know it more as part of this hackathon, I also decided on Rust because it is extremely efficient and because it has safeguards as part of its philosophy it will force coding agents on a better implementation.

Project Init is a harness that loads Codex to interact with models, it also have an implementation that integrates Mistral, here is how it works.

- Prompt the harness with a file that have the initial prompt.
- After the initial inference phase the harness digest it into different decisions and questions.
- The user can either answer the question one by one or just use auto answer to rely on a completely automated testing environment.
- In Auto answer there are 5 agents spawn to avoid contradiction 1 Coordinator 3 workers and 1 Judge.
- Then the output is put into the project folder beside the SQLite database of all the decisions.

# The implementation

The project uses Ratatui for the TUI along with Tokio for starting up the processes, it can work with Codex agent or a local mistral.rs integration using Docker.

The project contains multiple modules

## Storage

`storage` owns the SQLite boundary. `SqliteStore` opens the database, applies the embedded
migrations, enables foreign keys, and exposes transactional operations for projects, findings,
questions, answers, evidence, decisions, workflow runs, document revisions, and agent activity.
It converts SQLite rows into checked domain values and keeps SQL, schema details, and transaction
handling out of the rest of the application. SQLite is the authoritative source of truth: a
failed write rolls back the complete state transition rather than leaving a partially updated
project behind.

## Domain

`domain` defines the provider- and database-independent language of the application. It contains
typed project and finding identifiers; projects, questions, answers, requirements, trace links,
evidence, decisions, document revisions, and workflow status; plus the controlled vocabularies
and validation rules that govern their lifecycle. This layer has no dependency on SQLite, the
terminal UI, Docker, or a model provider, which makes its rules directly testable and prevents
infrastructure concerns from becoming business rules.

## Agents

`agents` provides provider-neutral contracts for analysis, cited research, automatic answering,
and document generation. It includes adapters for the Codex CLI and a local OpenAI-compatible
mistral.rs HTTP service. Both adapters validate structured responses, sanitize and bound activity
events, honour cancellation and time limits, and return the same typed results to the workflow.
The local runtime configuration also verifies model files and controls the loopback Docker service
used for local inference.

## Workflow

`workflow` contains the application use cases and the project state machine. `ProjectService`
turns validated analysis into authoritative project records, accepts answers and evidence,
manages approval and repair boundaries, and calculates the next safe action. `WorkflowRunner`
uses those services for both the CLI and the workbench, coordinating planning, generation,
validation, and the coordinator/three-worker/judge automatic-answer flow without allowing
provisional model output to bypass persistence rules.

## Documents

`documents` renders the generated Markdown package from a consistent `ProjectSnapshot`. Its
renderer produces the requirements, assumptions, open questions, SWOT, mission and vision,
visual identity, brand prompt, technical architecture, research lanes, traceability, session log,
and validation report. Rendering is deterministic when it works from stored knowledge, so the
generated documents remain traceable to the SQLite records that support them.

## TUI and CLI

`tui` implements the Ratatui workbench and creation progress view. It owns terminal setup,
keyboard handling, focus and composer state, and the presentation of provider activity; it sends
typed commands to the workflow instead of writing to SQLite itself. `main.rs` is the thin CLI
entry point: Clap parses commands and configuration, Tokio runs asynchronous provider work, and
the entry point composes the selected provider, storage, workflow, documents, and optional
interactive workbench.

# The Evaluation

In order to improve the prompts used by Project Init, I also implemented a separate evaluation. The evaluator is an independent, read-only Rust application: it grades a generated project-foundation package against its original brief, but never calls an LLM, changes the submitted package, or depends on network access. It uses deterministic extraction and keyword/token-overlap rules to make every score and deduction repeatable. Across the available cases in this repository's  [`evaluation`](../evaluation/) folder, the skill-based approach began at approximately 59 points;
the refined prompt packages score around 90 points. These results are comparison evidence, not a
claim that a higher score alone proves a better project outcome.

## Evaluation pipeline

The evaluator first validates the input paths, collects only supported UTF-8 project artifacts,
and processes them in a stable path order. It supports Markdown, plain text, JSON, TOML, and YAML;
skips unsupported files and symbolic links; and applies explicit per-file, package-size, artifact,
and discovery limits. It then extracts normalized requirements, decisions, assumptions,
constraints, answers, evidence claims, open questions, acceptance criteria, and explicit trace
links from the original brief, generated documents, and optional project-model or metadata JSON.

The extracted records feed deterministic metrics and the fixed 100-point Decision-Ready Project
Foundation Score (DRPFS). The rubric has seven dimensions: Brief Fidelity (20 points), Assumption
Discipline (15), Cross-Document Consistency (15), Evidence Quality (15), Traceability (15),
Actionability (15), and Artifact Quality (5). Scores are bounded and rounded to one decimal place.
Missing authoritative telemetry remains unavailable rather than being substituted with a made-up
zero or inferred value.

Each deduction is reported with a severity, criterion, source artifact, bounded evidence excerpt,
and recommended correction. The report can therefore surface concrete issues such as unsupported decisions, unlabelled assumptions, contradictory documents, missing acceptance criteria, traceability links that merely co-locate identifiers, decorative or repeated research, and
unresolved consequential questions. Keyword overlap only proposes candidate coverage; it is not
treated as semantic equivalence, proof of evidence, or a substitute for human review.



## Reports and comparison

Running `evaluate` produces both `validation-report.json` for structured analysis and
`validation-report.md` for a human-readable scorecard. Both reports are created only after a
successful evaluation, so an invalid input cannot leave a partial final report.

The `compare` command evaluates a baseline package and a candidate package independently using
the same original brief. It writes the per-package reports to separate directories and produces a
`comparison-report.md` that shows DRPFS, all seven dimensions, available quality metrics, and
resource differences side by side. This isolation prevents one package's files, metadata, or
score from influencing the other; a change in the comparison reflects two independently
reproducible evaluations rather than an LLM judging its own output.


