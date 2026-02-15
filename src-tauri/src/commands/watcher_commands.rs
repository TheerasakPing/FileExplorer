use crate::state::watcher_state::WatcherState;
use crate::filesystem::watcher::FileWatcher;
use tauri::{AppHandle, State, Runtime};

#[tauri::command]
pub async fn watch_directory<R: Runtime>(
    app: AppHandle<R>,
    state: State<'_, WatcherState>,
    path: String,
) -> Result<(), String> {
    let mut watcher_guard = state.watcher.lock().map_err(|e| e.to_string())?;

    if watcher_guard.is_none() {
        match FileWatcher::new(app) {
            Ok(w) => *watcher_guard = Some(w),
            Err(e) => return Err(format!("Failed to create watcher: {}", e)),
        }
    }

    if let Some(watcher) = watcher_guard.as_mut() {
        watcher.watch(path).map_err(|e| format!("Failed to watch path: {}", e))?;
    }

    Ok(())
}

#[tauri::command]
pub async fn unwatch_directory(
    state: State<'_, WatcherState>,
) -> Result<(), String> {
    let mut watcher_guard = state.watcher.lock().map_err(|e| e.to_string())?;

    if let Some(watcher) = watcher_guard.as_mut() {
        watcher.unwatch().map_err(|e| format!("Failed to unwatch: {}", e))?;
    }

    Ok(())
}
