use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use serde::{Serialize};
use crate::icons::parser::{IconTheme, VscodeIconTheme};
use zip::ZipArchive;
use walkdir::WalkDir;
use uuid::Uuid;
use regex::Regex;

#[derive(Clone, Serialize)]
pub struct IconThemeInfo {
    pub id: String,
    pub name: String,
}

#[derive(Clone)]
pub struct IconManager {
    themes: Arc<Mutex<HashMap<String, IconTheme>>>,
    active_theme_id: Arc<Mutex<Option<String>>>,
    themes_storage_dir: PathBuf,
}

impl IconManager {
    pub fn new(app_data_dir: PathBuf) -> Self {
        let themes_storage_dir = app_data_dir.join("icon_themes");
        if let Err(e) = fs::create_dir_all(&themes_storage_dir) {
            eprintln!("Failed to create themes directory: {}", e);
        }

        Self {
            themes: Arc::new(Mutex::new(HashMap::new())),
            active_theme_id: Arc::new(Mutex::new(None)),
            themes_storage_dir,
        }
    }

    pub fn import_theme(&self, file_path: &str) -> Result<String, String> {
        let file = fs::File::open(file_path).map_err(|e| format!("Failed to open file: {}", e))?;
        let mut archive = ZipArchive::new(file).map_err(|e| format!("Failed to read zip: {}", e))?;

        // Generate a unique ID for the new theme
        let theme_id = Uuid::new_v4().to_string();
        let extract_path = self.themes_storage_dir.join(&theme_id);

        if let Err(e) = fs::create_dir_all(&extract_path) {
            return Err(format!("Failed to create extraction directory: {}", e));
        }

        if let Err(e) = archive.extract(&extract_path) {
            // Cleanup on failure
            let _ = fs::remove_dir_all(&extract_path);
            return Err(format!("Failed to extract zip: {}", e));
        }

        // Search for package.json to identify the theme
        let (manifest_path, package_json) = self.find_package_json(&extract_path)?;

        // Find the icon theme contribution point
        let contributes = package_json.get("contributes")
            .ok_or("No 'contributes' section in package.json")?;

        let icon_themes = contributes.get("iconThemes")
            .ok_or("No 'iconThemes' in 'contributes'")?
            .as_array()
            .ok_or("'iconThemes' is not an array")?;

        if icon_themes.is_empty() {
             return Err("No icon themes found in package".to_string());
        }

        // For now, we just pick the first one
        let theme_def = &icon_themes[0];
        let label = theme_def.get("label").and_then(|v| v.as_str()).unwrap_or("Unknown Theme");
        let path_val = theme_def.get("path").and_then(|v| v.as_str())
            .ok_or("Theme definition missing 'path'")?;

        // The path in package.json is relative to package.json location
        let theme_json_path = manifest_path.parent().unwrap().join(path_val);

        let theme_content = fs::read_to_string(&theme_json_path)
            .map_err(|e| format!("Failed to read theme json: {}", e))?;

        // Remove comments from JSON (VSCode allows comments in JSON)
        let json_clean = self.strip_json_comments(&theme_content);

        let config: VscodeIconTheme = serde_json::from_str(&json_clean)
            .map_err(|e| format!("Failed to parse theme json: {}", e))?;

        let theme = IconTheme {
            id: theme_id.clone(),
            name: label.to_string(),
            base_path: theme_json_path.parent().unwrap().to_string_lossy().to_string(),
            config,
        };

        {
            let mut themes = self.themes.lock().unwrap();
            themes.insert(theme_id.clone(), theme);
        }

        Ok(theme_id)
    }

    fn find_package_json(&self, root: &Path) -> Result<(PathBuf, serde_json::Value), String> {
        for entry in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
            if entry.file_name() == "package.json" {
                let content = fs::read_to_string(entry.path())
                    .map_err(|e| e.to_string())?;
                let json: serde_json::Value = match serde_json::from_str(&self.strip_json_comments(&content)) {
                    Ok(v) => v,
                    Err(e) => return Err(format!("Failed to parse package.json: {}", e)),
                };
                return Ok((entry.path().to_path_buf(), json));
            }
        }
        Err("package.json not found".to_string())
    }

    fn strip_json_comments(&self, json: &str) -> String {
        // Simple regex-based stripper to handle // and /* */ comments
        let re_block = Regex::new(r"/\*[\s\S]*?\*/").unwrap();
        let re_line = Regex::new(r"//.*").unwrap();

        let no_block = re_block.replace_all(json, "");
        let no_line = re_line.replace_all(&no_block, "");

        no_line.to_string()
    }

    pub fn get_available_themes(&self) -> Vec<IconThemeInfo> {
        let themes = self.themes.lock().unwrap();
        themes.values().map(|t| IconThemeInfo {
            id: t.id.clone(),
            name: t.name.clone(),
        }).collect()
    }

    pub fn set_active_theme(&self, theme_id: String) -> Result<(), String> {
        let themes = self.themes.lock().unwrap();
        if themes.contains_key(&theme_id) {
            *self.active_theme_id.lock().unwrap() = Some(theme_id);
            Ok(())
        } else {
            Err("Theme not found".to_string())
        }
    }

    pub fn get_file_icon(&self, filename: &str, is_dir: bool, is_opened: bool) -> Option<String> {
        let active_id_guard = self.active_theme_id.lock().unwrap();
        let active_id = active_id_guard.as_ref()?;

        let themes = self.themes.lock().unwrap();
        let theme = themes.get(active_id)?;
        let config = &theme.config;

        let icon_name = if is_dir {
            // Check folder names
            let folder_name = filename; // Assuming filename is folder name here

            // Check folderExpanded if opened
            if is_opened {
                if let Some(expanded) = &config.folder_names_expanded {
                    if let Some(icon) = expanded.get(folder_name) {
                        return Some(icon.clone());
                    }
                }
            }

            // Check specific folder names
            if let Some(folders) = &config.folder_names {
                if let Some(icon) = folders.get(folder_name) {
                     return Some(icon.clone());
                }
            }

            // Default folder icon
            if is_opened {
                 config.folder_expanded.clone().or(config.folder.clone())
            } else {
                 config.folder.clone()
            }
        } else {
            // Check file names
            if let Some(names) = &config.file_names {
                if let Some(icon) = names.get(filename) {
                     return Some(icon.clone());
                }
            }

            // Check extensions
            let ext = Path::new(filename).extension().and_then(|e| e.to_str()).unwrap_or("");
            if !ext.is_empty() {
                 if let Some(extensions) = &config.file_extensions {
                     if let Some(icon) = extensions.get(ext) {
                          return Some(icon.clone());
                     }
                 }
            }

            // Default file icon
            config.file.clone()
        };

        if let Some(icon_ref) = icon_name {
            return self.resolve_icon_path(&theme.base_path, config, &icon_ref);
        }

        None
    }

    fn resolve_icon_path(&self, base_path: &str, config: &VscodeIconTheme, icon_ref: &str) -> Option<String> {
        if let Some(defs) = &config.icon_definitions {
            if let Some(def) = defs.get(icon_ref) {
                if let Some(path) = &def.icon_path {
                    // Combine base_path and relative icon path
                    let full_path = Path::new(base_path).join(path);
                    return Some(full_path.to_string_lossy().to_string());
                }
            }
        }
        None
    }
}
