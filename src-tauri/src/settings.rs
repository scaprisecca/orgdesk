// Persisted app settings (currently just the watched-folder list), stored
// as JSON under the platform app-config directory. The `FileWatcher`'s
// in-memory `watched_folders` list is the runtime source of truth; this is
// only what's written to disk so folders are restored on the next launch.

use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Settings {
    pub watched_folders: Vec<String>,
    /// The `.org` file `create_task` appends new tasks to. `None` until the
    /// user picks one in Settings — `create_task` refuses to guess a path
    /// (see H3 in the code review: no more silently-relative `new_tasks.org`).
    #[serde(default)]
    pub inbox_file: Option<String>,
}

impl Settings {
    /// Loads settings from `path`, falling back to defaults if the file is
    /// missing or unreadable (e.g. first launch).
    pub fn load(path: &Path) -> Self {
        std::fs::read_to_string(path)
            .ok()
            .and_then(|contents| serde_json::from_str(&contents).ok())
            .unwrap_or_default()
    }

    pub fn save(&self, path: &Path) -> std::io::Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, serde_json::to_string_pretty(self)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_load_missing_file_returns_default() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("settings.json");
        let settings = Settings::load(&path);
        assert!(settings.watched_folders.is_empty());
    }

    #[test]
    fn test_save_then_load_round_trips() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("nested").join("settings.json");
        let settings = Settings {
            watched_folders: vec!["/home/user/org".to_string()],
            inbox_file: Some("/home/user/org/inbox.org".to_string()),
        };
        settings.save(&path).unwrap();

        let loaded = Settings::load(&path);
        assert_eq!(loaded.watched_folders, vec!["/home/user/org".to_string()]);
        assert_eq!(
            loaded.inbox_file.as_deref(),
            Some("/home/user/org/inbox.org")
        );
    }
}
