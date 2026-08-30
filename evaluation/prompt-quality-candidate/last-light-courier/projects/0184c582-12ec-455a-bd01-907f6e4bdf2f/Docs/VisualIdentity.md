# Visual Identity

## Status

This is a proposed identity system under ASS-010, DEC-005, and DEC-014. The brief authorizes simple neon shapes, strong audio feedback, a blackout setting, and no detailed art. Specific colors, typefaces, symbols, copy, and effects are creative inference pending OQ-012. Accessibility practices remain pending OQ-002.

## Concept: Powered paths in a sleeping grid

The visual system should make power and danger inseparable. Darkness provides negative space; thin geometric city traces wake as the courier’s line reaches them. The same bright line reads as infrastructure from afar and a hard boundary at play scale.

## Proposed palette

| Role | Hex | Use | Status |
|---|---|---|---|
| Blackout | #05070D | Primary playfield | Proposed |
| Grid shadow | #111827 | Unpowered city geometry | Proposed |
| Courier white | #F4FAFF | Player ship and highest-priority neutral cue | Proposed |
| Current cyan | #3CF2FF | Trail and powered route | Proposed |
| Parcel magenta | #FF3CC7 | One match family | Proposed |
| Tower amber | #FFC857 | Another match family, warnings when paired with shape | Proposed |
| Surge lime | #A8FF3E | Multiplier and successful close pass | Proposed |
| Fault coral | #FF5B6E | Collision and critical battery warning | Proposed |

**Constraint:** Do not rely on these colors alone for parcel/tower matching or critical state. Shape or icon redundancy is ASS-004, recommended by EVD-004 but not yet approved.

## Proposed geometry

- **Courier:** small forward-pointing kite or chevron with a white core.
- **Trail:** continuous line with a narrow solid collision core and a separate cosmetic halo.
- **Parcel:** compact outlined glyph; shape family is part of its match identity.
- **Tower:** larger stationary frame that repeats the parcel glyph.
- **City:** dim rectilinear traces that illuminate locally when powered.
- **Close-pass cue:** short, localized ring or edge spark; avoid full-screen flash.
- **Battery:** simple bar or arc plus text or icon state, not hue alone.
- **Multiplier:** large stable numeral with a restrained pulse on change.

Geometry is implementation guidance, not approved behavior. Collision uses logical geometry, never glow pixels.

## Logo direction

**Proposed wordmark:** narrow uppercase lettering with generous tracking; “LAST LIGHT” restrained, “COURIER” brighter or interrupted by one continuous route line.

**Proposed mark:** a tiny courier chevron turning around a corner made from its own luminous trail, forming an abstract “L.” Keep it recognizable at favicon size and reproducible as one color.

No specific commercial font is selected. Use a system or permissively licensed development font only after license review.

## Typography behavior

- Prefer a squared grotesk or monospaced display face for headings and HUD.
- Use a high-legibility system sans-serif for settings, instructions, and debug information.
- Avoid ultra-thin body text, all-caps paragraphs, glow directly on small text, and text embedded only in canvas when semantic DOM controls are available.
- Numeric timer and multiplier widths should remain stable during updates.

## Motion and effects

- Make the city-power transition directional and local, tied to actual trail state.
- Keep glow cosmetic and bounded so collision edges stay crisp.
- Avoid essential information conveyed only by camera shake, blur, or flashing.
- Keep a no-shake and low-effects implementation path available while OQ-002 is unresolved.
- Do not claim photosensitivity safety without the testing identified by EVD-007.

## Audio identity

Proposed audio should be synthetic, short, and event-shaped rather than cinematic: upward interval for collection, resolved chord for correct delivery, narrow rising tick for close passes, steady low-battery cadence, and abrupt broadband fault for collision. These are creative recommendations. REQ-015 authorizes distinguishable feedback; OQ-011 controls the audio-control scope.

## Tone of voice

Short, active, navigational: “PICK UP,” “MATCH,” “THREAD,” “DELIVER,” “BATTERY LOW.” Avoid lore-heavy copy during play. Do not use punitive or mocking failure language.

## Acceptance handoff

Use V-IDENT-001 and V-AUDIO-001 after OQ-012 approval. Approval must identify accepted palette, mark, typography licensing path, cue language, and any accessibility options separately.
