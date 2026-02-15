pub mod meta_data;
pub mod searchengine_data;
pub mod settings_data;
pub mod logging;
pub mod watcher_state;

pub use settings_data::*;

use logging::Logger;
use crate::state::searchengine_data::SearchEngineState;
use meta_data::MetaDataState;
use watcher_state::WatcherState;
use std::sync::{Arc, Mutex};
use tauri::{Builder, Wry, Manager};

pub fn setup_app_state(app: Builder<Wry>) -> Builder<Wry> {
    // Create our shared state instances
    let meta_data_state = Arc::new(Mutex::new(MetaDataState::new()));
    let settings_state = Arc::new(Mutex::new(SettingsState::new()));
    let search_engine_state = Arc::new(Mutex::new(SearchEngineState::new(settings_state.clone())));
    let watcher_state = WatcherState::new();
    
    // Initialize the logger with the settings state
    Logger::init(settings_state.clone());
    
    //To add more just .manage
    app.manage(meta_data_state)
        .manage(settings_state)
        .manage(search_engine_state)
        .manage(watcher_state)
}
