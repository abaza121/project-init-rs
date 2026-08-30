//! Builds the managed runtime from mistral.rs' published Docker and serve contracts.
//! See <https://docs.mistralrs.dev/guides/deploy/docker/> and
//! <https://docs.mistralrs.dev/reference/cli/serve/>.

use std::ffi::OsString;
use std::fs::{self, File};
use std::io::Read;
use std::path::{Component, PathBuf};

use clap::ValueEnum;
use sha2::{Digest, Sha256};
use tokio::process::Command;
use tokio::time::Instant;

use super::{AgentError, CancellationToken, sanitize_terminal_text};

const CONTAINER_MODEL_DIRECTORY: &str = "/models";
const DEFAULT_CONTEXT_LENGTH: &str = "32768";
const GEMMA_MODEL_BYTES: u64 = 6_975_879_296;
const GEMMA_MODEL_SHA256: &str = "93567e57a8fe10b23569b9d9ec38cd005deedf71e29477c421a4b83f418a538b";

/// Selects the mutually exclusive CPU or NVIDIA CUDA mistral.rs launch profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum LocalDevice {
    /// Forces inference to remain on CPU and system memory.
    Cpu,
    /// Grants the container NVIDIA GPU access and enables paged attention.
    Cuda,
}

/// Selects native safetensors/plain weights or an exact GGUF artifact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum LocalModelFormat {
    /// Loads a model directory containing config and safetensors files.
    #[value(alias = "tensor")]
    Plain,
    /// Loads one exact GGUF file with the pinned Gemma identity check.
    Gguf,
}

/// Holds validated inputs used to build a hardened local mistral.rs container command.
#[derive(Debug, Clone)]
pub struct LocalRuntimeConfig {
    model_directory: PathBuf,
    model_file: Option<String>,
    image: String,
    device: LocalDevice,
    format: LocalModelFormat,
    port: u16,
    docker_executable: OsString,
}

impl LocalRuntimeConfig {
    /// Creates a runtime configuration without touching Docker or downloading model files.
    pub fn new(
        model_directory: PathBuf,
        model_file: &str,
        image: &str,
        device: LocalDevice,
        port: u16,
    ) -> Result<Self, AgentError> {
        Self::new_with_format(
            model_directory,
            Some(model_file),
            image,
            device,
            LocalModelFormat::Gguf,
            port,
        )
    }

    /// Creates a runtime configuration for either a plain model directory or a GGUF file.
    pub fn new_with_format(
        model_directory: PathBuf,
        model_file: Option<&str>,
        image: &str,
        device: LocalDevice,
        format: LocalModelFormat,
        port: u16,
    ) -> Result<Self, AgentError> {
        if !model_directory.is_absolute() {
            return Err(AgentError::InvalidRequest(
                "local model directory must be an absolute path".to_owned(),
            ));
        }
        let Some(model_directory_text) = model_directory.to_str() else {
            return Err(AgentError::InvalidRequest(
                "local model directory must be valid Unicode for the Docker mount".to_owned(),
            ));
        };
        if model_directory_text.contains(',') {
            return Err(AgentError::InvalidRequest(
                "local model directory must not contain a comma".to_owned(),
            ));
        }
        match (format, model_file) {
            (LocalModelFormat::Gguf, Some(file)) if is_file_name(file) => {}
            (LocalModelFormat::Gguf, _) => {
                return Err(AgentError::InvalidRequest(
                    "local GGUF model file must be one filename without path components".to_owned(),
                ));
            }
            (LocalModelFormat::Plain, Some(_)) => {
                return Err(AgentError::InvalidRequest(
                    "plain local model format does not accept --local-model-file".to_owned(),
                ));
            }
            (LocalModelFormat::Plain, None) => {}
        }
        let image = image.trim();
        if !is_pinned_image(image) {
            return Err(AgentError::InvalidRequest(
                "local runtime image must use a versioned tag or digest, not latest".to_owned(),
            ));
        }
        if port == 0 {
            return Err(AgentError::InvalidRequest(
                "local runtime port must be greater than zero".to_owned(),
            ));
        }
        Ok(Self {
            model_directory,
            model_file: model_file.map(str::to_owned),
            image: image.to_owned(),
            device,
            format,
            port,
            docker_executable: "docker".into(),
        })
    }

