# Compatibility, Generation, and Test Delivery

## Verified Findings

EVD-010: Cross-browser test combinations should be prioritized by importance to the target audience, and real devices provide the greatest behavioral and experience accuracy, according to [MDN testing strategy guidance](https://developer.mozilla.org/en-US/docs/Learn_web_development/Extensions/Testing/Testing_strategies), accessed 2026-08-30; EVD-010 supports DEC-020 because target-audience usage and real devices are inputs to browser and device scope, not a ready-made support matrix.
EVD-011: Web Platform Baseline describes feature availability across named popular browsers but is not a substitute for accessibility, usability, performance, security, or other testing and may not cover older devices or webviews, according to [MDN’s Baseline definition](https://developer.mozilla.org/en-US/docs/Glossary/Baseline/Compatibility), accessed 2026-08-30; EVD-011 supports DEC-020 because Baseline compatibility cannot by itself settle the supported browser and device scope.
EVD-012: Chrome Device Mode is a first-order approximation that does not run code on a mobile device, and actual mobile hardware is recommended when simulation is insufficient, according to [Chrome for Developers](https://developer.chrome.com/docs/devtools/device-mode), accessed 2026-08-30; EVD-012 supports DEC-020 because real-device behavior remains necessary when defining and validating device support.
EVD-013: Content-generator evaluation methods and metrics should reflect the designer’s intended goals, while differing intentions and player subjectivity complicate universal content-quality judgments, according to [Procedural Content Generation in Games, Chapter 12](https://www.pcgbook.com/chapter12.pdf), accessed 2026-08-30; EVD-013 supports DEC-017 because randomness evaluation requires declared replay and predictability goals.
EVD-014: A deterministic procedural generator can reproduce content from the same starting conditions and parameters, with a random seed controlling generation space, according to [Procedural Content Generation in Games, Chapter 1](https://www.pcgbook.com/chapter01.pdf), accessed 2026-08-30; EVD-014 supports DEC-017 because seed reproducibility is an available test mechanism but does not select the per-run randomness amount.

## Boundary

Compatibility labels, emulation, and seed mechanics are tools. None establishes the project’s users, support matrix, reference hardware, or desired variation.

## Delivery Gate

Before a public build is treated as validated, stakeholders must approve the audience and support cohort, after which checks should run on named real devices and browsers. Before generated layouts are tuned, stakeholders must approve the replay-variation and predictability goals.
