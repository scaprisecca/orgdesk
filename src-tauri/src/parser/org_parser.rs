// Org file parser using orgize crate
// Handles reading and parsing .org files for task extraction

use orgize::{
    export::{from_fn, Container, Event},
    ParseConfig, TextRange,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fs;
use std::io;
use std::path::Path;

#[derive(Debug)]
pub enum ParserError {
    IO(io::Error),
    Parse(String),
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::IO(err) => write!(f, "IO error: {}", err),
            ParserError::Parse(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}

impl Error for ParserError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ParserError::IO(err) => Some(err),
            ParserError::Parse(_) => None,
        }
    }
}

impl From<io::Error> for ParserError {
    fn from(err: io::Error) -> Self {
        ParserError::IO(err)
    }
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
    #[serde(skip)]
    pub range: Option<TextRange>,
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

    /// Parse content string into a list of headlines using orgize
    pub fn parse_content(&self, content: &str) -> Result<Vec<OrgHeadline>, ParserError> {
        let config = ParseConfig {
            todo_keywords: (
                vec!["TODO".to_string(), "SOMEDAY".to_string()],
                vec!["DONE".to_string()],
            ),
            ..Default::default()
        };
        let org = config.parse(content);
        let mut headlines = Vec::new();

        org.traverse(&mut from_fn(|event| {
            if let Event::Enter(Container::Headline(h)) = event {
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

                let headline = OrgHeadline {
                    title: h.title_raw().trim().to_string(),
                    level: h.level() as u8,
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

    pub fn update_headline<P: AsRef<Path>>(
        &self,
        file_path: P,
        old_headline: &OrgHeadline,
        new_headline: &OrgHeadline,
    ) -> Result<(), ParserError> {
        if let Some(range) = old_headline.range {
            let mut org = orgize::Org::parse(&fs::read_to_string(&file_path)?);
            org.replace_range(range, &new_headline.to_org_string());
            fs::write(file_path, org.to_org())?;
            Ok(())
        } else {
            Err(ParserError::Parse(
                "Cannot update headline without a valid range".to_string(),
            ))
        }
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