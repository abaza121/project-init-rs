# Technology Notes

## Evidence
EVD-016: Godot’s stable documentation describes a dedicated 2D rendering engine, physics system, and 2D-specific tools ([Introduction to 2D](https://docs.godotengine.org/en/stable/tutorials/2d/introduction_to_2d.html), accessed 2026-08-30). EVD-016 supports DEC-002 because dedicated 2D systems informed the provisional Godot 4 choice for the single-screen 2D prototype.
EVD-017: Godot’s stable documentation says its HTML5 export publishes to browsers through WebAssembly and WebGL 2.0, while Godot 4 C# projects currently cannot export to the web ([Exporting for the Web](https://docs.godotengine.org/en/stable/tutorials/export/exporting_for_web.html), accessed 2026-08-30). EVD-017 supports DEC-002 because browser export and the C# restriction directly informed the GDScript and Compatibility renderer choice.
EVD-018: Godot’s stable documentation supports command-line release export from configured presets and identifies Windows and HTML5 artifact forms ([Exporting projects](https://docs.godotengine.org/en/stable/tutorials/export/exporting_projects.html), accessed 2026-08-30). EVD-018 supports DEC-018 because configured native desktop and HTML5 presets make repeatable dual-target artifacts possible.
EVD-019: Godot’s stable web-export documentation says browsers can require a user input gesture before audio playback and documents limitations for web sample playback effects ([Exporting for the Web](https://docs.godotengine.org/en/stable/tutorials/export/exporting_for_web.html#audio-playback), accessed 2026-08-30). EVD-019 supports DEC-011 because key-gesture activation and pre-rendered event samples reduce browser audio failure risk.

## Boundary

DEC-002 is provisional because no team capability, existing codebase, licensing review, or target-device profile was supplied. The external documentation establishes capabilities and limits, not that Godot is uniquely correct. OQ-006 still controls the final desktop and browser matrix.

## Compatibility Risks

- Browser tests must begin from an HTTP(S) server, not by opening exported files directly.
- The first keyboard interaction should also unlock audio where the browser requires it.
- WebGL 2.0 and the Compatibility renderer are the assumed browser floor under DEC-002.
- Audio feedback must be tested independently in native and browser artifacts because their playback paths differ.
