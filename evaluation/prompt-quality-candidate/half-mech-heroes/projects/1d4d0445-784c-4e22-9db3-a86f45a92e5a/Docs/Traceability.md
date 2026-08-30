# Traceability and Decision Register

This file is the canonical consequential-decision register and the explicit semantic trace graph. Each edge names one source and one target; comma-separated ID lists are not used as substitutes for trace edges. Evidence constrains a decision but never supplies stakeholder authority.

## Consequential decisions

| Decision ID | One subject | Status | Provenance type | Exact source IDs | Relevant evidence IDs | Rationale |
|---|---|---|---|---|---|---|
| DEC-001 | Project name | Accepted | Stakeholder brief | SRC-001 | None | Preserve declared identity exactly. |
| DEC-002 | Mandatory play mode | Accepted | Stakeholder brief | SRC-002 | EVD-MKT-001 | The brief requires local simultaneous two-player play; platform terminology only corroborates capability distinctness. |
| DEC-003 | Player embodiment | Accepted | Stakeholder brief | SRC-003 | None | Both players must act through one mech. |
| DEC-004 | Role control topology | Accepted | Stakeholder brief | SRC-004; SRC-005 | None | Movement/jump and arm/shield are deliberately split between player assignments. |
| DEC-005 | Role-swap trigger | Accepted | Stakeholder brief | SRC-006 | None | Valid mech damage is the authoritative automatic-swap trigger. |
| DEC-006 | Zone structure | Accepted | Stakeholder brief | SRC-007 | None | Each attempt uses one scrolling disaster zone. |
| DEC-007 | Civilian rescue interaction | Accepted | Stakeholder brief | SRC-008 | None | Catching falling civilians is explicitly required. |
| DEC-008 | Wreckage interaction | Accepted | Stakeholder brief | SRC-009 | None | Throwing wreckage clear is explicitly required. |
| DEC-009 | Teamwork multiplier | Accepted | Stakeholder brief | SRC-010 | None | Coordinated play must build a multiplier before timeout. |
| DEC-010 | Attempt duration | Accepted | Stakeholder brief | SRC-011 | None | “Three-minute” supplies an exact active-time duration of 180 seconds. |
| DEC-011 | Restart intent | Accepted | Stakeholder brief | SRC-012 | None | Rapid repeat attempts are required; a quantitative threshold is separate. |
| DEC-012 | Physics style | Accepted | Stakeholder brief | SRC-013 | None | The brief explicitly requires exaggerated physics. |
| DEC-013 | Silhouette style | Accepted | Stakeholder brief | SRC-014 | EVD-EXP-004 | Chunky silhouettes are required; contrast guidance constrains evaluation only. |
| DEC-014 | Mechanical-failure tone | Accepted | Stakeholder brief | SRC-015 | None | Failures must support comedy. |
| DEC-015 | Television-readable presentation | Accepted | Stakeholder brief | SRC-015 | EVD-EXP-001; EVD-EXP-005 | Television readability is authoritative; sources constrain a provisional evaluation context. |
| DEC-016 | Mandatory input configurations | Accepted | Stakeholder brief | SRC-016 | None | Two controllers and one shared keyboard are the two required configurations. |
| DEC-017 | City-block content count | Accepted | Stakeholder brief | SRC-017 | None | The first prototype contains one city-block unit. |
| DEC-018 | Civilian gameplay-type count | Accepted | Stakeholder brief | SRC-018 | None | The first prototype contains one civilian archetype. |
| DEC-019 | Hazard gameplay-type count | Accepted | Stakeholder brief | SRC-019 | None | The first prototype contains exactly three mechanically distinct hazards. |
| DEC-020 | Coordination as outcome | Accepted | Stakeholder brief | SRC-020 | None | Coordination is an explicit desired experience. |
| DEC-021 | Mastery as outcome | Accepted | Stakeholder brief | SRC-020 | None | Mastery is an explicit desired experience. |
| DEC-022 | Comedy as outcome | Accepted | Stakeholder brief | SRC-020 | None | Comedy is an explicit desired experience. |
| DEC-023 | Authority for declared open choices | Accepted | Stakeholder brief | SRC-021 | EVD-EXP-006 | Remapping, solo play, scaling, and age remain stakeholder-open; advisory evidence cannot close them. |
| DEC-024 | City-block/disaster-zone boundary relationship | Deferred | Unresolved project finding | UNK-009; ANS-004 | EVD-DEL-002 | Neither source wording nor geographic terminology selects equality of boundaries. |
| DEC-025 | Civilian creative identity | Deferred | Unresolved project finding | UNK-008; ANS-010 | EVD-EXP-004; EVD-EXP-005 | Presentation guidance cannot choose a civilian identity. |
| DEC-026 | Three hazard creative identities | Deferred | Unresolved project finding | UNK-007; ANS-009 | EVD-EXP-004; EVD-EXP-005 | Cue guidance cannot choose hazard identities. |
| DEC-027 | Primary target platform | Deferred | Unresolved project finding | UNK-005; ANS-002 | EVD-MKT-002; EVD-MKT-003; EVD-TEC-003 | Platform prerequisites constrain feasibility but do not express stakeholder preference. |
| DEC-028 | Implementation technology | Deferred | Unresolved project finding | UNK-006; ANS-003 | EVD-TEC-001; EVD-TEC-002; EVD-TEC-003 | Multiple technologies have relevant capabilities and platform choice affects feasibility. |
| DEC-029 | Input-remapping scope | Deferred | Stakeholder-declared open choice | SRC-021; ANS-006 | EVD-EXP-002; EVD-EXP-006 | Accessibility benefit does not authorise prototype scope. |
| DEC-030 | Solo-play scope | Deferred | Stakeholder-declared open choice | SRC-021; ANS-001 | EVD-MKT-001; EVD-MKT-004 | Local and solo capabilities are distinct; inclusion remains stakeholder-owned. |
| DEC-031 | Difficulty-scaling scope | Deferred | Stakeholder-declared open choice | SRC-021; ANS-007 | EVD-EXP-003; EVD-EXP-006 | Guidance cannot select scaling scope or model. |
| DEC-032 | Exact target age | Deferred | Stakeholder-declared open choice | SRC-021; ANS-008 | EVD-AUD-001; EVD-AUD-002 | Ratings evidence cannot select intended audience capability. |
| DEC-033 | Explicit prototype exclusions | Deferred | Unresolved project finding | UNK-011; ANS-011 | EVD-DEL-003 | Narrow-prototype guidance does not identify which project items to exclude. |
| DEC-034 | Quick-restart timing gate | Provisional | Design inference | SRC-012; ASM-002 | None | A measurable build gate is needed, but 5.0 seconds and hardware context lack stakeholder approval. |
| DEC-035 | Television-readability test gate | Provisional | Research-constrained inference | SRC-015; ASM-001; ASM-006 | EVD-EXP-001; EVD-DEL-001 | A reproducible context is needed; display, distance, cohort, and pass rate remain unapproved. |
| DEC-036 | Exaggerated-physics test gate | Provisional | Design inference | SRC-013; ASM-003 | None | Observable movement bounds enable testing without claiming approved tuning. |
| DEC-037 | Silhouette-recognition test gate | Provisional | Research-constrained inference | SRC-014; ASM-004 | EVD-EXP-004 | A blinded recognition check operationalises the intent, but cohort and threshold remain unapproved. |
| DEC-038 | Comedy test gate | Provisional | Evaluation inference | SRC-015; SRC-020; ASM-005 | EVD-DEL-001 | A prototype signal is useful, but humour definition, cohort, and threshold remain stakeholder-owned. |
| DEC-039 | Mastery test gate | Provisional | Evaluation inference | SRC-020; ASM-008 | EVD-EXP-003; EVD-DEL-001 | Attempt-to-attempt improvement is only a working proxy for mastery. |
| DEC-040 | Coordination test gate | Provisional | Evaluation inference | SRC-020; ASM-007 | EVD-DEL-001 | Coordinated-versus-control outcome is only a working proxy until effect and cohort are approved. |
| DEC-041 | Engine-agnostic architecture boundary | Provisional | Technical inference | OQ-005; OQ-006; ASM-009 | EVD-TEC-001; EVD-TEC-002; EVD-TEC-003 | Abstract ports avoid inventing a stack while preserving testable domain boundaries. |

