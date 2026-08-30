# Local Inference Setup

Project Init can use a managed `mistral.rs` HTTP server as an explicit secondary provider. The default remains Codex. Deterministic `--offline` mode remains separate and performs no provider or network work.

## Supported starting profiles

- CPU-only Windows host with Docker Desktop/WSL2 and at least 64 GB RAM.
- NVIDIA CUDA Windows host with Docker Desktop/WSL2, at least 64 GB RAM, and at least 12 GB VRAM.

The 12 GB figure is a minimum starting point, not a guarantee of 32K context. KV cache, driver/runtime versions, compute capability, and concurrent sequences affect actual fit. Run the runtime estimator and an application smoke test on each machine.

## 1. Download and verify the pinned model

Install the current Hugging Face CLI, accept the Gemma license if prompted, and authenticate only for the download step. Keep models outside the repository and user home directories exposed to the container.

```powershell
$modelRoot = 'C:\Models\project-init\gemma-4-12b'
New-Item -ItemType Directory -Path $modelRoot -Force | Out-Null
hf download google/gemma-4-12B-it-qat-q4_0-gguf gemma-4-12b-it-qat-q4_0.gguf --revision 29d097773436b69ff9feafd636ab4cf873786537 --local-dir $modelRoot
hf cache verify google/gemma-4-12B-it-qat-q4_0-gguf --revision 29d097773436b69ff9feafd636ab4cf873786537 --local-dir $modelRoot
Get-FileHash -Algorithm SHA256 "$modelRoot\gemma-4-12b-it-qat-q4_0.gguf"
```

Expected identity:

| Field | Value |
| --- | --- |
| Revision | `29d097773436b69ff9feafd636ab4cf873786537` |
| File | `gemma-4-12b-it-qat-q4_0.gguf` |
| Bytes | `6,975,879,296` |
| SHA-256 | `93567e57a8fe10b23569b9d9ec38cd005deedf71e29477c421a4b83f418a538b` |

Project Init repeats the byte-length and SHA-256 check before it launches a new container. A healthy already-running endpoint is checked through `/health` and `/v1/models` instead.

## 2. Choose and pull a versioned runtime image

Project Init rejects floating `latest` tags and runs Docker with `--pull never`, so setup must pull the selected image explicitly.

CPU example:

```powershell
docker pull ghcr.io/ericlbuehler/mistral.rs:cpu-0.9.0
```

CUDA requires a tag matching the driver-supported CUDA lane and GPU compute capability. For example, an RTX 40-series card (`sm89`) on a CUDA 12.8-capable driver can start from:

```powershell
docker pull ghcr.io/ericlbuehler/mistral.rs:cuda128-sm89-0.9.0
```

Do not copy that CUDA tag blindly. Use `nvidia-smi`, the mistral.rs Docker tag table, and NVIDIA's WSL guidance to select the correct lane. Prefer an image digest when your deployment process records one.

## 3. Estimate fit on the actual host

`mistralrs tune` is an estimator, not a benchmark. Run the versioned image selected above against the base model configuration and inspect the recommended context/device map before the smoke test.

```powershell
docker run --rm ghcr.io/ericlbuehler/mistral.rs:cpu-0.9.0 tune --profile balanced -m google/gemma-4-12B-it
```

For CUDA, add `--gpus all` and use the selected CUDA image. Project Init requests a 32K model and paged-attention context in CUDA mode. If the actual machine cannot load that configuration, treat the profile as unsupported until the configured context policy is deliberately revised and retested; do not assume the weight file's size proves fit.

## 4. Run Project Init with the local provider

CPU:

```powershell
cargo run -- --data-dir .project-init --provider local --local-model-dir C:\Models\project-init\gemma-4-12b --local-image ghcr.io/ericlbuehler/mistral.rs:cpu-0.9.0 --local-device cpu new --brief baseline/project-prompt.txt --name "Local inference smoke"
```

CUDA example after selecting the correct image:

