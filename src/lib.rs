// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-journald: journald component model.
//!
//! Real implementation: journald_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JournaldItem {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JournaldError {
    EmptyJournaldItemName,
    DuplicateJournaldItem(String),
    EmptyEventSubject,
}

impl fmt::Display for JournaldError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyJournaldItemName => write!(f, "journald_item name must not be empty"),
            Self::DuplicateJournaldItem(id) => write!(f, "duplicate journald_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for JournaldError {}

pub struct Journald {
    pub name: String,
    journald_items: BTreeMap<String, JournaldItem>,
    events: BTreeMap<String, Event>,
}

impl Journald {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            journald_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_journald_item(&mut self, item: JournaldItem) -> Result<(), JournaldError> {
        if item.name.trim().is_empty() {
            return Err(JournaldError::EmptyJournaldItemName);
        }
        let id = item.id.clone();
        if self.journald_items.contains_key(&id) {
            return Err(JournaldError::DuplicateJournaldItem(id.clone()));
        }
        self.journald_items.insert(id, item);
        Ok(())
    }

    pub fn journald_item_count(&self) -> usize {
        self.journald_items.len()
    }

    pub fn get_journald_item(&self, id: &str) -> Option<&JournaldItem> {
        self.journald_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), JournaldError> {
        if event.subject.trim().is_empty() {
            return Err(JournaldError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_journald_item(id: &str, name: &str) -> JournaldItem {
        JournaldItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Journald::new("Journald");
        assert_eq!(m.name, "Journald");
        assert_eq!(m.journald_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_journald_item() {
        let mut m = Journald::new("Journald");
        m.add_journald_item(sample_journald_item("a1", "Alpha")).unwrap();
        assert_eq!(m.journald_item_count(), 1);
        assert!(m.get_journald_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Journald::new("Journald");
        assert!(m.add_journald_item(sample_journald_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Journald::new("Journald");
        m.add_journald_item(sample_journald_item("x", "One")).unwrap();
        assert!(m.add_journald_item(sample_journald_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Journald::new("Journald");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
