use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::{Path, PathBuf};
use tauri::{AppHandle, Emitter, Runtime};

pub struct FileWatcher {
    watcher: RecommendedWatcher,
    watched_path: Option<PathBuf>,
}

impl FileWatcher {
    pub fn new<R: Runtime>(app: AppHandle<R>) -> Result<Self, notify::Error> {
        let app_handle = app.clone();

        let watcher = RecommendedWatcher::new(move |res: Result<Event, notify::Error>| {
            match res {
                Ok(event) => {
                    // Emit event to frontend
                    // We catch errors but don't crash
                    // Using "fs-change" as event name
                    if let Err(e) = app_handle.emit("fs-change", event) {
                        eprintln!("Failed to emit fs-change event: {}", e);
                    }
                },
                Err(e) => eprintln!("Watch error: {:?}", e),
            }
        }, Config::default())?;

        Ok(Self {
            watcher,
            watched_path: None,
        })
    }

    pub fn watch(&mut self, path: String) -> Result<(), notify::Error> {
        let new_path = PathBuf::from(&path);

        // If we are already watching this path, do nothing
        if let Some(current) = &self.watched_path {
            if *current == new_path {
                return Ok(());
            }
            // Unwatch old path
            // Ignore unwatch errors (e.g. if path was deleted)
            let _ = self.watcher.unwatch(current);
        }

        // Watch new path
        // NonRecursive is enough for the file list view
        self.watcher.watch(&new_path, RecursiveMode::NonRecursive)?;
        self.watched_path = Some(new_path);

        Ok(())
    }

    pub fn unwatch(&mut self) -> Result<(), notify::Error> {
        if let Some(current) = &self.watched_path {
            self.watcher.unwatch(current)?;
            self.watched_path = None;
        }
        Ok(())
    }
}