## Exact source-clause → requirement edges

| Source | Requirement | Relationship and semantic relevance |
|---|---|---|
| SRC-001 | REQ-001 | Declares the exact project name. |
| SRC-002 | REQ-002 | Declares local two-player couch arcade play. |
| SRC-003 | REQ-003 | Declares one shared malfunctioning mech. |
| SRC-004 | REQ-004 | Assigns movement and jump to the left-side role. |
| SRC-005 | REQ-005 | Assigns arm and shield aiming to the right-side role. |
| SRC-006 | REQ-006 | Defines damage as automatic role-swap trigger. |
| SRC-007 | REQ-007 | Defines the single scrolling zone. |
| SRC-008 | REQ-008 | Defines falling-civilian catching. |
| SRC-009 | REQ-009 | Defines wreckage throwing. |
| SRC-010 | REQ-010 | Defines teamwork-multiplier growth before timeout. |
| SRC-011 | REQ-011 | Defines three-minute attempts. |
| SRC-012 | REQ-012 | Defines quick-restart intent. |
| SRC-013 | REQ-013 | Defines exaggerated-physics intent. |
| SRC-014 | REQ-014 | Defines chunky-silhouette intent. |
| SRC-015 | REQ-015 | Defines funny mechanical failures. |
| SRC-015 | REQ-016 | Defines the television-readability condition. |
| SRC-016 | REQ-017 | Defines both mandatory input configurations. |
| SRC-017 | REQ-018 | Sets city-block content count. |
| SRC-018 | REQ-019 | Sets civilian-type content count. |
| SRC-019 | REQ-020 | Sets hazard-type content count. |
| SRC-020 | REQ-021 | Declares coordination as an outcome. |
| SRC-020 | REQ-022 | Declares mastery as an outcome. |
| SRC-020 | REQ-023 | Declares comedy as an outcome. |
| SRC-021 | REQ-024 | Declares four choices open and protects their authority status. |