    /// Overrides Docker resolution for controlled installations and deterministic tests.
    pub fn with_docker_executable(mut self, executable: OsString) -> Result<Self, AgentError> {
        if executable.is_empty() {
            return Err(AgentError::InvalidRequest(
                "Docker executable override must not be empty".to_owned(),
            ));
        }
        self.docker_executable = executable;
        Ok(self)
    }

    /// Returns direct Docker arguments for one least-privilege detached runtime.
    pub fn docker_arguments(&self) -> Vec<OsString> {
        let container_name = format!("project-init-mistralrs-{}", self.port);
        let port_mapping = format!("127.0.0.1:{0}:{0}", self.port);
        let model_mount = format!(
            "type=bind,source={},target={CONTAINER_MODEL_DIRECTORY},readonly",
            self.model_directory.display()
        );
        let mut arguments = vec![
            "run".into(),
            "--detach".into(),
            "--name".into(),
            container_name.into(),
            "--pull".into(),
            "never".into(),
            "--publish".into(),
            port_mapping.into(),
            "--read-only".into(),
            "--cap-drop".into(),
            "ALL".into(),
            "--security-opt".into(),
            "no-new-privileges".into(),
            "--tmpfs".into(),
            "/tmp:rw,noexec,nosuid,size=256m".into(),
            "--mount".into(),
            model_mount.into(),
            "--mount".into(),
            "type=volume,source=project-init-mistralrs-cache,target=/data".into(),
        ];
        if self.device == LocalDevice::Cuda {
            arguments.extend(["--gpus".into(), "all".into()]);
        }
        arguments.extend([
            self.image.clone().into(),
            "serve".into(),
            "--host".into(),
            "0.0.0.0".into(),
            "--port".into(),
            self.port.to_string().into(),
            "--no-ui".into(),
            "--max-seq-len".into(),
            DEFAULT_CONTEXT_LENGTH.into(),
            "--max-tool-rounds".into(),
            "16".into(),
            "--enable-search".into(),
            "--search-embedding-model".into(),
            "embedding-gemma".into(),
        ]);
        arguments.extend([
            "--model-id".into(),
            CONTAINER_MODEL_DIRECTORY.into(),
            "--format".into(),
            match self.format {
                LocalModelFormat::Plain => "plain".into(),
                LocalModelFormat::Gguf => "gguf".into(),
            },
        ]);
        if let (LocalModelFormat::Gguf, Some(model_file)) = (self.format, &self.model_file) {
            arguments.extend([
                "-f".into(),
                format!("{CONTAINER_MODEL_DIRECTORY}/{model_file}").into(),
            ]);
        }
        match self.device {
            LocalDevice::Cpu => arguments.push("--cpu".into()),
            LocalDevice::Cuda => arguments.extend([
                "--paged-attn".into(),
                "on".into(),
                "--pa-context-len".into(),
                DEFAULT_CONTEXT_LENGTH.into(),
            ]),
        }
        arguments
    }

    /// Returns the absolute model file or directory expected before Docker starts.
    pub fn model_path(&self) -> PathBuf {
        self.model_file.as_deref().map_or_else(
            || self.model_directory.clone(),
            |file| self.model_directory.join(file),
        )
    }

    /// Returns the loopback host port exposed by the runtime.
    pub const fn port(&self) -> u16 {
        self.port
    }

    /// Returns the deterministic container name associated with the configured loopback port.
    pub fn container_name(&self) -> String {
        format!("project-init-mistralrs-{}", self.port)
    }

