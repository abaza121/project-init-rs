# Mechanics Context

## Evidence
EVD-001: Inverse-square central gravity is a technically supported option for gravitational acceleration and orbital curvature, but it does not select a complete game physics model ([NASA Science](https://science.nasa.gov/learn/basics-of-space-flight/chapter3-3/)); imported from the supplied snapshot supports DEC-014 because inverse-square gravity is only one physics-model input and the exact model remains deferred.
EVD-002: An object continues according to its momentum when gravitational acceleration is absent, supporting velocity preservation as a physically grounded option without mandating a game release rule ([NASA](https://www.nasa.gov/solar-system/what-is-an-orbit-grades-5-8/)); imported from the supplied snapshot supports DEC-014 because momentum mechanics do not settle the exact Borrowed Orbit physics model.
EVD-003: A single central field can use gravitational acceleration with magnitude g = μ/r² ([NASA Technical Reports Server](https://ntrs.nasa.gov/api/citations/19770014192/downloads/19770014192.pdf)); imported from the supplied snapshot supports DEC-014 because the single-source equation does not authorize selecting one active gravity source for the game physics model.
EVD-004: Fixed-timestep simulation is supported by Box2D simulation guidance ([Box2D](https://box2d.org/documentation/md_simulation.html)); imported from the supplied snapshot supports DEC-019 because fixed-timestep guidance informed the replaceable simulation-policy boundary without selecting a timestep.

## Interpretation boundary

Physical grounding can improve legibility, but research does not decide anchor locking, source count, body kinematics, collision handling, integrator, or timestep. Those remain within `OQ-001`.
