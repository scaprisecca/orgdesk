use crate::parser::org_parser::OrgParser;
use crate::store::task_store::TaskStore;
use notify::{RecommendedWatcher, Watcher};
use notify_debouncer_full::{new_debouncer, DebounceEventResult, Debouncer, FileIdMap};
use std::error::Error;
use std::fmt;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Duration;

/// Event name emitted (by the caller, via the `on_change` callback passed to
/// `FileWatcher::new`) whenever a debounced filesystem event causes the
/// `TaskStore` to change, so the frontend can refetch.
pub const TASKS_CHANGED_EVENT: &str = "tasks-changed";

#[derive(Debug)]
pub enum FileWatcherError {
    Io(std::io::Error),
    Notify(notify::Error),
}

impl fmt::Display for FileWatcherError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FileWatcherError::Io(err) => write!(f, "IO error: {}", err),
            FileWatcherError::Notify(err) => write!(f, "watch error: {}", err),
        }
    }
}

impl Error for FileWatcherError {}

impl From<std::io::Error> for FileWatcherError {
    fn from(err: std::io::Error) -> Self {
        FileWatcherError::Io(err)
    }
}

impl From<notify::Error> for FileWatcherError {
    fn from(err: notify::Error) -> Self {
        FileWatcherError::Notify(err)
    }
}

pub struct FileWatcher {
    task_store: Arc<Mutex<TaskStore>>,
    parser: Arc<OrgParser>,
    watcher: Debouncer<RecommendedWatcher, FileIdMap>,
    watched_folders: Vec<PathBuf>,
}

