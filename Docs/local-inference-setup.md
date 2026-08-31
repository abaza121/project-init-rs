# Local Inference Setup

Project Init can use a managed `mistral.rs` HTTP server as an explicit secondary provider. The default remains Codex. Deterministic `--offline` mode remains separate and performs no provider or network work. GGUF remains the default compatibility format; native safetensors/plain loading is available with `--local-format plain` or its `tensor` alias.

## Supported starting profiles

- CPU-only Windows host with Docker Desktop/WSL2 and at least 64 GB RAM.
- NVIDIA CUDA Windows host with Docker Desktop/WSL2, at least 64 GB RAM, and at least 12 GB VRAM.

The 12 GB figure is a minimum starting point, not a guarantee of 32K context. KV cache, driver/runtime versions, compute capability, and concurrent sequences affect actual fit. Run the runtime estimator and an application smoke test on each machine.

## Recommended CUDA tensor/plain profile

The currently verified Windows CUDA profile uses the instruction-tuned `Qwen/Qwen3-4B-Instruct-2507` model with native safetensors. It completed the Project Init smoke test and produced valid clarification questions. Use a separate model directory; do not mix its files with a GGUF model.

```powershell
$modelRoot = 'C:\Models\project-init\qwen3-4b-instruct-2507'
New-Item -ItemType Directory -Path $modelRoot -Force | Out-Null
hf download Qwen/Qwen3-4B-Instruct-2507 --local-dir $modelRoot
```

The downloaded directory must contain `config.json`, one or more `.safetensors` shards, and the tokenizer assets. Project Init verifies the manifest and weight-shard presence before launching Docker.

### Authorize the optional search embedding model

Project Init enables mistral.rs web search, which downloads the gated `google/embeddinggemma-300m` model to rank search results. This is separate from the Qwen model download.

1. Sign in to <https://huggingface.co/google/embeddinggemma-300m>, review Google's terms, and select **Agree and access repository**.
2. Create a Hugging Face Read token at <https://huggingface.co/settings/tokens>.
3. Store it interactively in the Docker-owned cache volume used by Project Init. Do not put the token in a command line or commit it to the repository.

```powershell
docker run --rm -it --gpus all `
  --mount type=volume,source=project-init-mistralrs-cache,target=/data `
  ghcr.io/ericlbuehler/mistral.rs@sha256:0f9ef9babfc452dc46d3dde8d99cbac1c54cd798adb6775f28d3388a140e4f9e `
  login
```

The CUDA image needs `--gpus all` even for `login`, because it links to `libcuda.so.1`. The token is saved as `/data/token` in the named Docker volume, not in the repository. If Hugging Face returns HTTP 403 after login, the token is valid but the account has not yet accepted the model terms.

## Alternative pinned GGUF profile

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

Project Init repeats the byte-length and SHA-256 check before it launches a new GGUF container. Plain mode instead requires a model directory containing `config.json` and at least one regular `.safetensors` file. A healthy already-running endpoint is checked through `/health` and `/v1/models` instead.

## Choose and pull a versioned runtime image

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

## Estimate fit on the actual host

`mistralrs tune` is an estimator, not a benchmark. Run the versioned image selected above against the base model configuration and inspect the recommended context/device map before the smoke test.

```powershell
docker run --rm ghcr.io/ericlbuehler/mistral.rs:cpu-0.9.0 tune --profile balanced -m google/gemma-4-12B-it
```

For CUDA, add `--gpus all` and use the selected CUDA image. Project Init requests a 32K model and paged-attention context in CUDA mode. If the actual machine cannot load that configuration, treat the profile as unsupported until the configured context policy is deliberately revised and retested; do not assume the weight file's size proves fit.

## Run Project Init with the local provider

### Recommended Qwen tensor/plain smoke test

Use the exact image digest already loaded on the tested RTX 50-series CUDA host. Replace the model directory only if you deliberately select another native model.

```powershell
cargo run -- --data-dir .project-init-local-smoke --provider local --local-format tensor --local-model-dir C:\Models\project-init\qwen3-4b-instruct-2507 --local-image ghcr.io/ericlbuehler/mistral.rs@sha256:0f9ef9babfc452dc46d3dde8d99cbac1c54cd798adb6775f28d3388a140e4f9e --local-device cuda new --brief baseline/project-prompt.txt --name "Local instruct tensor smoke"
```

On a 12 GB GPU, mistral.rs can place most of this model on CUDA while offloading a small portion to CPU. This is supported but generation is slower than a full-GPU fit.

### GGUF examples

CPU:

```powershell
cargo run -- --data-dir .project-init --provider local --local-model-dir C:\Models\project-init\gemma-4-12b --local-image ghcr.io/ericlbuehler/mistral.rs:cpu-0.9.0 --local-device cpu new --brief baseline/project-prompt.txt --name "Local inference smoke"
```

CUDA example after selecting the correct image:

```powershell
cargo run -- --data-dir .project-init --provider local --local-model-dir C:\Models\project-init\gemma-4-12b --local-image ghcr.io/ericlbuehler/mistral.rs:cuda128-sm89-0.9.0 --local-device cuda new --brief baseline/project-prompt.txt --name "Local CUDA smoke"
```

The provider first checks `http://127.0.0.1:1234/health` and `http://127.0.0.1:1234/v1/models`. If the expected `default` model is not ready, it verifies the model file, starts `project-init-mistralrs-1234`, and polls until ready or the twenty-minute absolute limit expires. The named container remains available for later commands.

