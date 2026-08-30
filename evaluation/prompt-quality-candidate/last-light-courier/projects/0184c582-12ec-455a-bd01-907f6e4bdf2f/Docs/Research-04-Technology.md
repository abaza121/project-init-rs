# Research 04 — Technology

## Research boundary

This file records current technical evidence used for the provisional architecture in DEC-006. Framework selection is a reversible design inference, not stakeholder product authority. Current facts were checked on 2026-08-30 against the linked sources.

## Canonical evidence records

### EVD-014 — Phaser supports browser-first 2D development

- **Claim:** Phaser’s official documentation describes it as a 2D HTML5 game framework for web browsers with WebGL and Canvas rendering, JavaScript or TypeScript development, and third-party paths to native desktop packaging.
- **Source:** [Welcome to Phaser Docs](https://docs.phaser.io/)
- **Provenance:** External research performed 2026-08-30.
- **Reliability:** High; official framework documentation.
- **Limitation:** It does not guarantee compatibility with an undefined browser matrix or make a native wrapper part of Phaser itself.
- **Informs:** DEC-006; DEC-013; ASS-005.

### EVD-015 — Vite produces static-hostable bundles and has explicit browser targets

- **Claim:** Vite’s official production-build guide states that its build output is suitable for static hosting and documents default and configurable browser targets.
- **Source:** [Building for Production — Vite](https://vite.dev/guide/build.html)
- **Provenance:** External research performed 2026-08-30.
- **Reliability:** High; official tool documentation.
- **Limitation:** Supported project browsers remain a stakeholder-owned delivery choice; defaults can change by major version.
- **Informs:** DEC-006; DEC-013; OQ-008.

### EVD-016 — Phaser can fall back from WebGL to Canvas

- **Claim:** Phaser’s official tutorial recommends automatic renderer selection that tries WebGL and falls back to Canvas when needed; its effects documentation notes that some shader effects are WebGL-only.
- **Sources:** [Making your first Phaser Game](https://docs.phaser.io/phaser/getting-started/making-your-first-phaser-game) and [Phaser FX concepts](https://docs.phaser.io/phaser/concepts/fx)
- **Provenance:** External research performed 2026-08-30.
- **Reliability:** High; official framework documentation.
- **Limitation:** Fallback does not guarantee visual equivalence or performance. Renderer-specific glow must degrade deliberately.
- **Informs:** DEC-006; REQ-014 implementation planning.

### EVD-017 — Browser audio generally needs user interaction

- **Claim:** MDN documents that Web Audio contexts should be created or resumed from a user gesture because browser autoplay policies can suspend audio started without interaction.
- **Source:** [Web Audio API best practices — MDN](https://developer.mozilla.org/en-US/docs/Web/API/Web_Audio_API/Best_practices)
- **Provenance:** External research performed 2026-08-30.
- **Reliability:** High; maintained web-platform documentation summarizing browser behavior.
- **Limitation:** Exact policy varies by browser and must be verified in the approved matrix.
- **Informs:** DEC-006; REQ-015 implementation planning.

### EVD-018 — Hidden tabs expose visibility events and throttle work

- **Claim:** MDN documents visibility-change events and notes that browsers typically stop animation callbacks or throttle timers in hidden tabs.
- **Source:** [Page Visibility API — MDN](https://developer.mozilla.org/en-US/docs/Web/API/Page_Visibility_API)
- **Provenance:** External research performed 2026-08-30.
- **Reliability:** High; maintained web-platform documentation.
- **Limitation:** It does not decide whether this game pauses, forfeits, or continues on focus loss.
- **Informs:** DEC-006; OQ-009.

## Architecture inference

DEC-006 selects TypeScript, Phaser 4.1.x, and Vite provisionally because the evidence aligns with a 2D, web-first, keyboard arcade prototype. Exact patch versions and lockfile hashes must be pinned at implementation start. DEC-007 keeps rules independent of rendering to reduce framework lock-in and make collision and timing tests deterministic.

## Technology research still unavailable

No approved target hardware, browser/version matrix, operating-system matrix, native-wrapper requirement, performance budget, memory budget, deployment host, security review, dependency license review, CI provider, or package-size threshold exists. TechnicalArchitecture.md treats these as gates, not solved facts.

