# Open Questions Register

This is the canonical register for unresolved choices. Snapshot questions marked “answered” remain open here when their imported answer explicitly says the evidence cannot decide or begins with `FAIL`.

| ID | Status | Owner | Decision needed | Why it matters | Unavailable authority / next action |
|---|---|---|---|---|---|
| OQ-001 | Open | Stakeholder | Should in-game input remapping be included in the first prototype, and if so for which inputs? | Affects accessibility, UI, persistence, tests, and schedule. | ANS-006 supplies guidance but no authority. Decide scope explicitly; see EVD-EXP-002. |
| OQ-002 | Open | Stakeholder | Should solo play be included in addition to mandatory local two-player play? | Changes control substitution, AI, onboarding, and scope. | ANS-001 cannot decide. No current project authority is available. |
| OQ-003 | Open | Stakeholder | Should difficulty scaling be included, and what experience should it preserve? | Changes tuning, UI, content parameters, and evaluation. | ANS-007 supplies guidance but no authority; see EVD-EXP-003. |
| OQ-004 | Open | Stakeholder | What exact target age or age range is intended? | Affects evaluation cohort, language, complexity, content, and ratings planning. | ANS-008 cannot decide; classification is not audience selection. See EVD-AUD-001 and EVD-AUD-002. |
| OQ-005 | Open | Stakeholder | What is the primary target platform? | Determines input APIs, performance budgets, packaging, certification, and hardware tests. | ANS-002 cannot decide. Platform prerequisites are evidence, not authority; see EVD-MKT-002 and EVD-MKT-003. |
| OQ-006 | Open | Stakeholder | Which engine, language, and framework should implement the prototype? | Determines runtime architecture, tooling, team needs, and platform feasibility. | ANS-003 cannot decide. Resolve after or together with OQ-005; see EVD-TEC-001, EVD-TEC-002, and EVD-TEC-003. |
| OQ-007 | Open | Stakeholder / creative lead | Which three hazard identities should fill the required hazard slots? | Required for art, behaviour, telegraphing, and tuning. | ANS-009 constrains communication only. No identity authority is available. |
| OQ-008 | Open | Stakeholder / creative lead | Which single civilian type should fill the required civilian slot? | Required for art, tone, animation, and rescue presentation. | ANS-010 cannot decide and incorrectly suggested the rescue mechanic was absent; SRC-008 already requires catching. |
| OQ-009 | Open | Stakeholder / level-design lead | Does the one city block constitute the entire single scrolling disaster zone? | Changes level length, content boundaries, camera design, and completion definition. | ANS-004 cannot decide; see EVD-DEL-002 only for why “block” lacks a standard extent. |
| OQ-010A | Open | Stakeholder / research lead | What observable definition, cohort, method, and pass threshold establish mastery? | Required to replace ASM-008 and approve REQ-022. | Imported Q-010 bundled this with unrelated outcomes; no threshold authority is available. |
| OQ-010B | Open | Stakeholder / research lead | What observable definition, cohort, method, and pass threshold establish comedy? | Required to replace ASM-005 and approve REQ-015 and REQ-023. | Imported Q-010 bundled this with unrelated outcomes; no threshold authority is available. |
| OQ-010C | Open | Stakeholder / research lead | What observable definition, cohort, method, and pass threshold establish coordination? | Required to replace ASM-007 and approve REQ-021. | Imported Q-010 bundled this with unrelated outcomes; no threshold authority is available. |
| OQ-010D | Open | Stakeholder / UX lead | What display, viewing distance, cohort, tasks, and pass threshold establish television readability? | Required to replace ASM-001 and ASM-006 and approve REQ-016. | EVD-EXP-001 informs a test but cannot approve project context or threshold. |
| OQ-010E | Open | Stakeholder / delivery lead | What device and elapsed-time threshold establish a quick restart? | Required to replace ASM-002 and approve the qualitative part of REQ-012. | No project-specific threshold authority is available. |
| OQ-011A | Open | Stakeholder | Which gameplay features, if any, are explicitly excluded from the first prototype? | Prevents accidental scope expansion while preserving authority. | ANS-011 correctly identifies no current exclusion; decide individually. |
| OQ-011B | Open | Stakeholder | Which platforms or distribution channels, if any, are explicitly excluded? | Constrains technical evaluation and packaging. | No current exclusion authority is available. |
| OQ-011C | Open | Stakeholder | Which quality attributes, if any, may be deferred from the first prototype? | Affects definition of done and technical debt. | No current exclusion authority is available. |
| OQ-011D | Open | Stakeholder | Which deliverables beyond a playable prototype, if any, are explicitly excluded? | Clarifies whether tooling, analytics, documentation, or release artifacts are expected. | No current exclusion authority is available. |

## Resolution protocol

A question closes only when a named stakeholder decision records one subject, an explicit answer, date, rationale, and affected IDs. Research may be cited as evidence but never as stakeholder authority. Rejected or failing research answers remain historical events in the session log, not resolutions.
