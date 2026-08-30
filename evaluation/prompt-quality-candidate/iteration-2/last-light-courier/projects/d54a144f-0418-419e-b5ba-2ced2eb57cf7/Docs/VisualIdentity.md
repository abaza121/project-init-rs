# Visual Identity

## Core Idea

“The route is the light.” The visual system should make the run’s history readable as a luminous line across a nearly black civic grid. This is a reversible creative interpretation of the blackout courier in REQ-003, the trail in REQ-004, city power in REQ-005, solid obstacle in REQ-006, maze growth in REQ-007, and the shape choice in DEC-010, not external evidence.

## Shape Language

- Ship: a tiny forward-pointing kite or arrowhead with a hollow center.
- Parcel: a compact geometric token carrying both a color and a simple glyph.
- Tower: a larger outlined beacon repeating its matching parcel’s color and glyph.
- Trail: a continuous narrow core with a restrained outer glow; collision geometry must remain visually centered on the core.
- City: dim blocks or window constellations that brighten locally as city power rises.
- Hazard state: old trail remains luminous but shifts from soft fresh glow to crisp solid edge after the collision-grace boundary.

The parcel and tower glyph pairing follows provisional DEC-006. It is not a claim that accessibility scope has been approved under REQ-019.

## Palette

| Role | Provisional color | Use |
|---|---:|---|
| Blackout | `#05070D` | Main field |
| Structure | `#162033` | Dim city geometry |
| Courier light | `#F5FBFF` | Ship core and high-priority text |
| Power cyan | `#25F4FF` | Trail and city activation |
| Parcel magenta | `#FF3FB4` | One match family |
| Parcel amber | `#FFC857` | One match family |
| Warning red | `#FF4D5A` | Collision and critical battery state |

Colors are exploration values, not approved accessibility settings. Gameplay-critical matching must use the paired glyph in DEC-006 and should not rely on hue alone.

## Type and Layout

Use a squared, highly legible sans serif with tabular numerals. Keep the battery, score, multiplier, and carried parcel at stable edges of the single playfield. Avoid decorative microcopy. The first-run overlay should express movement, pickup, match, trail danger, and battery in one glance and follow DEC-012.

## Motion

Use short pulses for pickup and delivery, a restrained trail bloom, a single crisp collision freeze, and a low-battery rhythm. Continuous camera shake, rapid flashing, and gratuitous background motion are outside the provisional identity. Final reduced-effects scope remains controlled by OQ-002.

## Sound Character

Favor concise electrical tones: a soft deposition hum, parcel chirp, consonant tower chord, near-pass tick, multiplier arpeggio, dry collision cut, and descending battery pulse. Each sound maps to the event set in AC-015 and to separate buses under DEC-011. Audio references are descriptive direction, not final assets.

## Usage Rules

- The trail is always the brightest continuous object but must not obscure the ship.
- Fresh-versus-solid trail state requires an edge or texture change, not glow intensity alone.
- Parcel and matching tower repeat both color and glyph.
- Score feedback may pulse but must not cover the route ahead.
- Detailed painted scenery, texture-heavy backgrounds, and cinematic overlays violate CON-002 and CON-004 for the prototype.
