# Open Questions Register

Canonicality: this file is the only canonical unresolved-choice register.  
Metadata reconciliation: the imported pipeline marks `Q-001` through `Q-006` “answered” and their linked findings “resolved,” but every answer text says stakeholder authority is unavailable and the choice must remain open. The semantic answer text governs this package; pipeline status does not authorize a decision.

| Open ID | Imported ID | Question | Why it matters | Required decision owner | Blocks | Status |
|---|---|---|---|---|---|---|
| `OQ-001` | `Q-001` | Which deaf and hard-of-hearing support features will be adopted, for which milestone, and with what pass/fail checks? | Audio and signal perception are central to play. | stakeholder/product owner with accessibility input | `DAD-REQ-016` | open; high impact |
| `OQ-002` | `Q-002` | Who is the intended player population, including relevant age, PC familiarity, skill, and access-needs characteristics? | Audience is necessary for contextual usability and fairness evaluation. | stakeholder/product owner | `DAD-REQ-015`, `DAD-REQ-017` | open; high impact |
| `OQ-003` | `Q-005` | What are the relative weights, caps, and combination rule for rescues, signal accuracy, and risky time? | The values encode the intended score-attack priorities. | design owner approved by stakeholder | final acceptance of `DAD-REQ-012` | open; high impact |
| `OQ-004` | `Q-003` | Which measures and pass/fail threshold define “intriguing rather than unfair”? | It is the named central risk and has no universal threshold. | stakeholder with user-research owner | `DAD-REQ-015` | open; high impact |
| `OQ-005` | `Q-004` | What minimum and maximum observed run durations count as “about five minutes,” and over what sample? | The nominal target is known; acceptance bounds are not. | stakeholder/design owner | final acceptance of `DAD-REQ-013` | open; medium impact |
| `OQ-006` | `Q-006` | What work or product scope is explicitly excluded? | No exclusion is currently authorized. | interested stakeholders | scope baseline, `DAD-REQ-018` | open; medium impact |
| `OQ-007` | none | Which engine, language, renderer, audio stack, and licensing constraints are approved? | Determines implementation patterns, build process, and team needs. | technical owner with stakeholder budget authority | architecture commitment | open; high impact |
| `OQ-008` | none | What are the fail/end conditions when hostile interference reaches the antenna, and can the run end in other ways? | Required to complete the run state machine. | design owner | terminal-state implementation | open; high impact |
| `OQ-009` | none | Which PC operating systems, input-device variants, minimum hardware, display modes, and performance targets form the support baseline? | “PC” alone is insufficient for objective build acceptance. | product and technical owners | final acceptance of `DAD-REQ-003` | open; high impact |
| `OQ-010` | none | What tuning model is intended: continuous, stepped, banded, or hybrid; and how are acquisition and dangerous-band widths defined? | Central mechanics and test fixtures depend on it. | design owner | tuning implementation details | open; high impact |
| `OQ-011` | none | What exactly must the first playable contain, and is the single-room treatment selected or merely allowed? | `SRC-006` grants permission but does not define milestone acceptance. | stakeholder/product owner | delivery baseline | open; high impact |
| `OQ-012` | none | How many caller, hazard, route, and hostile archetypes are required for the first playable? | Content volume affects production and validation but is absent from the brief. | design and production owners | content plan | open; medium impact |
| `OQ-013` | none | Must tuning be operable when a mouse wheel is absent or difficult to use, and if so, which alternatives are required? | Mouse wheel is explicit, but alternative input scope is not. | stakeholder/product owner with accessibility input | input accessibility scope | open; high impact |
| `OQ-014` | none | Are score persistence, leaderboards, replays, analytics, and network services required? | “Score-attack” does not itself authorize storage or online systems. | stakeholder/product owner | data and service architecture | open; medium impact |
| `OQ-015` | none | What team, budget, schedule, review cadence, and approval roles apply? | Delivery feasibility cannot be estimated responsibly without them. | project sponsor | delivery plan | open; high impact |

## Evidence-backed options, not answers

- For `OQ-001`, `EVD-001`, `EVD-002`, and `EVD-003` support considering redundant sensory cues, subtitles/captions, configurable audio categories, and mono output. They do not select scope.
- For `OQ-004`, `EVD-004`, `EVD-005`, and `EVD-006` support context-specific evaluation. They do not supply a threshold.
- For `OQ-003`, `EVD-007` supports stakeholder ownership of relative weights. It does not supply values.
- For `OQ-002`, `EVD-008` and `EVD-009` show why context and content rating do not determine intended audience.
- For `OQ-006`, `EVD-011` and `EVD-012` support stakeholder agreement. They do not supply exclusions.

