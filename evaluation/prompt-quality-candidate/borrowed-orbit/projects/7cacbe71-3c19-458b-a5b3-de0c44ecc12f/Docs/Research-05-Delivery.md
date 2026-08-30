# Research 05 — Delivery and validation

## Research question

What evidence constrains how a solo developer should plan, test, and validate the prototype?

## Evidence retained

Relevant canonical records are [EVD-004](Traceability.md#evd-004), [EVD-006](Traceability.md#evd-006), [EVD-013](Traceability.md#evd-013), [EVD-014](Traceability.md#evd-014), [EVD-015](Traceability.md#evd-015), [EVD-016](Traceability.md#evd-016), [EVD-017](Traceability.md#evd-017), and [EVD-019](Traceability.md#evd-019). These evidence items constrain accessibility consideration, participant/task definition, practice-related measurement, device-profile documentation, and real-hardware validation. They do not supply staffing, schedule, budget, or approval authority.

## Delivery interpretation

The brief itself supplies the solo-developer constraint (CON-002). The proposed architecture therefore favors configuration, small procedural assets, local fixtures, and explicit gates. That approach is D-013 and A-006, not a research-proven productivity claim.

### Suggested gate order — planning inference

1. Close or explicitly sponsor a bounded spike for OQ-001; implement T-003, T-004, and T-005.
2. Close OQ-002, OQ-003, OQ-004, OQ-005, OQ-006, and OQ-007 before representing accessibility, performance, or randomness as accepted scope.
3. Build and tune the core loop; execute each applicable named verification in Requirements.md where its gate permits.
4. Close OQ-008, OQ-009, and OQ-010, predeclare T-015, then run the movement-proof study.

This order minimizes work around unsettled assumptions but is not a schedule commitment.

## Controls

- Keep all numerical feel values versioned and linked to test/playtest records (OQ-011).
- Require each decision to name one subject, provenance, source IDs, evidence IDs, status, and rationale (Traceability.md).
- Keep remote analytics, publishing, deployment, purchase, and external-record mutation out of scope unless separately authorized.
- Treat documentation validation as document validation; it is not prototype validation.

## Unavailable research

No staffing calendar, cost estimate, velocity history, risk budget, dependency inventory, hosting selection, CI benchmark, security review, privacy assessment, release checklist, or prototype implementation was supplied or produced. R-014 therefore requires a future solo-scope inventory and accepted estimate rather than asserting feasibility now.

## Project consequence

The package supports initiation and a bounded spike. It does not provide a credible delivery date, budget, commercial release plan, or proof that one developer has completed the work.
