# Known Limitations

- Initial-brief analysis and package generation use separate ephemeral Codex CLI executions. The workbench can resume generation and bounded repair, but answer-driven re-analysis and provider-session continuation are intentionally deferred.
- The workbench can answer questions, create structured questions, adjust the clarification threshold, resolve pending decisions, and authorize repair. Editing immutable answers and mutating findings, requirements, or evidence directly remain deferred.
- `--auto-answer` imports cited evidence for clarification recommendations, but general package-generation research still does not import a structured evidence manifest automatically; additional sources can be recorded with `add-evidence` and appear in generated research artifacts.
- Automatic-answer dependency selection is model-reviewed rather than formally derived because questions do not yet store explicit dependency edges. Structural validation guarantees bounded eligible identities, but blind quality evaluation is still required to quantify whether batch judgment improves recommendations enough to justify its additional Codex cost.
- The artifact dependency graph is intentionally fixed. User-defined document kinds, arbitrary graphs, and parallel execution remain deferred.
- Semantic retrieval abstractions and the LanceDB adapter are specified but not yet implemented. Relational mode is fully operational and SQLite remains sufficient for all current commands.
- Contradictions can be represented by trace and validation records in the schema, but automatic contradiction candidate discovery remains deferred. Repairable document findings can receive explicitly authorized bounded Codex repair; missing truth and user authority still pause explicitly.
- Evaluation tooling provides a reproducible case and metric contract but contains no claimed results.
