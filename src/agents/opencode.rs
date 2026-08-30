use std::env;
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;

use serde_json::Value;

use super::{ActivityEvent, ActivityKind, AgentError};

/// Resolves the native OpenCode CLI program used by child processes on the current host.
pub fn resolve_opencode_executable(configured: Option<OsString>) -> Result<PathBuf, AgentError> {
    if let Some(configured) = configured {
        let path = PathBuf::from(configured);
        if path.is_file() {
            return validate_native_executable(path);
        }
        return Err(AgentError::Execution(format!(
            "OPENCODE_BIN does not point to a file: {}",
            path.display()
        )));
    }
    let path = env::var_os("PATH").ok_or_else(missing_opencode_error)?;
    resolve_opencode_from_path(&path).ok_or_else(missing_opencode_error)
}

/// Rejects Windows shell shims because the subprocess adapter requires a native executable.
fn validate_native_executable(path: PathBuf) -> Result<PathBuf, AgentError> {
    #[cfg(windows)]
    if path
        .extension()
        .and_then(OsStr::to_str)
        .is_none_or(|extension| !extension.eq_ignore_ascii_case("exe"))
    {
        return Err(AgentError::Execution(format!(
            "OPENCODE_BIN must point to opencode.exe on Windows, not a shell shim: {}",
            path.display()
        )));
    }
    Ok(path)
}

/// Searches PATH for the native OpenCode executable used on the current platform.
#[cfg(windows)]
fn resolve_opencode_from_path(path: &OsStr) -> Option<PathBuf> {
    env::split_paths(path)
        .map(|directory| directory.join("opencode.exe"))
        .find(|candidate| candidate.is_file())
}

/// Searches PATH for the extensionless OpenCode executable used on Unix hosts.
#[cfg(not(windows))]
fn resolve_opencode_from_path(path: &OsStr) -> Option<PathBuf> {
    env::split_paths(path)
        .map(|directory| directory.join("opencode"))
        .find(|candidate| candidate.is_file())
}

/// Creates a stable setup error that explains how to select the required OpenCode program.
fn missing_opencode_error() -> AgentError {
    AgentError::Execution(
        "could not locate the OpenCode CLI executable; install OpenCode or set OPENCODE_BIN to its full path"
            .to_owned(),
    )
}

/// Converts one documented OpenCode JSON event into stable, non-sensitive activity.
pub fn decode_opencode_jsonl_event(
    line: &str,
    sequence: u32,
) -> Result<Option<ActivityEvent>, AgentError> {
    let event: Value =
        serde_json::from_str(line).map_err(|error| AgentError::InvalidEvent(error.to_string()))?;
    let Some(event_type) = event.get("type").and_then(Value::as_str) else {
        return Ok(None);
    };
    let mapped = match event_type {
        "step_start" => Some((
            ActivityKind::Progress,
            "OpenCode started a work step".to_owned(),
        )),
        "step_finish" => Some((
            ActivityKind::Progress,
            "OpenCode completed a work step".to_owned(),
        )),
        "tool_use" => opencode_tool_message(&event),
        "error" => Some((
            ActivityKind::Warning,
            "OpenCode reported an error".to_owned(),
        )),
        _ => None,
    };
    Ok(mapped.map(|(kind, message)| ActivityEvent::now(sequence, kind, &message)))
}

/// Maps tool completion without exposing provider-controlled arguments or output.
fn opencode_tool_message(event: &Value) -> Option<(ActivityKind, String)> {
    event
        .pointer("/part/tool")
        .and_then(Value::as_str)
        .filter(|tool| !tool.is_empty())
        .map(|tool| {
            (
                ActivityKind::Progress,
                format!(
                    "OpenCode completed tool: {}",
                    super::sanitize_terminal_text(tool)
                ),
            )
        })
}
