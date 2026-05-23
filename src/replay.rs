use crate::event::Event;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayHint {
    pub command: Vec<String>,
    pub supported: bool,
    pub partial: bool,
    pub unsupported_reason: Option<String>,
}

pub fn hints(events: &[Event]) -> Vec<ReplayHint> {
    events
        .iter()
        .filter(|event| event.event == "command.end")
        .filter_map(|event| {
            let command = event
                .body
                .get("cmd")
                .and_then(|value| value.as_array())?
                .iter()
                .filter_map(|part| part.as_str().map(str::to_string))
                .collect::<Vec<_>>();
            if command.is_empty() {
                return None;
            }
            let unsupported_reason = unsupported_reason(&command);
            Some(ReplayHint {
                command,
                supported: unsupported_reason.is_none(),
                partial: unsupported_reason.is_some(),
                unsupported_reason,
            })
        })
        .collect()
}

pub fn to_markdown(hints: &[ReplayHint]) -> String {
    let mut output = String::new();
    output.push_str("# runtrail Replay Hints\n\n");
    if hints.is_empty() {
        output.push_str("No replayable command events were found.\n");
        return output;
    }
    for hint in hints {
        let status = if hint.supported {
            "supported"
        } else {
            "partial"
        };
        output.push_str(&format!("- `{}` — {status}", shell_join(&hint.command)));
        if let Some(reason) = &hint.unsupported_reason {
            output.push_str(&format!(" ({reason})"));
        }
        output.push('\n');
    }
    output.push_str("\n```bash\n");
    for hint in hints.iter().filter(|hint| hint.supported) {
        output.push_str(&shell_join(&hint.command));
        output.push('\n');
    }
    output.push_str("```\n");
    output
}

fn unsupported_reason(command: &[String]) -> Option<String> {
    let joined = command.join(" ");
    if joined.contains("docker compose") || joined.contains("services:") {
        return Some("requires external services not captured by runtrail".to_string());
    }
    if command.first().is_some_and(|bin| bin == "act") {
        return Some("requires act and GitHub Actions runner parity".to_string());
    }
    None
}

fn shell_join(command: &[String]) -> String {
    command
        .iter()
        .map(|part| {
            if part
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || "._/-".contains(c))
            {
                part.clone()
            } else {
                format!("'{}'", part.replace('\'', "'\\''"))
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::{Event, Level, NewEvent};
    use serde_json::{Map, json};

    #[test]
    fn hints_include_supported_and_partial_metadata() {
        let event = Event::new(NewEvent {
            seq: 1,
            event: "command.end".to_string(),
            level: Level::Error,
            src: Some("runtrail".to_string()),
            attrs: Map::new(),
            body: json!({"cmd":["cargo","test"],"exit_code":101}),
            trace_id: None,
            span_id: None,
            parent_span_id: None,
            duration_ms: None,
        });
        let hints = hints(&[event]);
        assert_eq!(hints.len(), 1);
        assert!(hints[0].supported);
        assert!(!hints[0].partial);
        assert!(to_markdown(&hints).contains("cargo test"));
    }
}
