# Physics and Browser Timing

## Verified Findings

EVD-006: Newton’s gravitation equation is an inverse-square central force and circular orbital speed depends on gravitational mass and orbital radius, according to [OpenStax University Physics Volume 1](https://openstax.org/books/university-physics-volume-1/pages/13-key-equations), accessed 2026-08-30; EVD-006 supports DEC-014 because inverse-square gravity is one possible physics-model basis but does not select the exact game force model.
EVD-007: Without gravity a moving satellite follows a straight path while gravity curves the path into orbit, according to [NASA’s orbit explainer](https://www.nasa.gov/solar-system/what-is-an-orbit-grades-5-8/), accessed 2026-08-30; EVD-007 supports DEC-014 because inertial satellite motion distinguishes release from gravity-driven curvature without authorizing zero released acceleration.
EVD-008: Fixed physics intervals promote consistent simulation updates, while decreasing the fixed timestep increases precision and computational work, according to the [Unity 6 manual](https://docs.unity3d.com/6000.0/Documentation/Manual/fixed-updates.html), accessed 2026-08-30; EVD-008 supports DEC-014 because fixed-step timing is a physics-model option but supplies no project timestep.
EVD-009: A typical 60 Hz display offers about 16.66 milliseconds per frame, while browser overhead and missed refresh cadence reduce the application’s usable budget and delivered frame rate, according to [web.dev rendering guidance](https://web.dev/articles/rendering-performance), accessed 2026-08-30; EVD-009 supports DEC-016 because display frame timing informs measurement but does not define a low-end-device frame-rate target.

## Boundary

These physical and runtime descriptions expose design variables. They do not settle the number of active attractors, attachment impulse, force scaling, integration method, timestep, collision geometry, release acceleration, or performance threshold.

## Prototype Instrumentation

Keep simulation stepping observable and replaceable. Capture object state, selected anchor, velocity around release, collision and skim events, and frame timings so later stakeholder choices can be compared without rebuilding the game rules.
