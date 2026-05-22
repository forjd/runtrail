use serde_json::{Value, json};
use std::path::Path;
use std::process::{Command, ExitStatus};
use std::time::Instant;
use time::{OffsetDateTime, format_description::well_known::Rfc3339};

#[derive(Debug, Clone)]
pub struct CommandRunResult {
    pub start_body: Value,
    pub end_body: Value,
    pub exit_code: i32,
}

pub fn run_command(
    cwd: &Path,
    command: &[String],
    preview_bytes: usize,
) -> anyhow::Result<CommandRunResult> {
    anyhow::ensure!(!command.is_empty(), "command must not be empty");
    let started_at = OffsetDateTime::now_utc().format(&Rfc3339)?;
    let timer = Instant::now();
    let output = Command::new(&command[0])
        .args(&command[1..])
        .current_dir(cwd)
        .output()?;
    let duration_ms = timer.elapsed().as_millis() as u64;
    let exit_code = exit_code(output.status);
    let start_body = json!({
        "cmd": command,
        "cwd": cwd.display().to_string(),
        "started_at": started_at,
    });
    let end_body = json!({
        "cmd": command,
        "cwd": cwd.display().to_string(),
        "exit_code": exit_code,
        "success": output.status.success(),
        "duration_ms": duration_ms,
        "stdout_preview": preview(&output.stdout, preview_bytes),
        "stderr_preview": preview(&output.stderr, preview_bytes),
        "stdout_truncated": output.stdout.len() > preview_bytes,
        "stderr_truncated": output.stderr.len() > preview_bytes,
    });
    Ok(CommandRunResult {
        start_body,
        end_body,
        exit_code,
    })
}

fn exit_code(status: ExitStatus) -> i32 {
    status.code().unwrap_or(128)
}

fn preview(bytes: &[u8], limit: usize) -> String {
    let slice = if bytes.len() > limit {
        &bytes[..limit]
    } else {
        bytes
    };
    String::from_utf8_lossy(slice).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn run_command_captures_exit_code_and_output() {
        let dir = tempdir().unwrap();
        let command = vec![
            "sh".to_string(),
            "-c".to_string(),
            "printf hello".to_string(),
        ];
        let result = run_command(dir.path(), &command, 100).unwrap();
        assert_eq!(result.exit_code, 0);
        assert_eq!(result.end_body["stdout_preview"], "hello");
        assert_eq!(result.end_body["success"], true);
    }

    #[test]
    fn run_command_truncates_preview() {
        let dir = tempdir().unwrap();
        let command = vec![
            "sh".to_string(),
            "-c".to_string(),
            "printf 123456".to_string(),
        ];
        let result = run_command(dir.path(), &command, 3).unwrap();
        assert_eq!(result.end_body["stdout_preview"], "123");
        assert_eq!(result.end_body["stdout_truncated"], true);
    }
}
