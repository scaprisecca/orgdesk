use crate::parser::org_parser::OrgHeadline;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TodoState {
    Todo,
    Done,
    InProgress,
    Someday,
    Canceled,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    A,
    B,
    C,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub state: TodoState,
    pub level: u8,
    pub tags: Vec<String>,
    pub priority: Option<Priority>,
    /// ISO `YYYY-MM-DD` (converted from the org timestamp at this
    /// boundary — see `org_timestamp_to_iso_date`), not the raw
    /// `<2024-08-01 Thu>` org syntax.
    pub scheduled: Option<String>,
    /// ISO `YYYY-MM-DD`, see `scheduled`.
    pub deadline: Option<String>,
    pub properties: std::collections::HashMap<String, String>,
    pub file_path: String,
}

/// Converts an org timestamp's raw text (e.g. `"<2024-08-01 Thu>"` or
/// `"[2024-08-01 Thu +1w]"`) into a plain ISO `YYYY-MM-DD` string for the
/// IPC wire format. The date always starts at the same offset right after
/// the opening bracket, so this doesn't need to re-parse the timestamp
/// through orgize — it just validates and extracts that substring.
///
/// Returns `None` if the text isn't in the expected org timestamp shape;
/// callers get no date rather than a malformed one.
fn org_timestamp_to_iso_date(raw: &str) -> Option<String> {
    let trimmed = raw.trim_start_matches(['<', '[']);
    let date_part = trimmed.get(0..10)?;
    chrono::NaiveDate::parse_from_str(date_part, "%Y-%m-%d").ok()?;
    Some(date_part.to_string())
}

/// Fixed namespace for deriving stable task ids (see `Task::new`). The
/// value itself is arbitrary — it only needs to never change, so the same
/// input always hashes to the same id, and to differ from `uuid`'s built-in
/// namespaces so OrgDesk ids can't collide with unrelated v5 UUIDs.
const TASK_ID_NAMESPACE: Uuid = Uuid::from_bytes([
    0x6f, 0x8f, 0x9b, 0x3a, 0x4b, 0x8b, 0x4c, 0x9a, 0x9a, 0x1a, 0x3b, 0x6e, 0x7f, 0x2d, 0x9c, 0x10,
]);

