use crate::parser::org_parser::OrgHeadline;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub state: TodoState,
    pub tags: Vec<String>,
    pub priority: Option<Priority>,
    pub scheduled: Option<String>,
    pub deadline: Option<String>,
    pub properties: std::collections::HashMap<String, String>,
    pub file_path: String,
}

impl From<&OrgHeadline> for Task {
    fn from(headline: &OrgHeadline) -> Self {
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
            id: Uuid::new_v4(),
            title: headline.title.clone(),
            state,
            tags: headline.tags.clone(),
            priority,
            scheduled: headline.scheduled.clone(),
            deadline: headline.deadline.clone(),
            properties: headline.properties.clone(),
            file_path: String::new(), // Will be set when the task is added to the store
        }
    }
} 