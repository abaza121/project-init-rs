# Research 04 — Technology

## Scope

The brief authorizes behaviors and a PC target but does not authorize an engine, language, operating-system baseline, renderer, audio middleware, input abstraction, data store, network service, build pipeline, or licensing model.

## Result

No external technology claim is retained. Selecting or comparing tools without team, budget, licensing, platform-baseline, and service-scope inputs would create an untraceable recommendation. Instead, `D-006` records an engine-agnostic initiation architecture and `D-014` records a reversible separation between deterministic rules and presentation.

## Requirement impact

- `DAD-REQ-003` requires a PC build but is blocked for final acceptance by `OQ-009`.
- `DAD-REQ-005` requires mouse-wheel tuning; alternative inputs remain `OQ-013`.
- `DAD-REQ-006` through `DAD-REQ-013` motivate deterministic behavioral interfaces and fixtures under `ASM-004`.
- `DAD-REQ-016` motivates an architecture capable of adding alternative sensory presentation, but it does not commit features.

## Required inputs before tool research

- `OQ-007`: engine, language, renderer, audio stack, licensing.
- `OQ-009`: operating systems, hardware, display, and performance baseline.
- `OQ-014`: persistence, leaderboard, replay, analytics, and network scope.
- `OQ-015`: team capabilities, budget, schedule, and decision roles.

## Deferred research

Once those inputs are approved, perform a time-bounded, primary-source comparison against exact criteria such as supported PC targets, 2D workflow, audio routing, input APIs, deterministic test support, licensing, build automation, and team proficiency. No candidate or ranking is invented in this package.

