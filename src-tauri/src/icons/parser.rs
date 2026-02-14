use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VscodeIconTheme {
    pub icon_definitions: Option<HashMap<String, IconDefinition>>,
    pub file_extensions: Option<HashMap<String, String>>,
    pub file_names: Option<HashMap<String, String>>,
    pub folder_names: Option<HashMap<String, String>>,
    pub folder_names_expanded: Option<HashMap<String, String>>,
    pub language_ids: Option<HashMap<String, String>>,
    pub light: Option<Box<VscodeIconTheme>>,
    pub high_contrast: Option<Box<VscodeIconTheme>>,
    pub file: Option<String>,
    pub folder: Option<String>,
    pub folder_expanded: Option<String>,
    pub root_folder: Option<String>,
    pub root_folder_expanded: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IconDefinition {
    pub icon_path: Option<String>,
    pub font_character: Option<String>,
    pub font_color: Option<String>,
    pub font_size: Option<String>,
    pub font_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IconTheme {
    pub id: String,
    pub name: String,     // Human readable name
    pub base_path: String, // Directory where icon files are relative to
    pub config: VscodeIconTheme,
}
