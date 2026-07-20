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

/// Single source of truth for the org todo keywords this app recognizes: the
/// keyword text, the `TodoState` it maps to, and whether it counts as
/// "done". `OrgParser::parse_content`'s `ParseConfig` keyword lists,
/// `TodoState::as_org_keyword`, and `TodoState::from_org_keyword` all derive
/// from this table so the three can't drift out of sync (see M2 in the code
/// review — previously the parser only recognized TODO/SOMEDAY/DONE, so
/// IN_PROGRESS/CANCELED could never be produced even though `TodoState` had
/// variants for them).
pub const TODO_KEYWORD_TABLE: &[(&str, TodoState, bool)] = &[
    ("TODO", TodoState::Todo, false),
    ("IN_PROGRESS", TodoState::InProgress, false),
    ("SOMEDAY", TodoState::Someday, false),
    ("DONE", TodoState::Done, true),
    ("CANCELED", TodoState::Canceled, true),
];

impl TodoState {
    /// The org todo keyword this state round-trips to. Used when writing a
    /// task's state back into an `OrgHeadline` for `update_task`.
    pub fn as_org_keyword(&self) -> &'static str {
        TODO_KEYWORD_TABLE
            .iter()
            .find(|(_, state, _)| state == self)
            .map(|(keyword, _, _)| *keyword)
            .expect("every TodoState variant has a TODO_KEYWORD_TABLE entry")
    }

    /// The `TodoState` a parsed org todo keyword maps to, or `None` if it
    /// isn't one of the keywords this app recognizes.
    pub fn from_org_keyword(keyword: &str) -> Option<Self> {
        TODO_KEYWORD_TABLE
            .iter()
            .find(|(k, _, _)| *k == keyword)
            .map(|(_, state, _)| state.clone())
    }

    /// The `(not-done, done)` keyword lists `ParseConfig::todo_keywords`
    /// needs, derived from `TODO_KEYWORD_TABLE`.
    pub fn keyword_lists() -> (Vec<String>, Vec<String>) {
        let mut not_done = Vec::new();
        let mut done = Vec::new();
        for (keyword, _, is_done) in TODO_KEYWORD_TABLE {
            if *is_done {
                done.push(keyword.to_string());
            } else {
                not_done.push(keyword.to_string());
            }
        }
        (not_done, done)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    A,
    B,
    C,
}

impl Priority {
    /// The org priority letter this round-trips to, matching the parsing in
    /// `Task::new`'s `match headline.priority`.
    pub fn as_char(&self) -> char {
        match self {
            Priority::A => 'A',
            Priority::B => 'B',
            Priority::C => 'C',
        }
    }
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

        let state = headline
            .todo_state
            .as_deref()
            .and_then(TodoState::from_org_keyword)
            .unwrap_or(TodoState::Todo); // Default state (also covers no-keyword headlines)
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
    fn test_todo_state_as_org_keyword() {
        assert_eq!(TodoState::Todo.as_org_keyword(), "TODO");
        assert_eq!(TodoState::Done.as_org_keyword(), "DONE");
        assert_eq!(TodoState::InProgress.as_org_keyword(), "IN_PROGRESS");
        assert_eq!(TodoState::Someday.as_org_keyword(), "SOMEDAY");
        assert_eq!(TodoState::Canceled.as_org_keyword(), "CANCELED");
    }

    #[test]
    fn test_todo_state_round_trips_through_org_keyword() {
        for state in [
            TodoState::Todo,
            TodoState::Done,
            TodoState::InProgress,
            TodoState::Someday,
            TodoState::Canceled,
        ] {
            let keyword = state.as_org_keyword();
            let content = format!("* {} Task\n", keyword);
            let headlines = OrgParser::new().parse_content(&content).unwrap();
            assert_eq!(
                Task::new(&headlines[0], "notes.org").state,
                state,
                "state {:?} did not round-trip through keyword {:?}",
                state,
                keyword
            );
        }
    }

    #[test]
    fn test_priority_round_trips_through_org_char() {
        for priority in [Priority::A, Priority::B, Priority::C] {
            let content = format!("* TODO [#{}] Task\n", priority.as_char());
            let headlines = OrgParser::new().parse_content(&content).unwrap();
            assert_eq!(
                Task::new(&headlines[0], "notes.org").priority,
                Some(priority.clone()),
                "priority {:?} did not round-trip",
                priority
            );
        }
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