    /// Verifies the pinned model and launches the configured detached Docker container.
    pub async fn launch(
        &self,
        cancellation: &CancellationToken,
        deadline: Instant,
    ) -> Result<(), AgentError> {
        let model_directory = self.model_directory.clone();
        let model_file = self.model_file.clone();
        let format = self.format;
        let verification = tokio::task::spawn_blocking(move || {
            verify_model_identity(
                &model_directory,
                model_file.as_deref(),
                format,
                GEMMA_MODEL_BYTES,
                GEMMA_MODEL_SHA256,
            )
        });
        tokio::select! {
            result = verification => result.map_err(|error| {
                AgentError::Execution(format!("model verification task failed: {error}"))
            })??,
            () = cancellation.cancelled() => return Err(AgentError::Cancelled),
            () = tokio::time::sleep_until(deadline) => return Err(AgentError::TimedOut),
        }
        let output = self
            .run_docker(self.docker_arguments(), cancellation, deadline)
            .await?;
        if !output.status.success() {
            let diagnostics = sanitize_terminal_text(&String::from_utf8_lossy(&output.stderr));
            return Err(AgentError::Execution(if diagnostics.is_empty() {
                format!("Docker launch failed with {}", output.status)
            } else {
                format!("Docker launch failed: {diagnostics}")
            }));
        }
        Ok(())
    }

    /// Runs one direct Docker command without a shell and under the operation deadline.
    async fn run_docker(
        &self,
        arguments: Vec<OsString>,
        cancellation: &CancellationToken,
        deadline: Instant,
    ) -> Result<std::process::Output, AgentError> {
        let mut command = Command::new(&self.docker_executable);
        command.args(arguments).kill_on_drop(true);
        tokio::select! {
            output = command.output() => output.map_err(|error| {
                AgentError::Execution(format!("could not start Docker: {error}"))
            }),
            () = cancellation.cancelled() => Err(AgentError::Cancelled),
            () = tokio::time::sleep_until(deadline) => Err(AgentError::TimedOut),
        }
    }
}

/// Returns whether a string is one ordinary GGUF filename without traversal or separators.
fn is_file_name(value: &str) -> bool {
    let path = std::path::Path::new(value);
    !value.contains(',')
        && value.to_ascii_lowercase().ends_with(".gguf")
        && path.components().count() == 1
        && matches!(path.components().next(), Some(Component::Normal(_)))
}

/// Returns whether an image reference selects an immutable digest or non-floating tag.
fn is_pinned_image(value: &str) -> bool {
    if value.is_empty() || value.contains(char::is_whitespace) {
        return false;
    }
    if let Some((name, digest)) = value.split_once("@sha256:") {
        return !name.is_empty()
            && digest.len() == 64
            && digest
                .chars()
                .all(|character| character.is_ascii_hexdigit());
    }
    let final_segment = value.rsplit('/').next().unwrap_or(value);
    let Some((_, tag)) = final_segment.rsplit_once(':') else {
        return false;
    };
    !tag.is_empty()
        && !matches!(tag, "latest" | "main" | "master" | "nightly")
        && !tag.ends_with("-latest")
        && tag.chars().any(|character| character.is_ascii_digit())
}

/// Validates either the pinned GGUF identity or the required plain-model asset manifest.
fn verify_model_identity(
    model_directory: &std::path::Path,
    model_file: Option<&str>,
    format: LocalModelFormat,
    expected_bytes: u64,
    expected_sha256: &str,
) -> Result<(), AgentError> {
    match format {
        LocalModelFormat::Gguf => {
            let model_file = model_file.ok_or_else(|| {
                AgentError::InvalidRequest(
                    "local GGUF model file is required for GGUF format".to_owned(),
                )
            })?;
            verify_file_identity(
                &model_directory.join(model_file),
                expected_bytes,
                expected_sha256,
            )
        }
        LocalModelFormat::Plain => verify_plain_model_directory(model_directory),
    }
}

