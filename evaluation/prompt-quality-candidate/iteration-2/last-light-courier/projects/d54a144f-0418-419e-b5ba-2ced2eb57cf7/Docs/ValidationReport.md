# Validation Report

## Scope

These results describe static checks actually performed on the Markdown package in the staging directory on 2026-08-30. They do not validate a game build, stakeholder approval, external market fit, usability, or continuation readiness.

## Results

- **PASS — required paths:** the required-path manifest contains 17 names and all 17 files are present after this report was added.
- **PASS — canonical definitions:** static parsing found 22 `REQ-*`, 22 `AC-*`, 19 `DEC-*`, 6 `ASM-*`, 5 `CON-*`, 5 `ANS-*`, 19 `EVD-*`, and 9 `OQ-*` definition lines; each prefix begins at 001, increases monotonically, and has no numeric gap.
- **PASS — brief retention:** static comparison found 20 brief-derived exact-wording fragments and every fragment occurs in the preserved verbatim brief.
- **PASS — implementation checks:** every `REQ-*` identifier is the explicit endpoint of at least one one-line `AC-*` edge containing an objective pass/fail observation.
- **PASS — consequential choices:** every `REQ-*` identifier is the explicit endpoint of at least one one-line `DEC-*` edge; every canonical decision line declares status, provenance type, and an exact identified input.
- **PASS — external claims:** every canonical `EVD-*` line contains a direct link and an explicit one-line edge to an identified `DEC-*`; external claims are not restated in this report.
- **PASS — authority labels:** high-impact interpretations are recorded as `ASM-*`; stakeholder-owned difficulty, accessibility, progression, audience, and continuation choices remain recorded as open and deferred.
- **PASS — semantic sections:** a heading-aware scan found zero nonblank unidentified lines beneath parser-sensitive headings.
- **PASS — identifier hygiene:** static search found zero canonical ID ranges, zero slash-combined canonical IDs, and zero forbidden `R-*`, `VER-*`, `A-*`, or `BCL-*` identifiers.
- **PASS — relationship syntax:** static search found no canonical relationship line lacking two explicit identified endpoints.
- **PASS — local navigation:** every relative Markdown link resolves to a file in the staging directory.

## Limits

External links were retained from supplied data and representative primary pages were opened read-only, but the session did not independently open every external URL or archive copies. The checks are static and cannot prove semantic correctness beyond the explicit subject-overlap inspections recorded in [Traceability.md](Traceability.md). Deferred records remain unresolved even though this package passes its document checks.
