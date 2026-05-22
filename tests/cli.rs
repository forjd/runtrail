use assert_cmd::Command;
use predicates::prelude::*;
use serde_json::Value;
use tempfile::tempdir;

#[test]
fn log_message_appends_event_and_prints_json() {
    let dir = tempdir().unwrap();
    let file = dir.path().join("events.jsonl");

    let output = Command::cargo_bin("cel")
        .unwrap()
        .args([
            "log",
            "--file",
            file.to_str().unwrap(),
            "--event",
            "agent.note",
            "--message",
            "hello",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let printed: Value = serde_json::from_slice(&output).unwrap();
    assert_eq!(printed["event"], "agent.note");
    assert_eq!(printed["body"]["message"], "hello");

    let raw = std::fs::read_to_string(file).unwrap();
    let stored: Value = serde_json::from_str(raw.trim()).unwrap();
    assert_eq!(stored["seq"], 1);
}

#[test]
fn log_parses_attrs_and_body_json() {
    let dir = tempdir().unwrap();
    let file = dir.path().join("nested/events.jsonl");

    Command::cargo_bin("cel")
        .unwrap()
        .args([
            "log",
            "--file",
            file.to_str().unwrap(),
            "--event",
            "command.run",
            "--attr",
            "exit_code=0",
            "--attr",
            "tool.name=terminal",
            "--body",
            r#"{"cmd":"cargo test"}"#,
        ])
        .assert()
        .success();

    let raw = std::fs::read_to_string(file).unwrap();
    let stored: Value = serde_json::from_str(raw.trim()).unwrap();
    assert_eq!(stored["attrs"]["exit_code"], 0);
    assert_eq!(stored["attrs"]["tool.name"], "terminal");
    assert_eq!(stored["body"]["cmd"], "cargo test");
}

#[test]
fn log_requires_event_name() {
    Command::cargo_bin("cel")
        .unwrap()
        .arg("log")
        .assert()
        .failure()
        .stderr(predicate::str::contains("event"));
}
