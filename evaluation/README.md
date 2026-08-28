# Evaluation

Use the same brief for the baseline and improved flow. Do not fill result files without executing the corresponding run.

## Baseline

```text
baseline\run-codex-docs-pipeline.bat -f evaluation\cases\fishing-vr.txt
```

## Relational implementation

```text
cargo run -- --data-dir evaluation/improved new --brief evaluation/cases/fishing-vr.txt --name "Calm Fishing VR"
```

Record model configuration when applicable, wall time, reported token usage, user-question count, agent-call count, validation/repair passes, output directory, artifact count, unresolved high-impact questions, and trace coverage. Preserve raw outputs beneath `evaluation/baseline/` and `evaluation/improved/`; summarize only measured values in `evaluation/reports/`.