## Requirement → decision or constraint edges

| Requirement | Decision / constraint | Relationship and semantic relevance |
|---|---|---|
| REQ-001 | DEC-001 | Name decision governs title acceptance. |
| REQ-002 | DEC-002 | Mandatory-mode decision governs local session acceptance. |
| REQ-003 | DEC-003 | Embodiment decision governs entity count and ownership. |
| REQ-004 | DEC-004 | Control-topology decision governs movement-role isolation. |
| REQ-005 | DEC-004 | Control-topology decision governs tool-role isolation. |
| REQ-006 | DEC-005 | Swap-trigger decision governs damage transition behaviour. |
| REQ-007 | DEC-006 | Zone-structure decision governs attempt lifecycle. |
| REQ-007 | DEC-024 | Deferred boundary prevents an invented physical extent. |
| REQ-008 | DEC-007 | Rescue-interaction decision governs catch outcome. |
| REQ-009 | DEC-008 | Wreckage-interaction decision governs clearance outcome. |
| REQ-010 | DEC-009 | Multiplier decision governs coordinated score state. |
| REQ-011 | DEC-010 | Duration decision supplies the exact 180-second gate. |
| REQ-012 | DEC-011 | Restart-intent decision establishes repeat-attempt behaviour. |
| REQ-012 | DEC-034 | Provisional timing decision supplies the temporary measurable threshold. |
| REQ-013 | DEC-012 | Physics-style decision establishes the qualitative behaviour. |
| REQ-013 | DEC-036 | Provisional physics decision supplies temporary observable bounds. |
| REQ-014 | DEC-013 | Silhouette-style decision establishes visual intent. |
| REQ-014 | DEC-037 | Provisional recognition decision supplies the temporary evaluation gate. |
| REQ-015 | DEC-014 | Failure-tone decision establishes humorous malfunction intent. |
| REQ-015 | DEC-038 | Provisional comedy decision supplies the temporary observation gate. |
| REQ-016 | DEC-015 | Television-presentation decision establishes readability intent. |
| REQ-016 | DEC-035 | Provisional readability decision supplies the temporary context and threshold. |
| REQ-017 | DEC-016 | Input-configuration decision governs both required test executions. |
| REQ-018 | DEC-017 | City-block count decision governs manifest count. |
| REQ-018 | DEC-024 | Deferred boundary prevents conflating authored block and logical zone. |
| REQ-019 | DEC-018 | Civilian-count decision governs archetype count. |
| REQ-019 | DEC-025 | Deferred identity prevents invented civilian art or behaviour. |
| REQ-020 | DEC-019 | Hazard-count decision governs archetype count. |
| REQ-020 | DEC-026 | Deferred identities prevent invented hazard selections. |
| REQ-021 | DEC-020 | Coordination-outcome decision establishes desired effect. |
| REQ-021 | DEC-040 | Provisional coordination decision supplies the temporary comparison gate. |
| REQ-022 | DEC-021 | Mastery-outcome decision establishes desired learning effect. |
| REQ-022 | DEC-039 | Provisional mastery decision supplies the temporary progression gate. |
| REQ-023 | DEC-022 | Comedy-outcome decision establishes desired amusement effect. |
| REQ-023 | DEC-038 | Provisional comedy decision supplies the temporary observation gate. |
| REQ-024 | DEC-023 | Authority decision prevents research from closing stakeholder-open choices. |

