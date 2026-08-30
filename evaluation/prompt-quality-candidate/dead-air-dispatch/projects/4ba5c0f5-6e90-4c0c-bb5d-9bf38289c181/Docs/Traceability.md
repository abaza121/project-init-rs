# Traceability and Decision Register

Status: initiation baseline candidate  
Canonical decisions: this file.  
Canonical requirements: [Requirements.md](Requirements.md).  
Canonical assumptions: [Assumptions.md](Assumptions.md).  
Canonical open questions: [OpenQuestions.md](OpenQuestions.md).

## Provenance types

- **Stakeholder brief**: authority supplied directly in `SRC-*`.
- **Imported answer boundary**: an `ANS-*` record says that authority is unavailable; it cannot authorize a product choice.
- **External evidence**: a primary source constrains reasoning but has no project authority.
- **Design inference**: a reversible proposal made for planning; it is non-authoritative until approved.
- **Process contract**: package-creation rules in the current request.

## Constraint register

| ID | Constraint | Status | Exact source IDs | Rationale |
|---|---|---|---|---|
| `C-001` | Product form is a strange 2D score-attack game centered on an emergency radio in a supernatural storm. | authoritative | `SRC-002` | Direct product premise. |
| `C-002` | Target platform is PC; OS and hardware baseline are not specified. | authoritative with open detail | `SRC-002`; `ANS-004` | PC is explicit, while audience and detailed operating context are not. |
| `C-003` | Mouse-wheel tuning gates arena revelation and changes information and entity response by frequency. | authoritative | `SRC-003` | Direct mechanic. |
| `C-004` | Repeating order is scan, identify, hold to guide, then retune under interference pressure. | authoritative | `SRC-004` | Direct loop. |
| `C-005` | Score must use rescues, signal accuracy, and risky time; relative weights are unauthorized. | authoritative with open detail | `SRC-005`; `ANS-003` | Inputs are explicit; weights are stakeholder-owned. |
| `C-006` | Run target is about five minutes; acceptable lower and upper bounds are unauthorized. | authoritative with open detail | `SRC-005`; `ANS-005` | Target is explicit; tolerance is open. |
| `C-007` | The listed abstract presentation and single-room form are allowed for the first playable, not mandated final art. | authorized permission | `SRC-006` | Preserves “can use.” |
| `C-008` | Hidden-information fairness is a high-impact risk; no evaluation measure or threshold is authorized. | authoritative risk with open detail | `SRC-007`; `ANS-002` | Prevents a fabricated acceptance bar. |
| `C-009` | Deaf and hard-of-hearing support is undecided. | authoritative open constraint | `SRC-007`; `ANS-001` | Recommendations cannot settle scope. |
| `C-010` | Intended player audience is not specified. | authoritative open constraint | `ANS-004` | Platform and rating do not choose an audience. |
| `C-011` | No project exclusions are stakeholder-authorized. | authoritative open constraint | `ANS-006` | Avoids inferred out-of-scope declarations. |

## Consequential decision register

Each record covers one subject. `Evidence IDs: none` means the decision rests only on stakeholder authority or is explicitly an inference without a retained external research claim.

### D-001 — Canonical product name

- Status: approved
- Subject: product name
- Decision: Use “Dead Air Dispatch” as the canonical project identity.
- Provenance type: stakeholder brief
- Exact source IDs: `SRC-001`
- Relevant evidence IDs: none
- Rationale: the brief states the name directly.

### D-002 — Core playable loop interpretation

- Status: approved at concept level
- Subject: functional loop
- Decision: Treat scan, caller-frequency identification, signal hold, rescue, retune, and hostile approach as the core state sequence, with frequency also controlling revealed information and entity hearing.
- Provenance type: stakeholder brief
- Exact source IDs: `SRC-003`; `SRC-004`
- Relevant evidence IDs: none
- Rationale: this is a direct decomposition of the two mechanic clauses without selecting tuning values.

### D-003 — First-playable art permission

- Status: authorized option
- Subject: prototype presentation boundary
- Decision: Abstract waveforms, silhouettes, static, layered sound, and a single room are permitted for the first playable; they are not mandatory final-product identity.
- Provenance type: stakeholder brief
- Exact source IDs: `SRC-006`
- Relevant evidence IDs: none
- Rationale: preserves the permissive wording “can use.”

### D-004 — Mission framing

