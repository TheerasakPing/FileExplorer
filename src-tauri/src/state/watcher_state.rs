use crate::filesystem::watcher::FileWatcher;
use std::sync::Mutex;

pub struct WatcherState {
    pub watcher: Mutex<Option<FileWatcher>>,
}

impl WatcherState {
    pub fn new() -> Self {
        Self {
            watcher: Mutex::new(None),
        }
    }
}
