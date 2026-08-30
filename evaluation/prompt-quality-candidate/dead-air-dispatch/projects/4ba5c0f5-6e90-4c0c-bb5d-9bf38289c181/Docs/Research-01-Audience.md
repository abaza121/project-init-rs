# Research 01 — Audience

## Research question

Can the supplied platform, genre, mechanics, or likely rating identify the intended player audience?

## Finding

No. The stakeholder has not authorized an audience (`ANS-004`), and the retained external evidence does not fill that authority gap. This memo therefore informs `DAD-REQ-017`, `D-010`, and `OQ-002`; it does not propose a persona.

## Canonical evidence records

### EVD-008 — Users and user groups belong to context of use

- Claim: ISO 9241-115:2024 defines context of use through users, goals, tasks, resources, and environment, and recognizes that different users or user groups can have different needs.
- Source: [ISO 9241-115:2024 — Guidance on conceptual design, user-system interaction design, user interface design and navigation design](https://www.iso.org/standard/80773.html)
- Reliability: high; primary standards body.
- Verification: official standard page located and accessible on 2026-08-30; detailed text is partly paywalled, so the claim is retained from the supplied evidence snapshot and not extended beyond it.
- Authority boundary: informs why audience must be explicit; does not select an audience.
- Trace: `DAD-REQ-017`; `D-010`; `OQ-002`.

### EVD-009 — Content rating is not intended audience

- Claim: ESRB rating categories communicate suggested age appropriateness, while content descriptors identify content that may have triggered a rating or concern consumers.
- Source: [ESRB Ratings Guide](https://www.esrb.org/ratings-guide/)
- Reliability: high for the ESRB system; primary rating body.
- Verification: official guide accessed 2026-08-30.
- Authority boundary: an eventual rating can communicate content suitability but cannot establish whom Dead Air Dispatch intends to serve.
- Trace: `DAD-REQ-017`; `D-010`; `OQ-002`.

## Evidence and inference boundary

- Evidence: audience is part of the use context; an ESRB rating has a different function.
- Project-context fact: the intended audience is not specified.
- Inference: audience discovery should precede approval of the hidden-information fairness test population. This is a planning recommendation, not a requirement beyond `DAD-REQ-015`.

## Unavailable research

No stakeholder-approved geography, language, age band, skill segment, access-needs population, budget, storefront, pricing model, competitor set, or player-research sample is available. Segment sizing and personas are deliberately not invented.

