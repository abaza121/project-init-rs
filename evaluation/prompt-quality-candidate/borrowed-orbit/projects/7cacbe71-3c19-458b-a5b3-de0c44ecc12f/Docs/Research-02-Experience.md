# Research 02 — Player experience and movement proof

## Research question

How can the prototype evaluate whether anchor switching is skillful and predictable without substituting generic guidance for project-specific success criteria?

## Evidence retained

The canonical evidence records are [EVD-011](Traceability.md#evd-011), [EVD-012](Traceability.md#evd-012), [EVD-014](Traceability.md#evd-014), [EVD-015](Traceability.md#evd-015), and [EVD-016](Traceability.md#evd-016).

## Evidence-grounded interpretation

- EVD-014 supports evaluating specified criteria with representative users; EVD-015 supports consistent benchmark tasks and measures; EVD-016 supports examining change with practice. Together they inform D-007 and the required fields in OQ-009 and OQ-010.
- EVD-011 and EVD-012 support treating variation versus controllability as a design tradeoff tied to goals/audience. They inform D-011 and OQ-007, not a numeric randomness setting.

## Proposed study shape — design inference

After OQ-008 defines participants, a study could use versioned fixed movement scenarios and repeated attempts. A predictability task might ask participants to select an anchor/release moment to reach a marked region; a skill task might compare task success, error distance, time, or control actions across trials. These are candidate measures only. Sample size, scenario count, repetitions, thresholds, exclusion rules, confidence treatment, and required improvement are explicitly unset.

Internal developer playtesting is useful for tuning and defect discovery, but it cannot alone meet a stakeholder-approved representative-user criterion unless the protocol explicitly authorizes that population.

## Experience hypotheses

- Visible selection, acceleration, tangent release, and stable outcomes may improve causal legibility.
- Short run/restart loops may support repeated practice.
- Scoring pressure may either clarify mastery or distract from learning the movement.

All three are unvalidated project hypotheses. They are not retained as evidence claims.

## Unavailable research

No playtest results, task protocol, participant definition, learning curve, prediction accuracy, task-success rate, error distribution, confidence rating, simulator trace, or qualitative interview data was supplied or produced.

## Project consequence

R-015 remains `not evaluated`. D-007 is pending, and T-015 must not run as a pass/fail study until OQ-008, OQ-009, and OQ-010 provide the predeclared fields.

