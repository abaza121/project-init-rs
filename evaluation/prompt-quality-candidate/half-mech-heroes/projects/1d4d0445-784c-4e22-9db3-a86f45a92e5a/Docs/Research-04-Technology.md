# Research 04 — Technology

Research date: 2026-08-30  
Scope: official engine capability documentation used to show that the input brief does not uniquely determine a technology.

## Canonical evidence

### EVD-TEC-001 — Godot supports keyboard and gamepad input actions

- Claim type: sourced fact
- Claim: Godot’s stable documentation describes input actions backed by keyboard and gamepad inputs, including device-specific gamepad assignment.
- Primary source: [Godot: Player scene and input actions](https://docs.godotengine.org/en/stable/getting_started/first_3d_game/02.player_input.html)
- Reliability: high; official engine documentation.
- Checked: available 2026-08-30.
- Informs: DEC-028, DEC-041, and OQ-006. Capability does not constitute a Godot selection.

### EVD-TEC-002 — Unreal supports mapped action and axis input

- Claim type: sourced fact
- Claim: Unreal Engine documentation describes action and axis mappings for keyboard and gamepad input.
- Primary source: [Unreal Engine: Setting Up User Inputs](https://dev.epicgames.com/documentation/en-us/unreal-engine/setting-up-user-inputs-in-unreal-engine)
- Reliability: high; official engine documentation.
- Checked: available 2026-08-30; page currently identifies Unreal Engine 5.8 documentation.
- Informs: DEC-028, DEC-041, and OQ-006. Capability does not constitute an Unreal selection.

### EVD-TEC-003 — Godot console release support requires external arrangements

- Claim type: sourced fact
- Claim: Godot states that official console SDK access requires platform-holder approval and that console ports rely on third-party providers or self-developed tooling rather than official Godot Foundation ports.
- Primary source: [Godot Console Support](https://godotengine.org/consoles/)
- Reliability: high; official engine project documentation.
- Checked: available 2026-08-30.
- Informs: DEC-027, DEC-028, DEC-041, OQ-005, and OQ-006 by linking platform choice to technology feasibility without selecting either.

## Evidence-based conclusion

At least two candidate engines document relevant mapped keyboard/gamepad input, so the control brief does not uniquely select a stack. Console support models may materially affect feasibility. The architecture must remain engine-agnostic until stakeholders resolve platform and technology.

## Unavailable research

- Team size, skills, existing licenses, budget, source-control/build infrastructure, and schedule.
- Physics determinism spike or controller coexistence spike in any engine.
- Target-hardware profiling.
- License/legal review.
- Console NDA material or approved SDK access.

No engine ranking or recommendation is supportable from the available evidence.