impl Task {
    /// Build a `Task` from a parsed headline with a stable `id`, so the
    /// same headline maps to the same id across reparses instead of a fresh
    /// `Uuid::new_v4()` every time (see H2 in the code review).
    ///
    /// Identity is derived, in order:
    /// 1. The headline's own `:ID:` property, if set — parsed directly as a
    ///    UUID when it already is one (this is what Emacs' `org-id` writes),
    ///    otherwise hashed into one so any custom string ID is still stable.
    /// 2. Otherwise, a hash of the file path plus the headline's outline
    ///    path (sibling-index chain from the root). This survives edits
    ///    elsewhere in the file, though not reordering of prior siblings —
    ///    good enough until headlines get real `:ID:`s written back (H3).
    pub fn new(headline: &OrgHeadline, file_path: &str) -> Self {
        let id = match headline.properties.get("ID") {
            Some(raw_id) => Uuid::parse_str(raw_id)
                .unwrap_or_else(|_| Uuid::new_v5(&TASK_ID_NAMESPACE, raw_id.as_bytes())),
            None => {
                let path = headline
                    .path
                    .iter()
                    .map(|n| n.to_string())
                    .collect::<Vec<_>>()
                    .join("/");
                Uuid::new_v5(&TASK_ID_NAMESPACE, format!("{file_path}#{path}").as_bytes())
            }
        };

        let state = match headline.todo_state.as_deref() {
            Some("TODO") => TodoState::Todo,
            Some("DONE") => TodoState::Done,
            Some("IN_PROGRESS") => TodoState::InProgress,
            Some("SOMEDAY") => TodoState::Someday,
            Some("CANCELED") => TodoState::Canceled,
            _ => TodoState::Todo, // Default state
        };
        let priority = match headline.priority {
            Some('A') => Some(Priority::A),
            Some('B') => Some(Priority::B),
            Some('C') => Some(Priority::C),
            _ => None,
        };

        Task {
            id,
            title: headline.title.clone(),
            state,
            level: headline.level,
            tags: headline.tags.clone(),
            priority,
            scheduled: headline
                .scheduled
                .as_deref()
                .and_then(org_timestamp_to_iso_date),
            deadline: headline
                .deadline
                .as_deref()
                .and_then(org_timestamp_to_iso_date),
            properties: headline.properties.clone(),
            file_path: file_path.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::org_parser::OrgParser;

    #[test]
    fn test_id_stable_across_reparse_without_id_property() {
        let parser = OrgParser::new();
        let content = "* TODO First task\n* TODO Second task\n";

        let first_parse = parser.parse_content(content).unwrap();
        let second_parse = parser.parse_content(content).unwrap();

        let t1a = Task::new(&first_parse[0], "notes.org");
        let t1b = Task::new(&second_parse[0], "notes.org");
        assert_eq!(t1a.id, t1b.id, "same headline should get the same id on reparse");

        let t2 = Task::new(&first_parse[1], "notes.org");
        assert_ne!(t1a.id, t2.id, "different headlines must not collide");
    }

    #[test]
    fn test_id_derived_from_id_property_when_present() {
        let parser = OrgParser::new();
        let content = "* TODO Task with id\n  :PROPERTIES:\n  :ID:       5b8f2a1e-4b7f-4a1e-9c3a-1234567890ab\n  :END:\n";
        let headlines = parser.parse_content(content).unwrap();

        let task = Task::new(&headlines[0], "notes.org");
        assert_eq!(
            task.id,
            Uuid::parse_str("5b8f2a1e-4b7f-4a1e-9c3a-1234567890ab").unwrap()
        );
    }

    #[test]
    fn test_state_serializes_to_screaming_snake_case() {
        assert_eq!(serde_json::to_string(&TodoState::Todo).unwrap(), "\"TODO\"");
        assert_eq!(serde_json::to_string(&TodoState::Done).unwrap(), "\"DONE\"");
        assert_eq!(
            serde_json::to_string(&TodoState::InProgress).unwrap(),
            "\"IN_PROGRESS\""
        );
        assert_eq!(
            serde_json::to_string(&TodoState::Someday).unwrap(),
            "\"SOMEDAY\""
        );
        assert_eq!(
            serde_json::to_string(&TodoState::Canceled).unwrap(),
            "\"CANCELED\""
        );
    }

    #[test]
    fn test_scheduled_and_deadline_convert_to_iso_dates() {
        let parser = OrgParser::new();
        let content = "* TODO Task\n  SCHEDULED: <2024-08-01 Thu> DEADLINE: <2024-08-15 Thu +1w>\n";
        let headlines = parser.parse_content(content).unwrap();

        let task = Task::new(&headlines[0], "notes.org");
        assert_eq!(task.scheduled.as_deref(), Some("2024-08-01"));
        assert_eq!(task.deadline.as_deref(), Some("2024-08-15"));
    }

    #[test]
    fn test_task_serializes_file_path_as_camel_case() {
        let parser = OrgParser::new();
        let headlines = parser.parse_content("* TODO Task\n").unwrap();
        let task = Task::new(&headlines[0], "notes.org");

        let json = serde_json::to_value(&task).unwrap();
        assert_eq!(json.get("filePath").unwrap(), "notes.org");
        assert!(json.get("file_path").is_none());
    }

    #[test]
    fn test_level_is_carried_from_headline() {
        let parser = OrgParser::new();
        let content = "* TODO Parent\n** TODO Child\n";
        let headlines = parser.parse_content(content).unwrap();

        assert_eq!(Task::new(&headlines[0], "notes.org").level, 1);
        assert_eq!(Task::new(&headlines[1], "notes.org").level, 2);
    }

    #[test]
    fn test_id_differs_across_files() {
        let parser = OrgParser::new();
        let content = "* TODO Same title, same position\n";
        let headlines = parser.parse_content(content).unwrap();

        let task_a = Task::new(&headlines[0], "a.org");
        let task_b = Task::new(&headlines[0], "b.org");
        assert_ne!(task_a.id, task_b.id);
    }
} 