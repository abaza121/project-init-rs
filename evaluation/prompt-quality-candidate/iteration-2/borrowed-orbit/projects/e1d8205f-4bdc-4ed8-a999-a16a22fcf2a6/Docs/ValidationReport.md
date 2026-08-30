# Validation Report

## Scope

This report records local checks actually run on the staged Markdown package on 2026-08-30. It does not claim product correctness, stakeholder approval, external-page freshness, playtest success, browser compatibility, or implementation completeness.

## Counts observed

| Item | Count |
|---|---:|
| Required Markdown paths | 17 |
| Present Markdown paths | 17 |
| `REQ-*` definitions | 20 |
| `AC-*` definitions | 20 |
| `DEC-*` definitions | 25 |
| `ASM-*` definitions | 11 |
| `CON-*` definitions | 4 |
| `ANS-*` definitions | 4 |
| `EVD-*` definitions | 16 |
| `OQ-*` definitions | 7 |

## Mechanical checks

- Required-path comparison found no missing or extra Markdown files.
- Canonical-definition scan found no duplicate identifiers and no gaps from `001` to the highest number within each prefix.
- Requirement-to-criterion scan found every defined `REQ-*` on at least one one-line `AC-* supports REQ-* because` edge.
- Criterion grammar scan found every `AC-*` line contains an objective observation or threshold, explicit PASS and FAIL outcomes, the literal `supports` verb, and both endpoints.
- Decision grammar scan found every `DEC-*` line declares status, provenance type, one canonical input, one consequential subject, rationale, the literal `supports` verb, and a defined `REQ-*` endpoint.
- External-claim scan found every `EVD-*` line contains one direct HTTPS link and one `supports DEC-* because` edge to a defined decision.
- Prefix scan found no canonical `R-*`, `VER-*`, `A-*`, or `BCL-*` definitions.
- Semantic-heading scan found no non-record content beneath headings reserved for requirements, criteria, decisions, assumptions, constraints, supplied answers, external claims, or open matters.
- Reference scan found no undefined canonical identifier references.
- Link scan found no unresolved relative Markdown links.
- Brief-clause scan found all twenty retained exact brief phrases in `Requirements.md`, including the name and the four explicitly unchosen subjects.
- Trace-table scan found each record relationship row uses the literal `supports` verb with individual endpoints rather than identifier ranges.

## Limitations

The read-only web tool returned no inspectable content during this session. The sixteen external claims and direct links are therefore retained as imported from the supplied project snapshot, not independently reconfirmed. No application build, simulation, participant session, device benchmark, audio review, or visual review existed to run; the `AC-*` records define future observations rather than completed product tests.
