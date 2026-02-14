use crate::icons::manager::IconManager;
use tauri::{State, command};

#[command]
pub async fn import_icon_theme(state: State<'_, IconManager>, file_path: String) -> Result<String, String> {
    state.import_theme(&file_path)
}

#[command]
pub async fn get_available_themes(state: State<'_, IconManager>) -> Result<Vec<crate::icons::manager::IconThemeInfo>, String> {
    Ok(state.get_available_themes())
}

#[command]
pub async fn set_active_theme(state: State<'_, IconManager>, theme_id: String) -> Result<(), String> {
    state.set_active_theme(theme_id)
}

#[command]
pub async fn get_file_icon(state: State<'_, IconManager>, filename: String, is_dir: bool, is_opened: bool) -> Result<Option<String>, String> {
    Ok(state.get_file_icon(&filename, is_dir, is_opened))
}
