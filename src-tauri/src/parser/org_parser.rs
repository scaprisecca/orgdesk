// Org file parser using orgize crate
// Handles reading and parsing .org files for task extraction

use orgize::{
    export::{from_fn, Container, Event},
    ParseConfig, TextRange,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParserError {
    #[error("IO error: {0}")]
    IO(#[from] io::Error),
    #[error("Parse error: {0}")]
    Parse(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ParsedOrgFile {
    pub file_path: String,
    pub content: String,
    pub headlines: Vec<OrgHeadline>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OrgHeadline {
    pub title: String,
    pub level: u8,
    pub todo_state: Option<String>,
    pub tags: Vec<String>,
    pub priority: Option<char>,
    pub scheduled: Option<String>,
    pub deadline: Option<String>,
    pub properties: HashMap<String, String>,
    /// Full range of this headline's subtree (header line, planning,
    /// properties, body text, and any nested child headlines).
    #[serde(skip)]
    pub range: Option<TextRange>,
    /// Range covering only the header line, planning (SCHEDULED/DEADLINE),
    /// and properties drawer — i.e. everything `to_org_string()` regenerates.
    /// This excludes the body section and child headlines, so replacing just
    /// this range (see `OrgParser::update_headline`) never deletes them.
    #[serde(skip)]
    pub header_range: Option<TextRange>,
    /// This headline's position in the outline: sibling index (1-based) at
    /// each level from the root down to this headline. E.g. the second
    /// child of the first top-level headline is `[1, 2]`. Combined with the
    /// file path, this gives a stable fallback identity for headlines that
    /// have no `:ID:` property (see `Task::new`) — it survives edits
    /// elsewhere in the file, though not reordering of preceding siblings.
    #[serde(skip)]
    pub path: Vec<usize>,
}

impl OrgHeadline {
    pub fn to_org_string(&self) -> String {
        let mut parts = Vec::new();
        parts.push("*".repeat(self.level as usize));
        if let Some(state) = &self.todo_state {
            parts.push(state.clone());
        }
        if let Some(priority) = self.priority {
            parts.push(format!("[#{}]", priority));
        }
        parts.push(self.title.clone());
        if !self.tags.is_empty() {
            parts.push(format!(":{}:", self.tags.join(":")));
        }
        let mut s = parts.join(" ");

        if let Some(scheduled) = &self.scheduled {
            s.push_str(&format!("\n  SCHEDULED: {}", scheduled));
        }
        if let Some(deadline) = &self.deadline {
            s.push_str(&format!("\n  DEADLINE: {}", deadline));
        }
        if !self.properties.is_empty() {
            s.push_str("\n  :PROPERTIES:");
            for (k, v) in &self.properties {
                s.push_str(&format!("\n  :{}:       {}", k, v));
            }
            s.push_str("\n  :END:");
        }
        s
    }
}

pub struct OrgParser;

impl OrgParser {
    /// Create a new OrgParser instance
    pub fn new() -> Self {
        Self
    }

    /// Check if a file has the .org extension (case-insensitive)
    pub fn is_org_file<P: AsRef<Path>>(path: P) -> bool {
        path.as_ref()
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.to_lowercase() == "org")
            .unwrap_or(false)
    }

    /// Best-effort normalized form of `path` for use as a stable `TaskStore`
    /// key. Canonicalizes `path` directly when it still exists (the common
    /// case: initial scan, create/modify events); when it doesn't (a Remove
    /// event, where the file itself is already gone) canonicalizes the
    /// parent directory instead and rejoins the file name, so the key still
    /// matches whatever `parse_file` inserted it under.
    pub fn normalize_path<P: AsRef<Path>>(path: P) -> String {
        let path = path.as_ref();
        if let Ok(canonical) = fs::canonicalize(path) {
            return canonical.to_string_lossy().to_string();
        }
        match (
            path.parent().and_then(|p| fs::canonicalize(p).ok()),
            path.file_name(),
        ) {
            (Some(parent), Some(name)) => parent.join(name).to_string_lossy().to_string(),
            _ => path.to_string_lossy().to_string(),
        }
    }

    /// Parse content string into a list of headlines using orgize
    pub fn parse_content(&self, content: &str) -> Result<Vec<OrgHeadline>, ParserError> {
        let config = ParseConfig {
            // Keyword lists derived from `TodoState::TODO_KEYWORD_TABLE` (see
            // M2 in the code review) so the parser, `TodoState::as_org_keyword`,
            // and `TodoState::from_org_keyword` can't drift out of sync.
            todo_keywords: crate::models::task::TodoState::keyword_lists(),
            ..Default::default()
        };
        let org = config.parse(content);
        let mut headlines = Vec::new();
        // Tracks the 1-based sibling index seen so far at each outline
        // depth, so each headline can be tagged with its path from the root.
        let mut sibling_counts: Vec<usize> = Vec::new();

        org.traverse(&mut from_fn(|event| {
            if let Event::Enter(Container::Headline(h)) = event {
                let level = h.level();
                if sibling_counts.len() < level {
                    sibling_counts.resize(level, 0);
                }
                sibling_counts.truncate(level);
                sibling_counts[level - 1] += 1;
                let path = sibling_counts.clone();

                let tags: Vec<String> = h.tags().map(|tag| tag.to_string()).collect();

                let properties: HashMap<String, String> = h
                    .properties()
                    .map(|p| {
                        p.to_hash_map()
                            .into_iter()
                            .map(|(k, v)| {
                                let k_range = k.text_range();
                                let v_range = v.text_range();
                                let k_str = &content
                                    [usize::from(k_range.start())..usize::from(k_range.end())];
                                let v_str = &content
                                    [usize::from(v_range.start())..usize::from(v_range.end())];
                                (k_str.to_string(), v_str.to_string())
                            })
                            .collect()
                    })
                    .unwrap_or_default();

                let (scheduled, deadline) = if let Some(planning) = h.planning() {
                    (
                        planning.scheduled().map(|ts| ts.raw().to_string()),
                        planning.deadline().map(|ts| ts.raw().to_string()),
                    )
                } else {
                    (None, None)
                };

                // The header ends where the body (SECTION) or the first
                // child headline begins, whichever comes first; if neither
                // exists, the headline's own range is just its header.
                let header_end = h
                    .section()
                    .map(|s| s.text_range().start())
                    .or_else(|| h.headlines().next().map(|child| child.text_range().start()))
                    .unwrap_or_else(|| h.text_range().end());
                let header_range = TextRange::new(h.text_range().start(), header_end);

                let headline = OrgHeadline {
                    title: h.title_raw().trim().to_string(),
                    level: level as u8,
                    todo_state: h.todo_keyword().map(|s| s.to_string()),
                    tags,
                    priority: h.priority().and_then(|p| {
                        let range = p.text_range();
                        let p_str =
                            &content[usize::from(range.start())..usize::from(range.end())];
                        p_str.chars().next()
                    }),
                    scheduled,
                    deadline,
                    properties,
                    range: Some(h.text_range()),
                    header_range: Some(header_range),
                    path,
                };
                headlines.push(headline);
            }
        }));

        Ok(headlines)
    }

    /// Parse an org file and return a ParsedOrgFile struct
    pub fn parse_file<P: AsRef<Path>>(&self, file_path: P) -> Result<ParsedOrgFile, ParserError> {
        let path = file_path.as_ref();
        
        // Read file content
        let content = fs::read_to_string(path)?;

        // Parse headlines
        let headlines = self.parse_content(&content)?;

        Ok(ParsedOrgFile {
            file_path: path.to_string_lossy().to_string(),
            content,
            headlines,
        })
    }

    /// Rewrite a headline's header (title/todo/priority/tags, planning, and
    /// properties drawer) in place, leaving its body text and any child
    /// headlines untouched.
    ///
    /// This replaces only `header_range`, not the full subtree `range` — the
    /// latter also covers the body section and nested headlines, and
    /// replacing it with `to_org_string()` (which only regenerates the
    /// header) would silently delete that content.
    pub fn update_headline<P: AsRef<Path>>(
        &self,
        file_path: P,
        old_headline: &OrgHeadline,
        new_headline: &OrgHeadline,
    ) -> Result<(), ParserError> {
        let (Some(header_range), Some(full_range)) =
            (old_headline.header_range, old_headline.range)
        else {
            return Err(ParserError::Parse(
                "Cannot update headline without a valid range".to_string(),
            ));
        };

        let mut org = orgize::Org::parse(&fs::read_to_string(&file_path)?);
        let mut replacement = new_headline.to_org_string();
        // header_range's end sits right before the body/child content that
        // follows it, on the same line as the newline that separates them;
        // preserve that newline unless the header ran to the end of the
        // subtree (no body, no children).
        if header_range.end() < full_range.end() {
            replacement.push('\n');
        }
        org.replace_range(header_range, &replacement);
        fs::write(file_path, org.to_org())?;
        Ok(())
    }

    /// Removes a headline's entire subtree (header, planning, properties,
    /// body text, and any nested child headlines — i.e. `range`, not just
    /// `header_range`) from the file. Unlike `update_headline`, deleting the
    /// full subtree is exactly what's wanted here.
    pub fn delete_headline<P: AsRef<Path>>(
        &self,
        file_path: P,
        headline: &OrgHeadline,
    ) -> Result<(), ParserError> {
        let Some(range) = headline.range else {
            return Err(ParserError::Parse(
                "Cannot delete headline without a valid range".to_string(),
            ));
        };

        let mut org = orgize::Org::parse(&fs::read_to_string(&file_path)?);
        org.replace_range(range, "");
        fs::write(file_path, org.to_org())?;
        Ok(())
    }
}

impl Default for OrgParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_org_file() {
        assert!(OrgParser::is_org_file("test.org"));
        assert!(OrgParser::is_org_file("test.ORG"));
        assert!(!OrgParser::is_org_file("test.txt"));
        assert!(!OrgParser::is_org_file("test"));
    }

    #[test]
    fn test_parse_content_fully() {
        let parser = OrgParser::new();
        let content = r#"
* TODO [#A] First task :work:
  SCHEDULED: <2024-08-01 Thu>
  :PROPERTIES:
  :ID:       task1
  :END:
** DONE Second task
* [#B] Another task :personal:home:
  DEADLINE: <2024-08-15 Thu>
* SOMEDAY Learn new technology :learning:
"#;

        let result = parser.parse_content(content);
        assert!(result.is_ok(), "Parsing failed: {:?}", result.err());
        let headlines = result.unwrap();

        assert_eq!(headlines.len(), 4);

        let h1 = &headlines[0];
        assert_eq!(h1.title, "First task");
        assert_eq!(h1.level, 1);
        assert_eq!(h1.todo_state, Some("TODO".to_string()));
        assert_eq!(h1.priority, Some('A'));
        assert_eq!(h1.tags, vec!["work".to_string()]);
        assert_eq!(h1.scheduled, Some("<2024-08-01 Thu>".to_string()));
        assert!(h1.deadline.is_none());
        assert_eq!(h1.properties.get("ID").unwrap(), "task1");
        assert!(h1.range.is_some());

        let h2 = &headlines[1];
        assert_eq!(h2.title, "Second task");
        assert_eq!(h2.level, 2);
        assert_eq!(h2.todo_state, Some("DONE".to_string()));
        assert!(h2.priority.is_none());
        assert!(h2.tags.is_empty());
        assert!(h2.scheduled.is_none());
        assert!(h2.deadline.is_none());
        assert!(h2.properties.is_empty());
        assert!(h2.range.is_some());

        let h3 = &headlines[2];
        assert_eq!(h3.title, "Another task");
        assert_eq!(h3.level, 1);
        assert!(h3.todo_state.is_none());
        assert_eq!(h3.priority, Some('B'));
        assert_eq!(h3.tags, vec!["personal".to_string(), "home".to_string()]);
        assert!(h3.scheduled.is_none());
        assert_eq!(h3.deadline, Some("<2024-08-15 Thu>".to_string()));
        assert!(h3.properties.is_empty());
        assert!(h3.range.is_some());

        let h4 = &headlines[3];
        assert_eq!(h4.title, "Learn new technology");
        assert_eq!(h4.level, 1);
        assert_eq!(h4.todo_state, Some("SOMEDAY".to_string()));
        assert!(h4.priority.is_none());
        assert_eq!(h4.tags, vec!["learning".to_string()]);
        assert!(h4.scheduled.is_none());
        assert!(h4.deadline.is_none());
        assert!(h4.properties.is_empty());
        assert!(h4.range.is_some());
    }

    #[test]
    fn test_parse_content_recognizes_in_progress_and_canceled_keywords() {
        let parser = OrgParser::new();
        let content = "* IN_PROGRESS Working on it\n* CANCELED Nevermind\n";
        let headlines = parser.parse_content(content).unwrap();

        assert_eq!(headlines.len(), 2);
        assert_eq!(headlines[0].todo_state, Some("IN_PROGRESS".to_string()));
        assert_eq!(headlines[0].title, "Working on it");
        assert_eq!(headlines[1].todo_state, Some("CANCELED".to_string()));
        assert_eq!(headlines[1].title, "Nevermind");
    }

    #[test]
    fn test_update_headline() {
        let parser = OrgParser::new();
        let file_content = r#"
* TODO [#A] First task :work:
  SCHEDULED: <2024-08-01 Thu>
  :PROPERTIES:
  :ID:       task1
  :END:
* SOMEDAY Learn new technology :learning:
"#;
        let file_path = "test_update.org";
        fs::write(file_path, file_content).unwrap();

        let parsed_file = parser.parse_file(file_path).unwrap();
        let old_headline = parsed_file.headlines[0].clone();

        let mut new_headline = old_headline.clone();
        new_headline.title = "Updated First task".to_string();
        new_headline.todo_state = Some("DONE".to_string());

        let result = parser.update_headline(file_path, &old_headline, &new_headline);
        assert!(result.is_ok());

        let updated_parsed_file = parser.parse_file(file_path).unwrap();
        let updated_headline = &updated_parsed_file.headlines[0];

        assert_eq!(updated_headline.title, "Updated First task");
        assert_eq!(updated_headline.todo_state, Some("DONE".to_string()));

        fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_update_headline_preserves_body_text() {
        let parser = OrgParser::new();
        let file_content = "* TODO Task\n  Some notes that must survive.\n";
        let file_path = "test_update_body.org";
        fs::write(file_path, file_content).unwrap();

        let parsed_file = parser.parse_file(file_path).unwrap();
        let old_headline = parsed_file.headlines[0].clone();

        let mut new_headline = old_headline.clone();
        new_headline.title = "Renamed task".to_string();

        parser
            .update_headline(file_path, &old_headline, &new_headline)
            .unwrap();

        let updated_content = fs::read_to_string(file_path).unwrap();
        assert!(updated_content.contains("Renamed task"));
        assert!(
            updated_content.contains("Some notes that must survive."),
            "body text was lost: {updated_content:?}"
        );

        fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_update_headline_preserves_child_headlines() {
        let parser = OrgParser::new();
        let file_content = "* TODO Parent\n** TODO Child\n";
        let file_path = "test_update_child.org";
        fs::write(file_path, file_content).unwrap();

        let parsed_file = parser.parse_file(file_path).unwrap();
        let old_headline = parsed_file.headlines[0].clone();
        assert_eq!(old_headline.title, "Parent");

        let mut new_headline = old_headline.clone();
        new_headline.title = "Renamed parent".to_string();

        parser
            .update_headline(file_path, &old_headline, &new_headline)
            .unwrap();

        let updated_content = fs::read_to_string(file_path).unwrap();
        assert!(updated_content.contains("Renamed parent"));
        assert!(
            updated_content.contains("** TODO Child"),
            "child headline was lost: {updated_content:?}"
        );

        let reparsed = parser.parse_file(file_path).unwrap();
        assert_eq!(reparsed.headlines.len(), 2);
        assert_eq!(reparsed.headlines[1].title, "Child");

        fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_delete_headline_removes_subtree_only() {
        let parser = OrgParser::new();
        let file_content = "* TODO Keep me\n* TODO Delete me\n  Some notes.\n** TODO Child\n* TODO Keep me too\n";
        let file_path = "test_delete.org";
        fs::write(file_path, file_content).unwrap();

        let parsed_file = parser.parse_file(file_path).unwrap();
        let to_delete = parsed_file.headlines[1].clone();
        assert_eq!(to_delete.title, "Delete me");

        parser.delete_headline(file_path, &to_delete).unwrap();

        let updated_content = fs::read_to_string(file_path).unwrap();
        assert!(!updated_content.contains("Delete me"));
        assert!(!updated_content.contains("Some notes."));
        assert!(!updated_content.contains("Child"));
        assert!(updated_content.contains("Keep me"));
        assert!(updated_content.contains("Keep me too"));

        let reparsed = parser.parse_file(file_path).unwrap();
        assert_eq!(reparsed.headlines.len(), 2);
        assert_eq!(reparsed.headlines[0].title, "Keep me");
        assert_eq!(reparsed.headlines[1].title, "Keep me too");

        fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_parse_non_existent_file() {
        let parser = OrgParser::new();
        let result = parser.parse_file("non_existent_file.org");
        assert!(result.is_err());
        match result.err().unwrap() {
            ParserError::IO(_) => (), // Expected error
            _ => panic!("Expected IO error"),
        }
    }
} 