# Local HTTP Analysis Backend

## Objective

Add an explicitly selected local inference backend to Project Init. It is a secondary provider for authoritative brief analysis, cited research, automatic clarification, and staged documentation generation. The first implementation targets `mistral.rs` behind its OpenAI-compatible HTTP API, while the workflow continues to depend on the existing capability-specific Rust traits so future providers such as Claude or Antigravity can be added without changing project state or workflow policy.

The primary user runs Project Init on Windows with Docker Desktop and WSL2, either CPU-only with at least 64 GB RAM or NVIDIA CUDA with at least 64 GB RAM and 12 GB VRAM. Local inference is intended to keep useful analysis and documentation work available when Codex is unavailable. Web research still requires network access; when it is unavailable the provider must report that limitation and must not invent citations.

## Confirmed User Experience

- `--provider codex` remains the default and preserves current behavior.
- `--provider local` verifies an already-running loopback server and otherwise starts the configured `mistral.rs` container.
- `--offline` retains its current deterministic, no-provider, no-network meaning. It is not an alias for local inference.
- Local analysis is authoritative after the same schema validation and atomic persistence used for Codex analysis. There is no second-opinion or consensus pass in this MVP.
- Local research uses DuckDuckGo through the `mistral.rs` built-in search tool and must return direct HTTPS citations that pass the existing evidence validators.
- Local documentation is generated only in the existing temporary staging directory. The model receives three application-owned tools: `list_expected_documents`, `read_document`, and `write_document`.
- A provider operation may continue while valid activity is arriving, but it has an absolute ten-minute limit. Silence is separately bounded by an activity-reset inactivity timeout.
- Failure is closed: no automatic provider fallback, no partial authoritative project, no uncited research answer, and no unvalidated document adoption.

## Commands and Configuration

The additive CLI contract is:

```text
project-init --provider local [local runtime options] new --brief <file>
project-init --provider local [local runtime options] run [project-id] [--auto-answer]
project-init --provider local [local runtime options] step <project-id>
project-init --provider local [local runtime options] open <project-id>
```

Local runtime settings are global options so creation, headless runs, single steps, and the workbench share one provider configuration:

```text
--local-endpoint <http://127.0.0.1:1234/v1>
--local-model-dir <directory containing the pinned GGUF>
--local-model-file <gemma-4-12b-it-qat-q4_0.gguf>
--local-image <versioned mistral.rs image tag or digest>
--local-device <cpu|cuda>
--local-port <1234>
```

The endpoint must use plain HTTP on loopback only. A non-loopback endpoint is rejected because this provider deliberately has no authentication or TLS contract. The model directory, image, and device are required for every managed local selection so Project Init can launch the runtime if the endpoint is not already healthy. CUDA users must choose an image matching their GPU compute capability and driver; Project Init does not guess a CUDA lane.

Initial model setup is explicit and reproducible:

```powershell
hf download google/gemma-4-12B-it-qat-q4_0-gguf gemma-4-12b-it-qat-q4_0.gguf --revision 29d097773436b69ff9feafd636ab4cf873786537 --local-dir C:\Models\project-init\gemma-4-12b
hf cache verify google/gemma-4-12B-it-qat-q4_0-gguf --revision 29d097773436b69ff9feafd636ab4cf873786537 --local-dir C:\Models\project-init\gemma-4-12b
Get-FileHash -Algorithm SHA256 C:\Models\project-init\gemma-4-12b\gemma-4-12b-it-qat-q4_0.gguf
```

Expected GGUF metadata at the pinned revision:

- filename: `gemma-4-12b-it-qat-q4_0.gguf`
- size: `6,975,879,296` bytes
- SHA-256: `93567e57a8fe10b23569b9d9ec38cd005deedf71e29477c421a4b83f418a538b`

Before relying on the model, run `mistralrs tune` for the actual machine and execute the Project Init capability probe. The desired context is 32K tokens, but the accepted context is the largest verified setting that fits the configured hardware. CUDA uses paged attention; CPU mode must not receive CUDA flags.

## Architecture

```text
CLI / TUI provider selection
          |
          v
ConfiguredProvider (closed dispatch enum)
          |
          +-- CodexCliClient
          |
          +-- LocalHttpProvider ----> RuntimeLauncher ----> Docker / mistral.rs
                     |
                     +-- AgentClient
                     +-- ResearchClient
                     +-- AutoAnswerClient
                     +-- DocumentationClient
```

The traits are analogous to small C# interfaces (`IAgentClient`, `IResearchClient`, and so on). `ConfiguredProvider` is analogous to a discriminated union that performs closed dispatch for the providers shipped by this binary. It avoids a single vague `ILlm` interface and keeps each capability's inputs, outputs, validation, and authority explicit.

`LocalHttpProvider` owns only transport, timeouts, response decoding, and runtime readiness. It does not own project state or document adoption. Existing domain validators remain authoritative at the boundary.

## HTTP Contract

The adapter uses `POST /v1/chat/completions` with `model: "default"` and OpenAI-compatible messages. Structured operations provide `response_format.type = "json_schema"`; research additionally provides `web_search_options` and a bounded `max_tool_rounds`.

Every external response is untrusted. The adapter bounds HTTP status bodies, SSE event size, accumulated content, token metadata, tool-call count, argument size, document size, and total generated bytes. Unknown response fields are ignored; missing required fields, malformed SSE, malformed JSON, and schema-invalid model output fail closed.

Streaming is used to expose bounded activity and reset the inactivity timer. It never exposes chain-of-thought. Cancellation interrupts the request immediately. One absolute ten-minute timer starts before runtime verification and cannot be reset.