- Status: proposed; non-authoritative
- Subject: internal mission statement
- Decision: Use the proposed mission in [MissionVision.md](MissionVision.md) for alignment until stakeholders approve or replace it.
- Provenance type: design inference
- Exact source IDs: `SRC-002`; `SRC-003`; `SRC-004`; `SRC-005`; `SRC-007`
- Relevant evidence IDs: none
- Rationale: condenses the supplied experience, loop, score, and risk into an internal planning statement without changing requirements.

### D-005 — Visual identity direction

- Status: proposed; non-authoritative
- Subject: visual identity
- Decision: Explore an instrument-panel/nocturnal-signal identity built from waveforms, silhouettes, restrained static, and high-legibility overlays.
- Provenance type: design inference
- Exact source IDs: `SRC-003`; `SRC-006`; `SRC-007`
- Relevant evidence IDs: `EVD-001`; `EVD-002`
- Rationale: uses explicitly permitted motifs; `EVD-001` and `EVD-002` inform, but do not authorize, the proposed legibility treatment.

### D-006 — Engine-agnostic initiation architecture

- Status: proposed; reversible; non-authoritative
- Subject: engine commitment
- Decision: Keep the initiation architecture engine-agnostic until stakeholders answer `OQ-007`.
- Provenance type: design inference
- Exact source IDs: `SRC-002`; `SRC-003`; `SRC-004`; `SRC-005`; `SRC-006`
- Relevant evidence IDs: none
- Rationale: the brief defines behaviors but supplies no engine, language, budget, team, or toolchain authority.

### D-007 — Deaf and hard-of-hearing scope

- Status: open; no selection authorized
- Subject: adopted deaf and hard-of-hearing features
- Decision: No captions, equivalent visual cues, audio-category controls, mono output, or other feature is yet committed.
- Provenance type: stakeholder brief plus imported answer boundary
- Exact source IDs: `SRC-007`; `ANS-001`
- Relevant evidence IDs: `EVD-001`; `EVD-002`; `EVD-003`
- Rationale: evidence supports options but does not provide project authority.

### D-008 — Hidden-information acceptance threshold

- Status: open; no threshold authorized
- Subject: fairness acceptance rule
- Decision: Do not declare hidden information “intriguing rather than unfair” until intended population, measures, and acceptable results are approved.
- Provenance type: stakeholder brief plus imported answer boundary
- Exact source IDs: `SRC-007`; `ANS-002`
- Relevant evidence IDs: `EVD-004`; `EVD-005`; `EVD-006`
- Rationale: the risk is explicit; `EVD-004`, `EVD-005`, and `EVD-006` constrain the evaluation decision but supply no project authority or cutoff.

### D-009 — Score weights

- Status: open; no weights authorized
- Subject: relative score weighting
- Decision: Preserve three independently observable score components and leave their weights unset until stakeholder approval.
- Provenance type: stakeholder brief plus imported answer boundary
- Exact source IDs: `SRC-005`; `ANS-003`
- Relevant evidence IDs: `EVD-007`
- Rationale: `EVD-007` is relevant to the authority boundary, but external methodology cannot choose project values.

### D-010 — Intended audience

- Status: open; no audience authorized
- Subject: target player population
- Decision: Do not infer an age, skill, demographic, or accessibility audience from PC platform, genre, mechanics, or a future content rating.
- Provenance type: imported answer boundary
- Exact source IDs: `ANS-004`
- Relevant evidence IDs: `EVD-008`; `EVD-009`
- Rationale: `EVD-008` and `EVD-009` constrain audience inference but do not authorize a target population.

### D-011 — Run-duration tolerance

- Status: open; no tolerance authorized
- Subject: acceptable duration bounds
- Decision: Retain “about five minutes” and do not invent minimum, maximum, percentile, or plus/minus bounds.
- Provenance type: stakeholder brief plus imported answer boundary
- Exact source IDs: `SRC-005`; `ANS-005`
- Relevant evidence IDs: `EVD-004`
- Rationale: `EVD-004` constrains acceptance-setting, while the project sources provide no range.

### D-012 — Project exclusions

- Status: open; no exclusions authorized
- Subject: excluded scope
- Decision: Record “none authorized” until interested parties agree explicit exclusions.
- Provenance type: imported answer boundary
- Exact source IDs: `ANS-006`
- Relevant evidence IDs: `EVD-011`; `EVD-012`
- Rationale: `EVD-011` and `EVD-012` support an approval gate but cannot identify this project’s exclusions.

### D-013 — Nominal duration representation

- Status: proposed interpretation; non-authoritative tolerance
- Subject: nominal timer value
- Decision: Represent five minutes as a 300-second nominal configuration value while leaving acceptable completion bounds open.
- Provenance type: design inference
- Exact source IDs: `SRC-005`; `ANS-005`
- Relevant evidence IDs: `EVD-004`
- Rationale: unit conversion makes the stated target testable without inventing tolerance.

