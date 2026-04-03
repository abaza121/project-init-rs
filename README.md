# Project Creation Prompt

This project wraps a staged Codex documentation workflow so a short project brief can be turned into a structured `Docs` package. The intent is to give later agents a stronger starting point by generating requirements, brand, research, architecture artifacts in a consistent order.

## Flow

1. `run-codex-docs-pipeline.bat` starts in the repository root and resolves the PowerShell runner at `scripts\run-codex-docs-pipeline.ps1`.
2. The batch file accepts either:
   - a direct prompt string, or
   - `-f path\to\prompt.txt` to load the project brief from a file.
3. If a direct prompt string is used, the batch file writes that text to a temporary runtime prompt file and passes that file into the PowerShell script.
4. The batch file ensures a `Docs` directory exists, resolves the `codex` executable from `CODEX_BIN` or the system path, and calls the PowerShell pipeline.
5. The PowerShell script validates paths, resolves the prompt file, chooses a writable `CODEX_HOME`, and creates supporting directories such as `Docs` and a Codex temp directory.
6. The script reads the project brief, optionally loads the newest `log-*.md` file in the repo root as a session-log template, and generates a modeled prompt sequence for the run.
7. From that data, the script builds one large instruction payload that tells Codex to create a staged documentation set in `Docs`, including:
   - `Docs\README.md`
   - `Docs\requirements.md`
   - `Docs\SWOT.md`
   - `Docs\MissionVision.md`
   - `Docs\VisualIdentity.md`
   - `Docs\BrandPrompt.md`
   - `Docs\TechnicalArchitecture.md`
   - `Docs\Research-01-<topic>.md` through `Docs\Research-05-<topic>.md`
   - `Docs\log-yyyy-MM-dd-HH-mm-ss.md`
8. The script runs `codex exec` in ephemeral mode with `workspace-write`, writes the last Codex summary message to `Docs\_pipeline-last-message.txt`, and returns the Codex exit code back to the batch caller.
9. If the batch file created a temporary prompt file, it deletes it before exiting.

## How To Execute

Prerequisites:

- Windows PowerShell
- Codex CLI available on `PATH`, or `CODEX_BIN` set to the Codex executable path

Run with the included prompt file:

```bat
run-codex-docs-pipeline.bat -f project-prompt.txt
```

Run with an inline brief:

```bat
run-codex-docs-pipeline.bat "I want to create a fishing VR game where the player explores calm fishing spots and receives subtle environmental cues."
```

Optional:

- Set `CODEX_BIN` if `codex` is not available on `PATH`.
- Add a root-level `log-*.md` file if you want the next run to use it as the structural session-log template.

The generated output is written into the repository `Docs` folder.