/// Verifies the minimum local asset manifest needed to load native safetensors weights.
fn verify_plain_model_directory(path: &std::path::Path) -> Result<(), AgentError> {
    let metadata = path.metadata().map_err(|error| {
        AgentError::Execution(format!(
            "could not inspect local plain model directory: {error}"
        ))
    })?;
    if !metadata.is_dir() {
        return Err(AgentError::InvalidRequest(
            "local plain model path must be a directory".to_owned(),
        ));
    }
    let config = path.join("config.json");
    if !config.is_file() {
        return Err(AgentError::InvalidRequest(
            "local plain model directory must contain config.json".to_owned(),
        ));
    }
    let has_safetensors = fs::read_dir(path)
        .map_err(|error| {
            AgentError::Execution(format!(
                "could not inspect local plain model directory: {error}"
            ))
        })?
        .filter_map(Result::ok)
        .any(|entry| {
            entry.file_type().is_ok_and(|file_type| file_type.is_file())
                && entry
                    .path()
                    .extension()
                    .is_some_and(|extension| extension.eq_ignore_ascii_case("safetensors"))
        });
    if !has_safetensors {
        return Err(AgentError::InvalidRequest(
            "local plain model directory must contain a safetensors file".to_owned(),
        ));
    }
    Ok(())
}

/// Streams one file through SHA-256 and validates both pinned identity dimensions.
fn verify_file_identity(
    path: &std::path::Path,
    expected_bytes: u64,
    expected_sha256: &str,
) -> Result<(), AgentError> {
    let metadata = path.metadata().map_err(|error| {
        AgentError::Execution(format!("could not inspect local model file: {error}"))
    })?;
    if !metadata.is_file() || metadata.len() != expected_bytes {
        return Err(AgentError::InvalidRequest(format!(
            "local model file must be a regular file of exactly {expected_bytes} bytes"
        )));
    }
    let mut file = File::open(path).map_err(|error| {
        AgentError::Execution(format!("could not open local model file: {error}"))
    })?;
    let mut hasher = Sha256::new();
    let mut buffer = [0_u8; 1024 * 1024];
    loop {
        let count = file.read(&mut buffer).map_err(|error| {
            AgentError::Execution(format!("could not verify local model file: {error}"))
        })?;
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
    }
    let actual = format!("{:x}", hasher.finalize());
    if !actual.eq_ignore_ascii_case(expected_sha256) {
        return Err(AgentError::InvalidRequest(
            "local model file checksum does not match the pinned release".to_owned(),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::{is_pinned_image, verify_file_identity, verify_plain_model_directory};

    /// Accepts exact digests and versioned tags while rejecting mutable or malformed references.
    #[test]
    fn runtime_image_identity_must_be_immutable() {
        assert!(is_pinned_image(
            "example.invalid/runtime@sha256:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
        ));
        assert!(is_pinned_image("example.invalid/runtime:cpu-0.9.0"));
        for image in [
            "example.invalid/runtime",
            "example.invalid/runtime:latest",
            "example.invalid/runtime:main",
            "example.invalid/runtime@sha256:abcd",
        ] {
            assert!(!is_pinned_image(image), "image should be rejected: {image}");
        }
    }

    /// Verifies both byte length and SHA-256 before a model file can reach Docker.
    #[test]
    fn model_identity_requires_exact_size_and_checksum() {
        let directory = tempfile::tempdir().expect("the fixture directory should exist");
        let path = directory.path().join("model.gguf");
        fs::write(&path, b"abc").expect("the fixture model should be written");

        verify_file_identity(
            &path,
            3,
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad",
        )
        .expect("the exact fixture identity should pass");
        assert!(
            verify_file_identity(
                &path,
                4,
                "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad",
            )
            .is_err()
        );
        assert!(verify_file_identity(&path, 3, &"0".repeat(64)).is_err());
    }

    /// Requires config metadata and at least one safetensors shard for plain mode.
    #[test]
    fn plain_model_identity_requires_native_assets() {
        let directory = tempfile::tempdir().expect("the fixture directory should exist");
        let error = verify_plain_model_directory(directory.path())
            .expect_err("plain models need a config manifest");
        assert!(error.to_string().contains("config.json"));

        fs::write(directory.path().join("config.json"), "{}")
            .expect("the config fixture should be written");
        let error = verify_plain_model_directory(directory.path())
            .expect_err("plain models need at least one safetensors shard");
        assert!(error.to_string().contains("safetensors"));

        fs::write(directory.path().join("model.safetensors"), b"fixture")
            .expect("the tensor fixture should be written");
        verify_plain_model_directory(directory.path())
            .expect("the minimum plain model manifest should pass");
    }
}
