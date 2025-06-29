use crate::parser::org_parser::OrgParser;
use crate::store::task_store::TaskStore;
use notify::{RecommendedWatcher, Watcher};
use notify_debouncer_full::{new_debouncer, DebounceEventResult, Debouncer, FileIdMap};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub struct FileWatcher {
    _task_store: Arc<Mutex<TaskStore>>,
    _parser: Arc<OrgParser>,
    _watcher: Debouncer<RecommendedWatcher, FileIdMap>,
}

impl FileWatcher {
    pub fn new(
        task_store: Arc<Mutex<TaskStore>>,
        parser: Arc<OrgParser>,
    ) -> Result<Self, notify::Error> {
        let task_store_clone = Arc::clone(&task_store);
        let parser_clone = Arc::clone(&parser);

        let debouncer = new_debouncer(
            Duration::from_secs(1),
            None,
            move |res: DebounceEventResult| match res {
                Ok(events) => {
                    for event in events {
                        for path in event.event.paths {
                            if OrgParser::is_org_file(&path) {
                                let mut store = task_store_clone.lock().unwrap();
                                match event.event.kind {
                                    notify::EventKind::Create(_) | notify::EventKind::Modify(_) => {
                                        match parser_clone.parse_file(&path) {
                                            Ok(parsed_file) => {
                                                store.add_tasks_from_file(parsed_file)
                                            }
                                            Err(e) => eprintln!("Error parsing file: {:?}", e),
                                        }
                                    }
                                    notify::EventKind::Remove(_) => {
                                        store.remove_tasks_by_file(
                                            &path.to_string_lossy().to_string(),
                                        );
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                }
                Err(e) => eprintln!("watch error: {:?}", e),
            },
        )?;

        Ok(Self {
            _task_store: task_store,
            _parser: parser,
            _watcher: debouncer,
        })
    }

    pub fn watch_directory(&mut self, path: &PathBuf) -> Result<(), notify::Error> {
        self._watcher
            .watcher()
            .watch(path.as_ref(), notify::RecursiveMode::Recursive)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::org_parser::OrgParser;
    use crate::store::task_store::TaskStore;
    use std::fs;
    use std::thread;
    use std::time::Duration;
    use tempfile::tempdir;

    #[test]
    fn test_file_watcher() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.org");
        fs::write(&file_path, "* TODO Task 1").unwrap();

        let task_store = Arc::new(Mutex::new(TaskStore::new()));
        let parser = Arc::new(OrgParser::new());

        let mut watcher = FileWatcher::new(Arc::clone(&task_store), Arc::clone(&parser)).unwrap();
        watcher.watch_directory(&dir.path().to_path_buf()).unwrap();

        // Initial parse
        let parsed_file = parser.parse_file(&file_path).unwrap();
        task_store.lock().unwrap().add_tasks_from_file(parsed_file);
        assert_eq!(task_store.lock().unwrap().get_all_tasks().len(), 1);

        // Modify file
        fs::write(&file_path, "* TODO Task 1\n* TODO Task 2").unwrap();
        thread::sleep(Duration::from_secs(2)); // Allow time for debouncer to react
        assert_eq!(task_store.lock().unwrap().get_all_tasks().len(), 2);

        // Remove file
        fs::remove_file(&file_path).unwrap();
        thread::sleep(Duration::from_secs(2)); // Allow time for debouncer to react
        assert_eq!(task_store.lock().unwrap().get_all_tasks().len(), 0);
    }
} 