## Sandboxed Documentation Tools

The `mistral.rs` server is not started with `--agent`, `--enable-code-execution`, or `--enable-shell`. Only `--enable-search` is enabled. Filesystem effects are performed by Project Init after strict tool-call validation:

- `list_expected_documents()` returns the exact `DocumentationRequest.required_paths` list.
- `read_document(relative_path)` accepts only an exact allowlisted path and returns bounded UTF-8 from staging.
- `write_document(relative_path, content)` accepts only an exact allowlisted path, rejects links and traversal, enforces per-file and aggregate limits, and writes inside staging.

Tool schemas use `strict: true`, `required`, exact path enums, and `additionalProperties: false`. Project Init caps the client-side tool loop. The model never receives a raw path, directory listing outside the allowlist, repository mount, home-directory mount, or arbitrary command primitive.

The runtime container has a read-only root filesystem, all Linux capabilities dropped, `no-new-privileges`, a bounded tmpfs, the dedicated model directory mounted read-only, and a Docker-owned cache volume for runtime/search assets. No repository, project-data, staging, home, or other host directory is mounted. The host port binds to `127.0.0.1`. The container receives outbound network access solely because DuckDuckGo research is in scope. A versioned image tag or digest is required; floating `latest` tags are rejected by configuration validation.

## Model and Runtime Choice

The MVP recommends Google's official QAT Q4_0 GGUF for Gemma 4 12B. At roughly 6.98 GB it is a plausible 12 GB VRAM starting point while remaining runnable with CPU offload or CPU-only memory. That is a fit hypothesis, not a guarantee: KV cache, context length, kernels, multimodal components, Docker/WSL overhead, and concurrent sequences change the actual requirement. `mistralrs tune` and a real capability probe are release gates for each supported hardware profile.

The runtime is `mistral.rs` 0.9-compatible and must be supplied as a versioned CPU or CUDA image appropriate for the host. The adapter targets the documented OpenAI-compatible fields: streaming chat completions, strict tools, JSON-schema response format, and web search.

## Testing Strategy

- Unit tests validate loopback endpoint rules, versioned-image rules, Docker arguments, exact model-file checks, SSE decoding, response bounds, and timeout/cancellation selection.
- Contract tests use a local deterministic HTTP fixture, never a live paid or downloaded model, to cover structured analysis, research requests, tool calls, malformed responses, inactivity, cancellation, and ten-minute hard limits with paused time where practical.
- Filesystem tests prove traversal, unlisted paths, links, oversized content, excessive rounds, and partial writes are rejected while valid allowlisted documents remain inside staging.
- CLI tests cover the default Codex provider, explicit local selection, `--offline` conflicts, and incomplete local launch configuration.
- A manual hardware smoke test verifies Docker readiness, `/health`, `/v1/models`, structured output, strict tool calling, DuckDuckGo citations, all required documents, cancellation, and acceptable context on one CPU and one CUDA profile.

## Boundaries

Always:

- Preserve capability-specific provider traits and current workflow validators.
- Validate configuration and all HTTP/model output at system boundaries.
- Keep generated files provisional until existing validation and adoption complete.
- Record provider identity, model identity when available, bounded activity, and failure reason without credentials or raw reasoning.

Never:

- Automatically fall back between Codex, local inference, and deterministic offline mode.
- Give the local model shell, Python, unrestricted filesystem, repository, home-directory, or remote mutation capabilities.
- Accept non-HTTPS citations, non-loopback inference endpoints, floating container tags, path traversal, symlink escapes, or unbounded tool loops.
- Treat successful inference as proof that hardware/context support is adequate.

Deferred:

- Claude, Antigravity, remote OpenAI-compatible servers, provider consensus, dynamic provider plugins, multimodal brief inputs, background model downloads, and automatic CUDA image selection.

## Success Criteria

- Every existing Codex and deterministic offline command remains compatible.
- Explicit local selection can create an authoritative analyzed project through a healthy local HTTP server.
- Local auto-answer produces only validator-accepted cited answers using DuckDuckGo or fails without mutation.
- Local generation and repair can create the complete required package solely through the three allowlisted document tools.
- Project Init verifies `/health` and launches a hardened configured container when needed.
- Valid activity prevents inactivity failure, but no operation exceeds ten minutes.
- Cancellation, HTTP failure, malformed output, missing documents, and validation failure leave authoritative state unchanged or resumably paused according to existing workflow rules.

## Sources

- mistral.rs Docker images, version pinning, loopback health checks, and GPU lanes: <https://docs.mistralrs.dev/guides/deploy/docker/>
- mistral.rs OpenAI-compatible server and endpoints: <https://docs.mistralrs.dev/guides/serve/openai-compatible-apis/>
- mistral.rs strict tool calling and bounded tool rounds: <https://docs.mistralrs.dev/guides/agents/tool-calling-basics/>
- mistral.rs GGUF selection and offline local files: <https://docs.mistralrs.dev/guides/models/run-gguf/>
- mistral.rs serve flags, including search and dangerous execution capabilities: <https://docs.mistralrs.dev/reference/cli/serve/>
- mistral.rs supported OpenAI fields: <https://docs.mistralrs.dev/reference/openai-compatibility/>
- mistral.rs hardware estimator: <https://docs.mistralrs.dev/guides/quantization/quantize-a-model/>
- Official Gemma GGUF repository: <https://huggingface.co/google/gemma-4-12B-it-qat-q4_0-gguf>
- Hugging Face CLI download and cache verification: <https://huggingface.co/docs/huggingface_hub/en/package_reference/cli>