The same global settings work with `run`, `step`, and `open`. For example:

```powershell
cargo run -- --data-dir .project-init --provider local --local-model-dir C:\Models\project-init\gemma-4-12b --local-image ghcr.io/ericlbuehler/mistral.rs:cpu-0.9.0 --local-device cpu run <project-id> --auto-answer
```

Use `--local-port` when 1234 is unavailable. If `--local-endpoint` is supplied, its loopback port must match `--local-port`. Use `--local-docker-bin` only when Docker is not discoverable as `docker`.

### Native safetensors/plain mode

To load tensor weights rather than the pinned GGUF, place the native model directory outside the repository. It must contain `config.json` and one or more `.safetensors` files, along with any tokenizer or processor assets required by the model. Omit `--local-model-file` and select plain mode:

```powershell
cargo run -- --data-dir .project-init --provider local --local-format plain --local-model-dir C:\Models\project-init\gemma-4-plain --local-image <versioned-image-or-digest> --local-device cuda new --brief baseline/project-prompt.txt --name "Local plain smoke"
```

The `tensor` value is accepted as an alias for `plain`. The selected mistral.rs image must support the model architecture in the directory; changing from GGUF to plain does not add architecture support to an older image.

## Capabilities and failure behavior

- Initial analysis uses JSON-schema constrained output and becomes authoritative only through the existing atomic validation/persistence path.
- Research and automatic-answer workers use mistral.rs built-in web search, which uses DuckDuckGo. Direct HTTPS evidence must still pass Project Init's validators.
- Documentation uses only `list_expected_documents`, `read_document`, and `write_document`. Tool path arguments are exact enums derived from the expected package.
- Web research requires network access. Local analysis can remain useful without it, but research must fail closed rather than fabricate citations.
- Valid streaming model events reset an inactivity timer that defaults to 60 seconds. Use the global `--local-inactivity-timeout-secs 300` option for slower models; accepted values are 1–600 seconds. This also controls waiting for response headers and gaps between network reads; heartbeat-only traffic does not reset the valid-event timer.
- Research and automatic-answer planning, workers, and judgment have no total inference deadline while valid activity continues. Runtime verification and startup still have a twenty-minute hard limit. Initial analysis and documentation retain their twenty-minute total limit, including startup and tool rounds. Cancellation, response-size limits, and tool-round bounds remain enforced.
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

- `local model file must be ... exactly 6975879296 bytes`: repeat the pinned GGUF `hf download` and checksum verification.
- `local model file checksum does not match`: remove the corrupt GGUF model copy and download the pinned revision again.
- `local plain model directory must contain config.json` or `safetensors file`: add the native model manifest and weight shards, then retry with `--local-format plain`.
- `Could not access ... embeddinggemma-300m ... HTTP 401`: run the interactive Docker `login` command above to save a Hugging Face token in `project-init-mistralrs-cache`.
- `Could not access ... embeddinggemma-300m ... HTTP 403`: accept the EmbeddingGemma repository terms with the same Hugging Face account that owns the token, then retry.
- `Device cuda[0] can fit 0 layers` while loading the embedding model: the primary model has consumed available VRAM, so the optional search embedding model runs on CPU. This warning is expected on the verified 12 GB profile and does not prevent local analysis.
- `structured analysis is invalid: duplicate findings are not allowed`: use the recommended instruction-tuned Qwen profile instead of the reasoning-default `Qwen/Qwen3-4B`, then retry from a fresh data directory.
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
- <https://huggingface.co/Qwen/Qwen3-4B-Instruct-2507>
- <https://huggingface.co/google/embeddinggemma-300m>
- <https://huggingface.co/docs/huggingface_hub/en/package_reference/cli>
- <https://docs.nvidia.com/cuda/wsl-user-guide/index.html>
