# Interaction and Comprehension

## Verified Findings

EVD-003: WCAG 2.2 Success Criterion 2.1.1 requires content functionality to be operable through a keyboard interface without specific timing for individual keystrokes except for path-dependent underlying functions, according to the [W3C Recommendation](https://www.w3.org/TR/WCAG22/#keyboard), accessed 2026-08-30; EVD-003 supports DEC-015 because keyboard operability constrains input accessibility features without selecting the game’s key or hold semantics.
EVD-004: Sustained holds can pose an access barrier, and toggle or automatic-hold alternatives should be considered for prolonged controls, according to [Microsoft Xbox Accessibility Guideline 107](https://learn.microsoft.com/en-us/xbox/accessibility/xbox-accessibility-guidelines/107), accessed 2026-08-30; EVD-004 supports DEC-015 because sustained-hold accessibility guidance informs but does not authorize Borrowed Orbit’s hold-or-toggle choice.
EVD-005: A keyboard-operable interface has a mode in which keyboard focus is visible under WCAG 2.2 Success Criterion 2.4.7, according to [W3C’s current explanation](https://www.w3.org/WAI/WCAG22/Understanding/focus-visible), accessed 2026-08-30; EVD-005 supports DEC-015 because visible keyboard focus constrains focus behavior without settling the full input accessibility baseline.

## Boundary

The material supports accessibility considerations, not a conformance claim or project-specific input scheme. In particular, it does not show that a toggle produces gameplay parity with a sustained gravity hold.

## Test Direction

Once stakeholders define supported inputs and audience, test both control access and preservation of the intended anchor-switching task. Keep visual, audio, and nonvisual cues synchronized to the same state changes.
