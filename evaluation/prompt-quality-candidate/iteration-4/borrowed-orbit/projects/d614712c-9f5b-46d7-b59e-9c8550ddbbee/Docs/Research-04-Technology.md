# Motion and Simulation Inquiry

## Scope

This inquiry supports only a reversible physics experiment. It does not select a browser framework, physics engine, rendering API, audio library, build system, or hosting platform.

## Findings

EVD-005: NASA explains that curved orbital motion requires center-directed acceleration supplied by gravity and that gravitational strength is inversely proportional to squared distance ([NASA Science](https://science.nasa.gov/learn/basics-of-space-flight/chapter3-3/)) supports DEC-001 because center-directed inverse-square gravity directly constrains the gated single-attractor acceleration experiment.
EVD-006: NASA explains that without gravity a moving satellite continues along a straight path and that momentum together with gravity produces orbital motion ([NASA](https://www.nasa.gov/solar-system/what-is-an-orbit-grades-5-8/)) supports DEC-002 because continuing momentum directly informs preserved satellite world-space velocity at release.
EVD-007: Box2D documentation recommends a fixed time step and says a 1/60-second step will usually provide a high-quality simulation ([Box2D simulation documentation](https://box2d.org/documentation/md_simulation.html)) supports DEC-003 because fixed 1/60-second stepping directly informs the proposed repeatable physics experiment.
EVD-008: Box2D documents semi-implicit Euler as its approximate differential-equation integration method ([Box2D FAQ](https://box2d.org/documentation/md_faq.html)) supports DEC-003 because semi-implicit Euler directly informs the proposed fixed-step integration method.

## Interpretation

Applying gravity only during a hold, choosing a single anchor, locking that anchor, clamping the force near contact, moving anchors kinematically, and omitting mutual attraction are project-specific design inferences. Numerical strength, radii, collision response, and final integration method require prototype measurement and stakeholder approval.

## Retrieval note

The supplied NASA technical-report PDF endpoint returned HTTP 403 during this session, so no retained claim relies solely on that unavailable document. The accessible NASA and Box2D pages above were inspected directly on 2026-08-30.
