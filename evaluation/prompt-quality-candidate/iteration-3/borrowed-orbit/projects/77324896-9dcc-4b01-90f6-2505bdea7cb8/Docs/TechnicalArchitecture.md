# Technical Design

## Decisions

DEC-001: Status: approved; choice: preserve a minimalist one-button browser game scope; provenance type: user brief; exact source: REQ-001; DEC-001 supports REQ-001 because the choice repeats minimalist one-button browser game and prevents added primary gameplay actions.
DEC-002: Status: approved; choice: make moving objects the gravity anchors used by the small satellite to survive; provenance type: user brief; exact source: REQ-002; DEC-002 supports REQ-002 because the choice repeats moving objects, gravity anchors, satellite, and survival.
DEC-003: Status: approved; choice: select the nearest eligible planet or debris when the button is held; provenance type: user brief; exact source: REQ-003; DEC-003 supports REQ-003 because the choice repeats holding, nearest eligible planet or debris, and attachment.
DEC-004: Status: approved; choice: preserve satellite momentum on button release to begin a new arc; provenance type: user brief; exact source: REQ-004; DEC-004 supports REQ-004 because the choice repeats release, preserved momentum, satellite, and new arc.
DEC-005: Status: approved; choice: make data fragments collectible; provenance type: user brief; exact source: REQ-005; DEC-005 supports REQ-005 because the choice repeats data fragments and collection.
DEC-006: Status: approved; choice: award bonus points for hazard skims; provenance type: user brief; exact source: REQ-006; DEC-006 supports REQ-006 because the choice repeats hazard skims and bonus points.
DEC-007: Status: approved; choice: provide a keep-alive orbit multiplier; provenance type: user brief; exact source: REQ-007; DEC-007 supports REQ-007 because the choice repeats orbit multiplier and keep-alive play.
DEC-008: Status: approved; choice: drift available gravity objects toward the collapsing star; provenance type: user brief; exact source: REQ-008; DEC-008 supports REQ-008 because the choice repeats available objects, drift, and collapsing star.
DEC-009: Status: approved; choice: terminate every run before three minutes; provenance type: user brief and constraint; exact sources: REQ-009 and CON-001; DEC-009 supports REQ-009 because the choice repeats every run and the less-than-three-minutes limit.
DEC-010: Status: proposed and reversible; choice: begin an instant restart no later than the next rendered frame after one activation; provenance type: planning assumption; exact sources: REQ-010 and ASM-002; DEC-010 supports REQ-010 because the choice operationalizes instant restart while retaining its provisional next-frame status.
DEC-011: Status: approved at principle level; choice: teach gameplay through animation and sound without visible instructional prose; provenance type: user brief and constraint; exact sources: REQ-011 and CON-002; DEC-011 supports REQ-011 because the choice repeats animation, sound, gameplay comprehension, and absence of visible instructional text.
DEC-012: Status: approved at principle level; choice: use sparse reusable geometric space art within solo-developer capacity; provenance type: user brief and constraint; exact sources: REQ-012, CON-003, and ASM-004; DEC-012 supports REQ-012 because the choice repeats sparse geometric space art and solo-developer production capacity.
DEC-013: Status: approved as prototype purpose but blocked on metric; choice: test skillful predictable gravity-anchor switching on one screen; provenance type: user brief and constraint; exact sources: REQ-013 and CON-005; DEC-013 supports REQ-013 because the choice repeats skillful predictable gravity-anchor switching and one screen.
DEC-014: Status: open; choice: withhold the exact physics model pending stakeholder confirmation; provenance type: supplied user answer; exact sources: REQ-014 and ANS-001; relevant technical boundaries: EVD-006, EVD-007, and EVD-008; DEC-014 supports REQ-014 because the exact physics model remains stakeholder-owned despite gravity, inertial motion, and fixed-step material.
DEC-015: Status: open; choice: withhold the input accessibility feature baseline pending stakeholder confirmation; provenance type: supplied user answer; exact sources: REQ-015 and ANS-002; relevant accessibility boundaries: EVD-003, EVD-004, and EVD-005; DEC-015 supports REQ-015 because input accessibility features remain stakeholder-owned despite keyboard, focus, and sustained-hold guidance.
DEC-016: Status: open; choice: withhold the low-end-device frame-rate target pending a confirmed cohort and measurement conditions; provenance type: supplied user answer; exact sources: REQ-016 and ANS-003; relevant frame-timing boundary: EVD-009; DEC-016 supports REQ-016 because frame rate and low-end devices cannot be selected from a typical display budget.
DEC-017: Status: open; choice: withhold the per-run randomness amount pending replay and predictability goals; provenance type: supplied user answer; exact sources: REQ-017 and ANS-004; relevant generator boundaries: EVD-013 and EVD-014; DEC-017 supports REQ-017 because randomness and seed policy remain stakeholder-owned despite evaluation and reproducibility methods.
DEC-018: Status: open; choice: withhold the intended audience pending stakeholder confirmation; provenance type: supplied user answer; exact sources: REQ-018 and ANS-007; relevant audience boundary: EVD-002; DEC-018 supports REQ-018 because the intended audience cannot be inferred from guidance about representative users.
DEC-019: Status: open; choice: withhold the quantitative skillful predictable movement threshold pending a defined audience, task, metric, and passing value; provenance type: supplied user answer; exact sources: REQ-019 and ANS-005; relevant usability boundary: EVD-001; DEC-019 supports REQ-019 because the quantitative movement threshold requires specified users, goals, and context.
DEC-020: Status: open; choice: withhold the supported browser and device scope pending audience and device confirmation; provenance type: supplied user answer; exact sources: REQ-020 and ANS-006; relevant compatibility boundaries: EVD-010, EVD-011, and EVD-012; DEC-020 supports REQ-020 because browser and device scope cannot be inferred from general compatibility or test guidance.
DEC-021: Status: proposed and reversible; choice: separate a single-page two-dimensional presentation shell from simulation, game rules, cue rendering, and deterministic test seams; provenance type: planning assumption; exact source: ASM-001; DEC-021 supports REQ-001 because the separated browser presentation shell retains the minimalist one-button game while keeping unsettled simulation choices replaceable.

## Runtime Shape

The proposed runtime has five replaceable boundaries:

1. The input adapter emits press, release, and restart intent without deciding device bindings.
2. The simulation boundary owns satellite and moving-object state without selecting a force law or integrator yet.
3. The rules layer owns fragments, hazard skim scoring, multiplier state, starward pressure, run termination, and reset.
4. The cue layer maps state transitions to geometric animation and sound while keeping nonvisual accessibility representation open.
5. The harness supplies controlled scenes, clocks, and observations for acceptance checks without setting live-run randomness policy.

## State Flow

The minimum state sequence is ready, running, terminated, and ready again. During running, press intent requests an eligible nearest anchor, release intent ends attachment without a release impulse, and rule events update score and multiplier. The physics equations, eligibility tie-breaking, collision geometry, and timestep remain explicit extension points.

## Data Shape

Runtime-only state should include the satellite transform and velocity, candidate object transforms, current anchor reference, fragment set, hazard contacts and skim bands, score, multiplier state, run clock, and terminal reason. No remote service, account, analytics, persistence, or leaderboard is authorized by the current brief.

## Quality Seams

The harness must be able to freeze inputs, inject controlled object arrangements, advance a controlled clock, capture pre-release and post-release velocity, inspect score transitions, and repeat runs. Performance instrumentation may record frame times, but no passing frame-rate number exists until the device cohort is confirmed.
