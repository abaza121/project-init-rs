# Assumptions

**Canonical status:** This is the authoritative register of inferred choices. Every item is non-authoritative until a stakeholder accepts it. “Working” permits reversible prototyping only; it does not change the source requirements.

| ID | Subject | Inferred statement | Impact | Status | Basis | Affected records | Disposition / falsification |
|---|---|---|---|---|---|---|---|
| ASS-001 | Nominal run time | **Tuning hypothesis:** interpret “about two minutes” as a 120,000 ms default battery timer; no acceptance tolerance is authorized. | High | Working, non-authoritative | SC-008 | REQ-008; DEC-003; OQ-006 | Replace with stakeholder-approved target and tolerance. Falsified by any different authorization. |
| ASS-002 | Parcel concurrency | **Design inference:** one parcel is available or carried at a time in the prototype. | Medium | Working, non-authoritative | SC-009; SC-010 and scope minimization | REQ-009; REQ-010; DEC-002; OQ-013 | Confirm or replace through OQ-013. A multi-parcel direction falsifies it. |
| ASS-003 | Trail collision eligibility | **Design inference:** trail segments become collidable only after the ship has cleared them, preventing immediate collision with the segment being emitted. | High | Working, non-authoritative | SC-005; SC-006; SC-012 | REQ-006; REQ-012; DEC-007; OQ-010 | Prototype behind a named parameter; replace with authorized rule. |
| ASS-004 | Match communication | **Accessibility-informed recommendation:** distinguish parcel/tower matches with both color and a simple shape or icon. This is not an approved accessibility scope. | Medium | Proposed, non-authoritative | SC-010; EVD-004 | REQ-010; DEC-005; DEC-009 | Stakeholder may accept under OQ-002 or authorize a different redundant cue. |
| ASS-005 | Desktop interpretation | **Architecture inference:** deliver the first playable as a desktop-browser web build; a native desktop wrapper is deferred. | High | Working, non-authoritative | SC-003; EVD-014; EVD-015 | REQ-003; DEC-006; DEC-013; OQ-008 | A requirement for a native executable falsifies this delivery interpretation. |
| ASS-006 | Close pass | **Tuning hypothesis:** a close pass is time spent inside a non-colliding proximity band around an eligible old trail. Distance, duration, cooldown, and multiplier window are unset. | High | Working, non-authoritative | SC-011; SC-017 | REQ-011; REQ-017; DEC-004; OQ-014 | Instrument parameters and replace only with stakeholder direction or authorized playtest tuning. |
| ASS-007 | “Old trails” | **Design inference:** a short, configurable recent-tail exemption makes a trail “old”; the value is unset and linked to collision eligibility. | High | Proposed, non-authoritative | SC-012 | REQ-006; REQ-012; DEC-007; OQ-010 | Confirm semantic rule before final collision acceptance. |
| ASS-008 | Connectivity and persistence | **Architecture inference:** the prototype is local, single-player, and does not require accounts, servers, telemetry, cloud saves, or network play. | Medium | Working, non-authoritative | No brief clause requests online services; prototype scope | DEC-006; DEC-010; OQ-003 | Any approved persistence, telemetry, or social feature triggers architecture review. |
| ASS-009 | World scaling | **Design inference:** gameplay uses one fixed logical arena scaled to the available viewport while preserving aspect ratio. | Medium | Working, non-authoritative | SC-002; SC-003 | REQ-002; REQ-003; DEC-006 | Replace if approved platforms require reflowed or multiple arenas. |
| ASS-010 | Visual identity | **Creative inference:** the palette, typography, icon geometry, and tone in VisualIdentity.md are proposed production direction, not stakeholder-approved brand assets. | Medium | Proposed, non-authoritative | SC-004; SC-014; SC-016 | DEC-005; OQ-012 | Stakeholder approval or replacement required before treating it as final identity. |
| ASS-011 | Mission language | **Product inference:** the proposed mission and vision summarize the brief but are not verbatim stakeholder statements. | Medium | Proposed, non-authoritative | SC-002; SC-005; SC-006; SC-007; SC-008; SC-009; SC-010; SC-011; SC-012; SC-017; SC-018 | DEC-002; OQ-012 | Stakeholder approval or edits resolve. |

## Rules

- Never cite an assumption as stakeholder authority.
- High-impact assumptions must be visible where used and must have a linked open question or explicit replacement path.
- Evidence can support relevance or feasibility, but cannot accept an assumption on behalf of the stakeholder.