impl FileWatcher {
    /// `on_change` fires (off the debouncer's callback thread) after a
    /// batch of filesystem events changes the store. Kept generic rather
    /// than tauri-specific so this module doesn't need a running `AppHandle`
    /// to construct or test — the caller supplies e.g.
    /// `move || { let _ = app_handle.emit(TASKS_CHANGED_EVENT, ()); }`.
    pub fn new<F>(
        task_store: Arc<Mutex<TaskStore>>,
        parser: Arc<OrgParser>,
        on_change: F,
    ) -> Result<Self, notify::Error>
    where
        F: Fn() + Send + Sync + 'static,
    {
        let task_store_clone = Arc::clone(&task_store);
        let parser_clone = Arc::clone(&parser);

        let debouncer = new_debouncer(
            Duration::from_secs(1),
            None,
            move |res: DebounceEventResult| match res {
                Ok(events) => {
                    let mut changed = false;
                    for event in events {
                        for path in &event.event.paths {
                            if OrgParser::is_org_file(path) {
                                let mut store = task_store_clone.lock().unwrap();
                                match event.event.kind {
                                    notify::EventKind::Create(_)
                                    | notify::EventKind::Modify(_) => {
                                        match parser_clone.parse_file(path) {
                                            Ok(parsed_file) => {
                                                store.add_tasks_from_file(parsed_file);
                                                changed = true;
                                            }
                                            Err(e) => log::error!("Error parsing file: {:?}", e),
                                        }
                                    }
                                    notify::EventKind::Remove(_) => {
                                        let key = OrgParser::normalize_path(path);
                                        store.remove_tasks_by_file(&key);
                                        changed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                    if changed {
                        on_change();
                    }
                }
                Err(e) => log::error!("watch error: {:?}", e),
            },
        )?;

        Ok(Self {
            task_store,
            parser,
            watcher: debouncer,
            watched_folders: Vec::new(),
        })
    }

    /// Canonicalizes `path`, synchronously scans it recursively for `.org`
    /// files and parses each into the store, then starts watching it for
    /// future changes. No-op if `path` is already watched. Returns the
    /// canonical path that was added.
    pub fn add_watched_folder(&mut self, path: &Path) -> Result<PathBuf, FileWatcherError> {
        let canonical = std::fs::canonicalize(path)?;
        if self.watched_folders.contains(&canonical) {
            return Ok(canonical);
        }

        for file in Self::scan_org_files(&canonical) {
            match self.parser.parse_file(&file) {
                Ok(parsed_file) => {
                    self.task_store
                        .lock()
                        .unwrap()
                        .add_tasks_from_file(parsed_file);
                }
                Err(e) => log::error!("Error parsing {:?}: {:?}", file, e),
            }
        }

        self.watcher
            .watcher()
            .watch(&canonical, notify::RecursiveMode::Recursive)?;
        self.watched_folders.push(canonical.clone());
        Ok(canonical)
    }

    /// Stops watching `path` (matched after canonicalizing, falling back to
    /// the raw path if canonicalization fails) and drops every task parsed
    /// from underneath it. No-op if the folder isn't currently watched.
    pub fn remove_watched_folder(&mut self, path: &Path) {
        let target = std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
        let Some(idx) = self.watched_folders.iter().position(|p| p == &target) else {
            return;
        };
        let canonical = self.watched_folders.remove(idx);

        if let Err(e) = self.watcher.watcher().unwatch(&canonical) {
            log::error!("Failed to unwatch {:?}: {:?}", canonical, e);
        }

        let prefix = canonical.to_string_lossy().to_string();
        self.task_store
            .lock()
            .unwrap()
            .remove_tasks_by_folder(&prefix);
    }

    pub fn watched_folders(&self) -> Vec<String> {
        self.watched_folders
            .iter()
            .map(|p| p.to_string_lossy().to_string())
            .collect()
    }

    fn scan_org_files(dir: &Path) -> Vec<PathBuf> {
        let mut results = Vec::new();
        let entries = match std::fs::read_dir(dir) {
            Ok(entries) => entries,
            Err(e) => {
                log::error!("Failed to read dir {:?}: {:?}", dir, e);
                return results;
            }
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                results.extend(Self::scan_org_files(&path));
            } else if OrgParser::is_org_file(&path) {
                results.push(path);
            }
        }
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::org_parser::OrgParser;
    use crate::store::task_store::TaskStore;
    use std::fs;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::thread;
    use std::time::{Duration, Instant};
    use tempfile::tempdir;

    fn wait_until(mut condition: impl FnMut() -> bool, timeout: Duration) -> bool {
        let start = Instant::now();
        while start.elapsed() < timeout {
            if condition() {
                return true;
            }
            thread::sleep(Duration::from_millis(50));
        }
        condition()
    }

    #[test]
    fn test_file_watcher() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.org");
        fs::write(&file_path, "* TODO Task 1").unwrap();

        let task_store = Arc::new(Mutex::new(TaskStore::new()));
        let parser = Arc::new(OrgParser::new());

        let mut watcher =
            FileWatcher::new(Arc::clone(&task_store), Arc::clone(&parser), || {}).unwrap();
        watcher.add_watched_folder(dir.path()).unwrap();
        assert_eq!(task_store.lock().unwrap().get_all_tasks().len(), 1);

        // Modify file
        fs::write(&file_path, "* TODO Task 1\n* TODO Task 2").unwrap();
        assert!(wait_until(
            || task_store.lock().unwrap().get_all_tasks().len() == 2,
            Duration::from_secs(5)
        ));

        // Remove file
        fs::remove_file(&file_path).unwrap();
        assert!(wait_until(
            || task_store.lock().unwrap().get_all_tasks().is_empty(),
            Duration::from_secs(5)
        ));
    }

    #[test]
    fn test_add_watched_folder_scans_existing_files_and_watches_new_ones() {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("a.org"), "* TODO A\n").unwrap();
        let sub = dir.path().join("sub");
        fs::create_dir(&sub).unwrap();
        fs::write(sub.join("b.org"), "* TODO B\n").unwrap();

        let task_store = Arc::new(Mutex::new(TaskStore::new()));
        let parser = Arc::new(OrgParser::new());
        let mut watcher =
            FileWatcher::new(Arc::clone(&task_store), Arc::clone(&parser), || {}).unwrap();

        watcher.add_watched_folder(dir.path()).unwrap();
        assert_eq!(task_store.lock().unwrap().get_all_tasks().len(), 2);
        assert_eq!(watcher.watched_folders().len(), 1);

        // Adding again is a no-op, not a duplicate watch/rescan.
        watcher.add_watched_folder(dir.path()).unwrap();
        assert_eq!(watcher.watched_folders().len(), 1);

        // New file created after watching starts should still be picked up.
        fs::write(dir.path().join("c.org"), "* TODO C\n").unwrap();
        assert!(wait_until(
            || task_store.lock().unwrap().get_all_tasks().len() == 3,
            Duration::from_secs(5)
        ));
    }

    #[test]
    fn test_remove_watched_folder_stops_watching_and_drops_tasks() {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("a.org"), "* TODO A\n").unwrap();

        let task_store = Arc::new(Mutex::new(TaskStore::new()));
        let parser = Arc::new(OrgParser::new());
        let mut watcher =
            FileWatcher::new(Arc::clone(&task_store), Arc::clone(&parser), || {}).unwrap();

        watcher.add_watched_folder(dir.path()).unwrap();
        assert_eq!(task_store.lock().unwrap().get_all_tasks().len(), 1);

        watcher.remove_watched_folder(dir.path());
        assert!(watcher.watched_folders().is_empty());
        assert!(task_store.lock().unwrap().get_all_tasks().is_empty());

        // Further changes to the (now unwatched) folder must not reappear.
        fs::write(dir.path().join("d.org"), "* TODO D\n").unwrap();
        thread::sleep(Duration::from_millis(1500));
        assert!(task_store.lock().unwrap().get_all_tasks().is_empty());
    }

    #[test]
    fn test_on_change_callback_fires_after_debounced_mutation() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.org");
        fs::write(&file_path, "* TODO Task 1\n").unwrap();

        let task_store = Arc::new(Mutex::new(TaskStore::new()));
        let parser = Arc::new(OrgParser::new());
        let calls = Arc::new(AtomicUsize::new(0));
        let calls_clone = Arc::clone(&calls);

        let mut watcher = FileWatcher::new(Arc::clone(&task_store), Arc::clone(&parser), move || {
            calls_clone.fetch_add(1, Ordering::SeqCst);
        })
        .unwrap();
        watcher.add_watched_folder(dir.path()).unwrap();

        fs::write(&file_path, "* TODO Task 1\n* TODO Task 2\n").unwrap();
        assert!(wait_until(
            || calls.load(Ordering::SeqCst) > 0,
            Duration::from_secs(5)
        ));
    }

}