## Evidence → decision edges

| Evidence | Decision | Relationship and semantic relevance |
|---|---|---|
| EVD-MKT-001 | DEC-002 | Corroborates local multiplayer as a distinct platform capability. |
| EVD-EXP-004 | DEC-013 | Constrains silhouette evaluation through background distinguishability. |
| EVD-EXP-001 | DEC-015 | Constrains television-readable text design. |
| EVD-EXP-005 | DEC-015 | Constrains critical state to redundant cue channels. |
| EVD-EXP-006 | DEC-023 | Establishes that accessibility guidance is advisory. |
| EVD-DEL-002 | DEC-024 | Shows geographic “block” terminology cannot supply a fixed extent. |
| EVD-EXP-004 | DEC-025 | Constrains eventual civilian presentation without choosing identity. |
| EVD-EXP-005 | DEC-025 | Constrains eventual civilian cues without choosing identity. |
| EVD-EXP-004 | DEC-026 | Constrains eventual hazard presentation without choosing identities. |
| EVD-EXP-005 | DEC-026 | Constrains eventual hazard cues without choosing identities. |
| EVD-MKT-002 | DEC-027 | Identifies Xbox-specific onboarding constraint. |
| EVD-MKT-003 | DEC-027 | Identifies Nintendo-specific onboarding constraint. |
| EVD-TEC-003 | DEC-027 | Shows platform selection can affect engine console-support feasibility. |
| EVD-TEC-001 | DEC-028 | Shows one candidate technology supports relevant input. |
| EVD-TEC-002 | DEC-028 | Shows another candidate technology supports relevant input. |
| EVD-TEC-003 | DEC-028 | Constrains technology choice by console-support model. |
| EVD-EXP-002 | DEC-029 | Supplies nonbinding remapping benefit and breadth guidance. |
| EVD-EXP-006 | DEC-029 | Prevents advisory guidance from becoming scope authority. |
| EVD-MKT-001 | DEC-030 | Establishes local capability without implying solo capability. |
| EVD-MKT-004 | DEC-030 | Establishes single-player as a distinct capability label. |
| EVD-EXP-003 | DEC-031 | Constrains difficulty discussion without selecting a model. |
| EVD-EXP-006 | DEC-031 | Prevents guidance from becoming scope authority. |
| EVD-AUD-001 | DEC-032 | Prevents content rating from becoming an audience choice. |
| EVD-AUD-002 | DEC-032 | Separates age suitability from gameplay difficulty. |
| EVD-DEL-003 | DEC-033 | Supports narrow experimentation without selecting exclusions. |
| EVD-EXP-001 | DEC-035 | Supplies a provisional 1080p text floor and context reminder. |
| EVD-DEL-001 | DEC-035 | Requires evaluation to remain context-specific. |
| EVD-EXP-004 | DEC-037 | Supports distinguishability as part of silhouette evaluation. |
| EVD-DEL-001 | DEC-038 | Supports explicit use context without supplying a comedy threshold. |
| EVD-EXP-003 | DEC-039 | Supports cohort-sensitive interpretation of skill barriers. |
| EVD-DEL-001 | DEC-039 | Supports context-specific evaluation without supplying a mastery threshold. |
| EVD-DEL-001 | DEC-040 | Supports context-specific evaluation without supplying a coordination threshold. |
| EVD-TEC-001 | DEC-041 | Demonstrates why an abstract input port can accommodate one candidate. |
| EVD-TEC-002 | DEC-041 | Demonstrates why an abstract input port can accommodate another candidate. |
| EVD-TEC-003 | DEC-041 | Justifies deferring vendor-specific console integration. |