### D-014 — Behavioral/presentation separation

- Status: proposed; reversible; non-authoritative
- Subject: component boundary
- Decision: Separate deterministic run rules and signal/threat state from presentation and audio adapters.
- Provenance type: design inference
- Exact source IDs: `SRC-003`; `SRC-004`; `SRC-005`; `SRC-006`; `SRC-007`
- Relevant evidence IDs: none
- Rationale: the separation supports exact mechanic tests and permits presentation/accessibility iteration without silently changing game rules.

### D-015 — Accessibility discovery options

- Status: recommendation only; not adopted scope
- Subject: options to take to stakeholder review
- Decision: Present captions, equivalent visual channels for important audio, independently configurable audio categories, and mono output as candidates for `OQ-001`.
- Provenance type: external evidence plus design inference
- Exact source IDs: `SRC-007`; `ANS-001`
- Relevant evidence IDs: `EVD-001`; `EVD-002`; `EVD-003`
- Rationale: these are relevant evidence-backed options, but selection and acceptance remain stakeholder-owned.

## Requirement-to-source and decision edges

Every row is one exact edge. No ID ranges or slash-combined IDs are used.

| Requirement | Edge | Target | Relevance |
|---|---|---|---|
| `DAD-REQ-001` | derived from | `SRC-001` | Supplies the exact name. |
| `DAD-REQ-001` | governed by | `D-001` | Establishes canonical use. |
| `DAD-REQ-002` | derived from | `SRC-002` | Supplies 2D score-attack form. |
| `DAD-REQ-002` | constrained by | `C-001` | Preserves product form. |
| `DAD-REQ-003` | derived from | `SRC-002` | Supplies PC target. |
| `DAD-REQ-003` | constrained by | `C-002` | Separates PC authority from missing baseline. |
| `DAD-REQ-004` | derived from | `SRC-002` | Supplies operator fantasy and storm context. |
| `DAD-REQ-004` | governed by | `D-002` | Places premise in core playable interpretation. |
| `DAD-REQ-005` | derived from | `SRC-003` | Supplies invisibility, wheel input, and reveal. |
| `DAD-REQ-005` | constrained by | `C-003` | Preserves the tuning/reveal dependency. |
| `DAD-REQ-006` | derived from | `SRC-003` | Supplies all three reveal types. |
| `DAD-REQ-006` | governed by | `D-002` | Places reveal in core loop system. |
| `DAD-REQ-007` | derived from | `SRC-003` | Supplies hearing and approach changes. |
| `DAD-REQ-007` | constrained by | `C-003` | Preserves frequency-dependent response. |
| `DAD-REQ-008` | derived from | `SRC-004` | Supplies scan step. |
| `DAD-REQ-008` | governed by | `D-002` | Defines sequence role. |
| `DAD-REQ-009` | derived from | `SRC-004` | Supplies caller-frequency identification. |
| `DAD-REQ-009` | constrained by | `C-004` | Preserves loop order. |
| `DAD-REQ-010` | derived from | `SRC-004` | Supplies signal hold and guidance. |
| `DAD-REQ-010` | governed by | `D-002` | Defines state transition. |
| `DAD-REQ-011` | derived from | `SRC-004` | Supplies retune/interference race. |
| `DAD-REQ-011` | constrained by | `C-004` | Preserves hostile-pressure stage. |
| `DAD-REQ-012` | derived from | `SRC-005` | Supplies three score inputs. |
| `DAD-REQ-012` | bounded by | `ANS-003` | States that weights remain open. |
| `DAD-REQ-012` | governed by | `D-009` | Keeps components inspectable and weights unset. |
| `DAD-REQ-013` | derived from | `SRC-005` | Supplies approximate five-minute target. |
| `DAD-REQ-013` | bounded by | `ANS-005` | States that tolerance remains open. |
| `DAD-REQ-013` | governed by | `D-011` | Prevents invented bounds. |
| `DAD-REQ-013` | interpreted by | `D-013` | Supplies nominal 300-second representation. |
| `DAD-REQ-014` | derived from | `SRC-006` | Supplies permitted first-playable treatment. |
| `DAD-REQ-014` | governed by | `D-003` | Preserves permission rather than mandate. |
| `DAD-REQ-015` | derived from | `SRC-007` | Supplies central hidden-information risk. |
| `DAD-REQ-015` | bounded by | `ANS-002` | States no threshold is authorized. |
| `DAD-REQ-015` | informed by | `EVD-004` | Supports context-specific values. |
| `DAD-REQ-015` | informed by | `EVD-005` | Supports selecting an explicit evaluation method. |
| `DAD-REQ-015` | informed by | `EVD-006` | Supports player-specific difficulty evaluation. |
| `DAD-REQ-015` | governed by | `D-008` | Defines the authority gate. |
| `DAD-REQ-016` | derived from | `SRC-007` | Supplies unresolved accessibility scope. |
| `DAD-REQ-016` | bounded by | `ANS-001` | Prevents recommendations becoming commitments. |
| `DAD-REQ-016` | informed by | `EVD-001` | Supplies multi-channel option. |
| `DAD-REQ-016` | informed by | `EVD-002` | Supplies caption option. |
| `DAD-REQ-016` | informed by | `EVD-003` | Supplies audio-control options. |
| `DAD-REQ-016` | governed by | `D-007` | Keeps adoption open. |
| `DAD-REQ-017` | bounded by | `ANS-004` | States audience is unauthorized. |
| `DAD-REQ-017` | informed by | `EVD-008` | Establishes user groups as context. |
| `DAD-REQ-017` | informed by | `EVD-009` | Distinguishes rating from audience. |
| `DAD-REQ-017` | governed by | `D-010` | Prevents audience inference. |
| `DAD-REQ-018` | bounded by | `ANS-006` | States exclusions are unauthorized. |
| `DAD-REQ-018` | informed by | `EVD-011` | Supports interested-party agreement. |
| `DAD-REQ-018` | informed by | `EVD-012` | Supports stakeholder agreement and provisionality. |
| `DAD-REQ-018` | governed by | `D-012` | Keeps exclusions open. |

