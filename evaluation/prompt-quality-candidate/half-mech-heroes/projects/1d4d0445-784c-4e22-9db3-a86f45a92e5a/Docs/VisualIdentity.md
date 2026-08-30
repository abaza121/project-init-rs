# Visual Identity

Status: provisional design direction derived from the brief. Creative approval is not present in the supplied context.

## Identity core

- Name: **Half-Mech Heroes** (authoritative; REQ-001).
- Personality: overbuilt, earnest, accident-prone, cooperative.
- Emotional sequence: urgent rescue → readable malfunction → shared recovery → comic relief.
- Visual priority: gameplay state first, character second, spectacle third.

## Shape language

Use a broad, asymmetric rescue-mech silhouette assembled from large geometric masses: cab-like torso, piston legs, one oversized magnetic arm, and one clearly separate shield plane. Civilians should read as a single compact silhouette class; wreckage and each hazard must have a distinct outer contour. This direction implements REQ-014 but does not select the unresolved civilian or hazard identities in OQ-007 and OQ-008.

Avoid fine internal detail as the primary identifier. Damage states should change pose, rhythm, or a large component—not rely only on small sparks or colour. Critical cues need redundant channels under the constraints referenced by EVD-EXP-004 and EVD-EXP-005.

## Colour system

The palette is provisional and must be contrast-tested before adoption.

| Token | Provisional use | Hex | Constraint |
|---|---|---:|---|
| Rescue amber | mech focal panels, objective accents | `#FFB000` | Never sole role indicator. |
| Signal cyan | movement-role cue | `#33D6FF` | Pair with left-side icon and label. |
| Shield magenta | tool-role cue | `#FF4FA3` | Pair with right-side icon and label. |
| Soot navy | background structure | `#111827` | Maintain entity separation. |
| Concrete mist | UI field and smoke contrast | `#D9E2E8` | Do not wash out civilians. |
| Alarm red | imminent damage only | `#E5484D` | Reserve for threat; pair with shape/motion/audio cue. |

## Typography

Use a heavy condensed display face for the title and a highly legible sans serif for all operational UI. The implementation font is not selected. At the provisional 1080p television context, critical text uses at least 26-pixel body height under ASM-001 and EVD-EXP-001; this is a non-authoritative test baseline, not a final brand size system.

## Motion and failures

- Prefer anticipation, overshoot, recoil, delayed secondary motion, and recoverable wobble.
- Make role swaps a large, immediate event using opposing side icons, a short mechanical reconfiguration, and redundant audio/haptic feedback where supported.
- Keep all humorous failures playable; a failure may disrupt control allocation or motion but must not obscure critical state or prevent attempt completion under REQ-015.

## Logo direction

Set `HALF-MECH` as two visibly mismatched mechanical halves joined by a stressed central fastener, with `HEROES` as a stable rescue-stencil base. This is a promptable concept, not an approved logo. Do not introduce a mascot, civilian identity, hazard identity, age cue, or platform mark until its owner approves it.

## Do / do not

| Do | Do not |
|---|---|
| Communicate roles with side, icon, text, colour, and motion. | Use colour alone for roles or hazards. |
| Keep silhouettes chunky at gameplay scale. | Depend on tiny bolts or internal linework for identification. |
| Let failures look dramatic but resolve quickly. | Let comedy conceal civilian state, timer, or threat. |
| Show rescue purpose through equipment and signage. | Select a specific civilian or hazard identity by implication. |
