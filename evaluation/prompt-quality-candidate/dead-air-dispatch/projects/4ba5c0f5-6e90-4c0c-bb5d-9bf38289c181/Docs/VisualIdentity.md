# Visual Identity Proposal

Status: proposed, non-authoritative, and reversible.  
Decision: `D-005`.  
Authorized source permission: `SRC-006`.  
Accessibility evidence references: `EVD-001` and `EVD-002`; these do not authorize features.

## Identity idea

**A storm heard through instruments.** The proposed identity combines emergency-broadcast utility, nocturnal supernatural unease, and the precision of a tuning instrument. The interface should feel operated rather than merely observed.

## Proposed visual grammar

| Element | Proposed direction | Status/boundary |
|---|---|---|
| Composition | One central tuning control, a horizontal or radial frequency scale, and peripheral signal traces. | Inference; control layout not approved. |
| Forms | Abstract waveforms, caller and entity silhouettes, antenna geometry, narrow instrument marks. | Waveforms and silhouettes are permitted by `SRC-006`; exact forms are proposed. |
| Motion | Sweep, lock, drift, pulse, interference buildup, and abrupt dropout tied to simulation state. | Inference; timings remain implementation tuning. |
| Texture | Restrained scan-line breakup and static used as atmosphere, not as an excuse to obscure critical state. | Static is permitted; legibility rule is a recommendation. |
| Space | A single-room first-playable frame with depth implied through signal layers rather than navigable geography. | `SRC-006` permits a single room; selecting it remains `OQ-011`. |
| Type | Condensed display face for labels plus a highly legible UI face for instructions and captions if adopted. | Typeface and caption scope are unapproved. |

## Proposed palette roles

These values are creative starting points, not accessibility certification or final production tokens.

| Role | Hex | Intended use |
|---|---|---|
| Storm black | `#090B12` | primary field |
| Receiver navy | `#121C2A` | panels and depth layers |
| Signal ivory | `#E8E2CE` | primary text and tuned traces |
| Rescue amber | `#F2B84B` | caller/guidance emphasis |
| Hazard magenta | `#D34B79` | hostile/interference emphasis |
| Route cyan | `#55C9C7` | safe-route emphasis |

Color is only a proposed reinforcing channel. Important state should not depend on color alone if stakeholders adopt the evidence-backed recommendation in `D-015`; shape, motion, pattern, text, or position candidates should be reviewed under `OQ-001`.

## Signal-state candidates

| State | Shape/pattern candidate | Motion candidate | Text candidate |
|---|---|---|---|
| caller present | broken speech-like waveform plus person silhouette | irregular breath pulse | caller identifier if adopted |
| signal locked | nested brackets around trace | steady convergence | “LOCK” if adopted |
| safe route | dashed path or chevrons | outward traveling pulse | route label if adopted |
| dangerous band | serrated boundary plus hatch | inward pressure/flicker | danger label if adopted |
| hostile approach | narrowing antenna-distance indicator | accelerating pulse | direction/distance caption if adopted |

These are design options, not requirements. Final semantics depend on `OQ-001`, `OQ-010`, and `OQ-012`.

## Logo/wordmark proposal

- Uppercase or small-cap “DEAD AIR” with wider tracking; “DISPATCH” as a compact stamped subline.
- A tuning notch or interrupted carrier line may cross the wordmark.
- Avoid faux-emergency seals, real agency insignia, and dense glitching that harms recognition.
- The canonical name remains `DAD-REQ-001`; no alternate name or tagline is approved.

## Validation needed

- Stakeholder identity review for tone and distinctiveness.
- Contrast and non-color state differentiation checks after token selection.
- Static/animation readability review in representative play.
- Caption/UI coexistence review only if those features are adopted.
- Player evaluation against the audience and measures eventually approved under `OQ-002` and `OQ-004`.

