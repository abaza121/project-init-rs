# Visual identity

Status: proposed implementation direction under D-006. The authoritative style requirement is R-013; this document operationalizes it without claiming stakeholder approval of specific colors or assets.

## Identity statement

Borrowed Orbit is drawn as a compact orbital diagram coming alive: dark space, restrained geometric bodies, thin trajectory echoes, and brief signal-like flashes. Motion explains interaction; decoration never competes with trajectory reading.

## Proposed visual system

### Shape language

- Satellite: a small diamond core with two short rectangular panels; orientation may follow velocity.
- Planets: circles with one ring, arc, or small satellite mark to distinguish types without texture.
- Debris: triangles, broken polygons, or paired line segments.
- Data fragments: small open squares that close or pulse on collection.
- Hazards: angular shapes with a distinct outer skim halo and solid collision core.
- Collapsing star: concentric circles/irregular radial spokes whose rhythm and scale communicate increasing pressure.
- Anchor relation: a thin curved tether or radial line that appears only while attached; the selected body gains a concise halo.

These recipes are proposed ways to satisfy R-013 and R-014. They are not additional requirements.

### Palette tokens

The following tokens are provisional and must be checked for contrast in the eventual approved contexts:

| Token | Proposed value | Role |
|---|---:|---|
| `space` | `#080B14` | Background |
| `satellite` | `#F3F7FF` | Player/highest-focus geometry |
| `gravity` | `#67D9FF` | Anchor/tether/trajectory cue |
| `data` | `#A8FFB0` | Collectible/positive feedback |
| `hazard` | `#FF6B78` | Collision danger |
| `skim` | `#FFCA68` | Risk/reward near-pass cue |
| `collapse` | `#C76BFF` | Star pressure/system escalation |

Color must not be the only carrier of mechanically necessary information; shape, motion, and sound should distinguish the same states (A-002).

### Composition and density

- Use one fixed playfield composition during active runs (R-016).
- Keep persistent marks limited to the player, active objects, required state cues, and a compact score/multiplier representation.
- Prefer negative space and short-lived trails over star-field texture.
- Avoid parallax layers that make distance and nearest-anchor judgment harder.

## Motion language

- Attach: tether grows satellite-to-anchor, satellite gives a one-frame directional squash/flare, and anchor halo resolves.
- Detach: tether retracts or snaps cleanly without a velocity kick; a short tangent trail previews continuity.
- Fragment: square closes, collapses into a point, and sends one pulse toward the score state.
- Skim: hazard halo compresses at closest approach and emits a brief tangent streak.
- Multiplier: orbit ring gains a segment on sustain and breaks into segments on reset.
- Danger/collapse: star rhythm accelerates and inward vectors become more apparent without adding instructional text.
- Death/restart ready: motion resolves into a compact collapse, then the start state becomes visibly responsive.

The exact timings are tuning, not approved values (OQ-011).

## Audio language

Use short synthesized tones that can be produced in-browser or as a tiny asset set: attach uses a rising lock tone, detach a dry release, fragments a clear ping, skim a brighter near-miss sweep, multiplier changes a compact harmonic step, danger a low pulse, and death a brief filtered collapse. Audio remains redundant under A-002 until stakeholders decide otherwise.

## Cue matrix for R-012 and T-012

| State | Required visual distinction | Proposed audio distinction |
|---|---|---|
| Attach | tether + selected-anchor halo | rising lock |
| Detach | tether removal + tangent trail | dry release |
| Fragment collection | closing square + score pulse | clear ping |
| Hazard skim | compressed halo + tangent streak | near-miss sweep |
| Multiplier change | orbit-ring segment change | harmonic step |
| Danger | star/inward motion escalation | low pulse |
| Death | satellite collapse/fade | filtered collapse |
| Restart ready | satellite/start state re-forms and responds | short ready tone or documented silent treatment |

## Asset budget approach

Maintain an inventory linking every visible/audio asset to a procedural recipe or a small editable source. No photoreal imagery, long-form animation, character illustration, voiceover, or required outsourced specialty asset is part of the proposed prototype. T-013 and T-014 determine conformance.
