# Link Map

## Coverage Map

- AC-001 supports REQ-001 because the single-screen 2D arcade inspection tests the same single-screen 2D arcade subject; DEC-001 supports REQ-001 because the fixed playfield is the chosen single-screen 2D arcade structure.
- AC-002 supports REQ-002 because the dual-artifact launch inspection tests desktop and browser builds; DEC-002 supports REQ-002 because the Godot GDScript target is compatible with desktop and browser export; DEC-018 supports REQ-002 because versioned native and HTML5 presets make both artifacts inspectable.
- AC-003 supports REQ-003 because the courier ship in an unlit city tests the blackout courier subject; DEC-003 supports REQ-003 because the scene composition centers that courier ship and blackout city.
- AC-004 supports REQ-004 because movement sampling tests glowing trail deposition; DEC-004 supports REQ-004 because sampled movement points create the glowing trail geometry.
- AC-005 supports REQ-005 because the city-power indicator must change only with trail growth; DEC-005 supports REQ-005 because accumulated trail length drives visible city power.
- AC-006 supports REQ-006 because collision inspection tests an old trail as a solid obstacle; DEC-004 supports REQ-006 because old trail render geometry also supplies collision geometry.
- AC-007 supports REQ-007 because persistent trail growth tests the player-built maze; DEC-004 supports REQ-007 because retained segments accumulate as that maze.
- AC-008 supports REQ-008 because measured battery expiry tests the two-minute run; DEC-008 supports REQ-008 because the visible 120-second countdown implements that run duration.
- AC-009 supports REQ-009 because the available-to-carried state transition tests parcel collection; DEC-006 supports REQ-009 because the active parcel state records collection.
- AC-010 supports REQ-010 because exact identifier acceptance tests the matching tower; DEC-006 supports REQ-010 because paired parcel and tower identifiers enforce matching.
- AC-011 supports REQ-011 because controlled distance bands test close-pass multiplier behavior; DEC-007 supports REQ-011 because measured near-pass distance governs the multiplier.
- AC-012 supports REQ-012 because separate trail-contact and battery-expiry cases test both hazards; DEC-004 supports REQ-012 because old trail geometry enables collision; DEC-008 supports REQ-012 because battery zero enables expiry.
- AC-013 supports REQ-013 because the complete keyboard-only task sequence tests keyboard controls; DEC-009 supports REQ-013 because named keyboard actions cover the complete task sequence.
- AC-014 supports REQ-014 because asset inspection tests simple neon shapes; DEC-010 supports REQ-014 because flat luminous geometry is the selected simple-neon-shape language.
- AC-015 supports REQ-015 because seven distinguishable event sounds test strong audio feedback without detailed art; DEC-010 supports REQ-015 because detailed illustration is excluded; DEC-011 supports REQ-015 because event samples and audio buses implement strong audio feedback.
- AC-016 supports REQ-016 because controlled score comparison tests route planning and precision rewards; DEC-019 supports REQ-016 because scoring reads route efficiency and near-pass precision.
- AC-017 supports REQ-017 because a five-second observation tests the brief tutorial; DEC-012 supports REQ-017 because the skippable overlay exposes controls and goals within five seconds.
- AC-018 supports REQ-018 because register inspection tests unresolved default difficulty; DEC-013 supports REQ-018 because the default difficulty choice is explicitly deferred to the stakeholder.
- AC-019 supports REQ-019 because register inspection tests unresolved accessibility scope; DEC-014 supports REQ-019 because the bundled accessibility scope is explicitly deferred while seams remain separable.
- AC-020 supports REQ-020 because plan inspection tests the absence of adopted metaprogression; DEC-015 supports REQ-020 because persistent progression is explicitly deferred.
- AC-021 supports REQ-021 because claim inspection tests unresolved intended audience; DEC-016 supports REQ-021 because audience claims and demographic tuning are explicitly deferred.
- AC-022 supports REQ-022 because threshold inspection tests unresolved continuation criteria; DEC-017 supports REQ-022 because build verification is explicitly separated from continuation thresholds.

## External Claim Links

Canonical external claim text and its exact decision edge appear once in each topic-note `EVD-*` record. This map does not reconstruct those claims.

## Authority Flow

The brief supplies the clause-bearing requirements and exact constraints in their canonical registers. The five supplied `ANS-*` records constrain the unresolved-choice requirements. Non-authoritative interpretations appear only as `ASM-*` records, open authority appears only as `OQ-*` records, and consequential choices or deferrals appear only as `DEC-*` records.