```powershell
cargo run -- --data-dir .project-init --provider local --local-model-dir C:\Models\project-init\gemma-4-12b --local-image ghcr.io/ericlbuehler/mistral.rs:cuda128-sm89-0.9.0 --local-device cuda new --brief baseline/project-prompt.txt --name "Local CUDA smoke"
```

The provider first checks `http://127.0.0.1:1234/health` and `http://127.0.0.1:1234/v1/models`. If the expected `default` model is not ready, it verifies the model file, starts `project-init-mistralrs-1234`, and polls until ready or the ten-minute absolute limit expires. The named container remains available for later commands.

The same global settings work with `run`, `step`, and `open`. For example:

```powershell
cargo run -- --data-dir .project-init --provider local --local-model-dir C:\Models\project-init\gemma-4-12b --local-image ghcr.io/ericlbuehler/mistral.rs:cpu-0.9.0 --local-device cpu run <project-id> --auto-answer
```

Use `--local-port` when 1234 is unavailable. If `--local-endpoint` is supplied, its loopback port must match `--local-port`. Use `--local-docker-bin` only when Docker is not discoverable as `docker`.

## Capabilities and failure behavior

- Initial analysis uses JSON-schema constrained output and becomes authoritative only through the existing atomic validation/persistence path.
- Research and automatic-answer workers use mistral.rs built-in web search, which uses DuckDuckGo. Direct HTTPS evidence must still pass Project Init's validators.
- Documentation uses only `list_expected_documents`, `read_document`, and `write_document`. Tool path arguments are exact enums derived from the expected package.
- Web research requires network access. Local analysis can remain useful without it, but research must fail closed rather than fabricate citations.
- Valid streaming model events reset a 60-second inactivity timer. Runtime verification, model work, and tool rounds share an unresettable ten-minute hard limit per provider operation.
- There is no automatic Codex/local/offline fallback. Select the intended authority explicitly.

## Container security boundary

Project Init launches Docker directly without a shell. The command:

- binds only `127.0.0.1:<port>`;
- mounts the dedicated model directory at `/models` read-only;
- uses a Docker-owned `project-init-mistralrs-cache` volume at `/data` for runtime/search assets;
- uses a read-only root filesystem and a bounded no-exec `/tmp` tmpfs;
- drops all Linux capabilities and enables `no-new-privileges`;
- disables the web UI;
- enables web search only; and
- never enables `--agent`, Python code execution, or shell execution.

The repository, project data directory, generated staging directory, and user home are not mounted into the container. Project Init itself applies validated document writes to temporary staging, and the existing package validator decides whether the result can be adopted.

## Troubleshooting

- `local model file must be ... exactly 6975879296 bytes`: repeat the pinned `hf download` and checksum verification.
- `local model file checksum does not match`: remove the corrupt model copy and download the pinned revision again.
- `Docker launch failed ... No such image`: pull the exact versioned CPU/CUDA image used by `--local-image`.
- CUDA startup failure: confirm Docker Desktop GPU support, the NVIDIA driver, WSL2 visibility, compute capability, and the selected image lane.
- A stopped or stale named container can be inspected with `docker logs project-init-mistralrs-1234`. Remove it deliberately before retrying; Project Init will not blindly restart an unverified old container configuration.
- Research failure while analysis works: confirm outbound HTTPS and allow the Docker-owned cache volume to populate the search embedding assets during initial connected setup.

## Primary references

- <https://docs.mistralrs.dev/guides/deploy/docker/>
- <https://docs.mistralrs.dev/guides/serve/openai-compatible-apis/>
- <https://docs.mistralrs.dev/guides/agents/tool-calling-basics/>
- <https://docs.mistralrs.dev/guides/models/run-gguf/>
- <https://docs.mistralrs.dev/reference/cli/serve/>
- <https://docs.mistralrs.dev/guides/quantization/quantize-a-model/>
- <https://huggingface.co/google/gemma-4-12B-it-qat-q4_0-gguf>
- <https://huggingface.co/docs/huggingface_hub/en/package_reference/cli>
- <https://docs.nvidia.com/cuda/wsl-user-guide/index.html>