## Evidence-to-decision edges

| Evidence | Edge | Decision | Relevance |
|---|---|---|---|
| `EVD-001` | informs | `D-005` | Applies its cue-channel guidance to the proposed identity without granting scope. |
| `EVD-001` | informs | `D-007` | Constrains the feature discussion without selecting a feature. |
| `EVD-001` | informs | `D-015` | Supplies a candidate category for stakeholder review. |
| `EVD-002` | informs | `D-005` | Applies its text/cue guidance to the proposed identity without granting scope. |
| `EVD-002` | informs | `D-007` | Constrains the feature discussion without selecting a feature. |
| `EVD-002` | informs | `D-015` | Supplies a candidate category for stakeholder review. |
| `EVD-003` | informs | `D-007` | Constrains the audio-feature discussion without selecting a feature. |
| `EVD-003` | informs | `D-015` | Supplies candidate categories for stakeholder review. |
| `EVD-004` | informs | `D-008` | Constrains threshold selection without supplying a value. |
| `EVD-004` | informs | `D-011` | Constrains duration acceptance without supplying bounds. |
| `EVD-004` | informs | `D-013` | Separates nominal representation from acceptance bounds. |
| `EVD-005` | informs | `D-008` | Constrains method selection without supplying a method or cutoff. |
| `EVD-006` | informs | `D-008` | Constrains the population-specific evaluation decision. |
| `EVD-007` | informs | `D-009` | Constrains who may authorize relative weights. |
| `EVD-008` | informs | `D-010` | Constrains target-population inference. |
| `EVD-009` | informs | `D-010` | Constrains rating-based audience inference. |
| `EVD-011` | informs | `D-012` | Constrains how exclusions become authorized. |
| `EVD-012` | informs | `D-012` | Constrains how project scope becomes baselined. |

## Source coverage

| Material source | Mapped requirement IDs | Coverage note |
|---|---|---|
| `SRC-001` | `DAD-REQ-001` | Exact name retained. |
| `SRC-002` | `DAD-REQ-002`, `DAD-REQ-003`, `DAD-REQ-004` | Genre, dimensionality, score-attack form, platform, operator role, and storm context retained. |
| `SRC-003` | `DAD-REQ-005`, `DAD-REQ-006`, `DAD-REQ-007` | Invisibility, input, revelations, hearing, and approach retained. |
| `SRC-004` | `DAD-REQ-008`, `DAD-REQ-009`, `DAD-REQ-010`, `DAD-REQ-011` | Every loop stage retained. |
| `SRC-005` | `DAD-REQ-012`, `DAD-REQ-013` | Duration and each scoring dimension retained; open tuning values remain open. |
| `SRC-006` | `DAD-REQ-014` | Every permitted first-playable treatment retained without converting permission to mandate. |
| `SRC-007` | `DAD-REQ-015`, `DAD-REQ-016` | Central risk and accessibility unknown retained. |
