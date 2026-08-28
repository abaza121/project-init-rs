---
name: project-initiation-pipeline
description: Turn a short project brief into a staged, internally consistent Docs foundation containing requirements, SWOT, mission and vision, visual identity, naming and brand prompts, technical architecture, focused research, and a timestamped decision log. Use when starting or bootstrapping a software, game, product, or creative project; when asked to initiate a project, create its foundational documentation, or run the project documentation pipeline; or when an existing project needs its early planning documents regenerated or aligned.
---

# Project Initiation Pipeline

Build a practical project foundation from a brief. Produce the documents in dependency order so later artifacts explicitly build on earlier decisions.

## Choose the execution mode

Use native skill mode by default:

1. Work directly in the target project.
2. Create or update the required documents yourself.
3. Do not launch another Codex process merely because the bundled runner exists.

Use runner mode only when the user explicitly asks to run the existing automation, reproduce the command-line pipeline, or test the runner. Resolve the directory containing this `SKILL.md`, then invoke:

```powershell
powershell.exe -NoProfile -ExecutionPolicy Bypass -File "<skill-root>\scripts\run-codex-docs-pipeline.ps1" -RootDir "<target-project-root>" -PromptFile "<project-brief-file>"
```

The runner requires Windows PowerShell and a `codex` executable on `PATH`, or a path supplied through `-CodexBin`. It starts an ephemeral nested Codex run and writes its last message to `Docs\_pipeline-last-message.txt`.

## Establish the run context

1. Resolve the exact target project root. Use the current workspace unless the user names another directory.
2. Obtain the project brief from the user's message or a supplied text file. Ask only when no usable project intent is available.
3. Inspect existing project files and `Docs` before writing. Preserve unrelated work and adapt existing documents instead of blindly replacing them.
4. Use the newest root-level `log-*.md` as a structural session-log reference when one exists. Do not copy its project-specific facts.
5. Record material assumptions when the brief leaves consequential choices open. Prefer a sensible working decision over blocking on low-risk details.

## Build the documents in order

Create `Docs` if necessary, then complete each stage before using it as an input to the next one.

### 1. Documentation index

Create `Docs/README.md` as the entry point. Explain the artifact sequence, summarize the project, link the documents, and tell future contributors where to begin.

### 2. Requirements

Create `Docs/requirements.md`. Cover goals, audiences, user journeys, functional and non-functional requirements, constraints, assumptions, out-of-scope items, risks, and measurable acceptance criteria. Separate confirmed requirements from inferred ones.

### 3. Strategic framing

Create `Docs/SWOT.md` from the requirements and relevant external context. Make each strength, weakness, opportunity, and threat specific enough to affect a decision.

Create `Docs/MissionVision.md` from the requirements and SWOT. Include mission, vision, brand or product pillars, audience promise, and decision principles that can guide design and implementation.

### 4. Identity and naming

Create `Docs/VisualIdentity.md` from the mission, vision, audience, and product experience. Define design principles, mood, color direction, typography direction, imagery, motion where relevant, accessibility considerations, and usage guidance.

Create `Docs/BrandPrompt.md` from the visual identity. Include viable names, the selected working name with rationale, naming constraints, palette-generation prompts, and logo-generation prompts. Treat generated names as candidates until the user confirms one.

### 5. Technical architecture

Create `Docs/TechnicalArchitecture.md` from the requirements and project constraints. Cover system boundaries, major components, data flow, technology choices, interfaces, security and privacy, performance, maintainability, scalability, testing, deployment, observability, risks, and staged delivery. Avoid inventing infrastructure the project does not need.

### 6. Focused research

Create at least five project-specific files named `Docs/Research-01-<topic>.md` through `Docs/Research-05-<topic>.md`. Choose topics that resolve the highest-impact uncertainty in the requirements, experience, market, technology, or delivery plan.

Browse for current, niche, or uncertain facts when internet access is available and the user has not prohibited research. Prefer primary sources, cite direct links near supported claims, distinguish evidence from inference, and record access dates for time-sensitive material. If browsing is unavailable, label unverified assumptions instead of presenting them as facts.

### 7. Session record

Create `Docs/log-yyyy-MM-dd-HH-mm-ss.md` using the current local timestamp. Include these sections:

- Notes
- User Prompt Log
- Planning Decisions Captured During The Session
- Resulting Session Artifacts
- Summary

Record the actual brief, meaningful follow-up instructions, consequential assumptions, chosen research topics, working-name decision, and files produced. Do not fabricate a conversation that did not occur.

## Maintain coherence

- Reuse the same audience, scope, terminology, and working name across documents.
- Let later documents cite or link earlier ones when the relationship matters.
- Resolve contradictions before completion; do not leave multiple incompatible decisions silently in place.
- Keep Markdown concise, actionable, and appropriate to the project's maturity.
- Use diagrams or tables only when they clarify relationships that prose would obscure.
- Do not add implementation code unless the user separately requests it.

## Verify completion

Before handing off:

1. Confirm every required core document exists and at least five numbered research files exist.
2. Check that the index links the generated artifacts and that referenced paths are valid.
3. Search for unresolved placeholders, copied facts from an unrelated log template, and conflicting project names or requirements.
4. Confirm the architecture traces back to the requirements and the identity traces back to the mission and vision.
5. Summarize created and updated files, key assumptions, research limitations, and decisions still requiring user confirmation.

