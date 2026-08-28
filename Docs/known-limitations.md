# Known Limitations

- The current deterministic analyzer is intentionally conservative and does not yet invoke the isolated Codex CLI client described in the architecture.
- The TUI is a read-only overview; answers are currently recorded with the `answer` CLI command.
- Evidence, decisions, validation runs, agent runs, and document metadata have database tables but do not yet have complete workflow commands.
- Document generation currently emits the traceability core package, not the full mission/vision, SWOT, visual identity, brand prompt, research, and technical architecture set.
- Semantic retrieval abstractions and the LanceDB adapter are specified but not yet implemented. Relational mode is fully operational and SQLite remains sufficient for all current commands.
- Contradictions can be represented by trace and validation records in the schema, but automatic candidate discovery and bounded repair are not yet wired into the CLI.
- Evaluation tooling provides a reproducible case and metric contract but contains no claimed results.