## Deferred decision → open-question edges

| Decision | Open question | Relationship and semantic relevance |
|---|---|---|
| DEC-024 | OQ-009 | Boundary decision requires explicit stakeholder answer. |
| DEC-025 | OQ-008 | Civilian identity requires creative authority. |
| DEC-026 | OQ-007 | Hazard identities require creative authority. |
| DEC-027 | OQ-005 | Platform decision requires stakeholder selection. |
| DEC-028 | OQ-006 | Technology decision requires stakeholder selection. |
| DEC-029 | OQ-001 | Remapping scope requires stakeholder selection. |
| DEC-030 | OQ-002 | Solo scope requires stakeholder selection. |
| DEC-031 | OQ-003 | Difficulty scope requires stakeholder selection. |
| DEC-032 | OQ-004 | Target age requires stakeholder selection. |
| DEC-033 | OQ-011A | Feature exclusions require explicit decisions. |
| DEC-033 | OQ-011B | Platform exclusions require explicit decisions. |
| DEC-033 | OQ-011C | Quality exclusions require explicit decisions. |
| DEC-033 | OQ-011D | Deliverable exclusions require explicit decisions. |
| DEC-034 | OQ-010E | Restart threshold remains provisional until approved. |
| DEC-035 | OQ-010D | Readability context and gate remain provisional until approved. |
| DEC-038 | OQ-010B | Comedy gate remains provisional until approved. |
| DEC-039 | OQ-010A | Mastery gate remains provisional until approved. |
| DEC-040 | OQ-010C | Coordination gate remains provisional until approved. |

## Coverage summary

- Every material source clause in the canonical ledger has at least one explicit edge to a generated requirement.
- Every requirement has an explicit edge to a semantically relevant accepted, deferred, or provisional decision.
- Every retained evidence record has at least one explicit edge to a consequential decision.
- Imported Q-010 was split into OQ-010A, OQ-010B, OQ-010C, OQ-010D, and OQ-010E.
- Imported Q-011 was split into OQ-011A, OQ-011B, OQ-011C, and OQ-011D.
