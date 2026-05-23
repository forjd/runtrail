use crate::event::Event;
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct IndexEntry {
    pub seq: u64,
    pub id: String,
    pub event: String,
    pub level: String,
    pub ts: String,
    pub trace_id: Option<String>,
}

pub fn build_index(events: &[Event]) -> Vec<IndexEntry> {
    events
        .iter()
        .map(|event| IndexEntry {
            seq: event.seq,
            id: event.id.clone(),
            event: event.event.clone(),
            level: format!("{:?}", event.level).to_ascii_lowercase(),
            ts: event.ts.clone(),
            trace_id: event.trace_id.clone(),
        })
        .collect()
}

pub fn counts_by_event(entries: &[IndexEntry]) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for entry in entries {
        *counts.entry(entry.event.clone()).or_insert(0) += 1;
    }
    counts
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::{Event, Level, NewEvent};
    use serde_json::{Map, Value};

    #[test]
    fn build_index_extracts_query_fields() {
        let event = Event::new(NewEvent {
            seq: 7,
            event: "agent.note".to_string(),
            level: Level::Warn,
            src: None,
            attrs: Map::new(),
            body: Value::Null,
            trace_id: Some("0123456789abcdef0123456789abcdef".to_string()),
            span_id: None,
            parent_span_id: None,
            duration_ms: None,
        });
        let entries = build_index(&[event]);
        assert_eq!(entries[0].seq, 7);
        assert_eq!(entries[0].event, "agent.note");
        assert_eq!(entries[0].level, "warn");
    }
}
