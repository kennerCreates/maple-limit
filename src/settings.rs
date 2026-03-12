use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

use crate::grid::{GridConfig, GridStyle};
use crate::theme::{ThemeMapping, ThemeMode, ThemePalette};

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub theme_mode: ThemeMode,
    pub dark_palette: ThemePalette,
    pub dark_mapping: ThemeMapping,
    pub light_palette: ThemePalette,
    pub light_mapping: ThemeMapping,
    pub base_text_size: f32,
    pub grid_style: GridStyle,
    pub grid_size: f32,
    pub grid_visible: bool,
    pub grid_snap: bool,
}

impl Settings {
    pub fn from_app(
        theme_mode: ThemeMode,
        dark_palette: &ThemePalette,
        dark_mapping: &ThemeMapping,
        light_palette: &ThemePalette,
        light_mapping: &ThemeMapping,
        base_text_size: f32,
        grid: &GridConfig,
    ) -> Self {
        Self {
            theme_mode,
            dark_palette: dark_palette.clone(),
            dark_mapping: *dark_mapping,
            light_palette: light_palette.clone(),
            light_mapping: *light_mapping,
            base_text_size,
            grid_style: grid.style,
            grid_size: grid.size,
            grid_visible: grid.visible,
            grid_snap: grid.snap,
        }
    }

    pub fn save(&self) {
        if let Some(path) = settings_path() {
            if let Some(parent) = path.parent() {
                let _ = fs::create_dir_all(parent);
            }
            if let Ok(json) = serde_json::to_string_pretty(self) {
                let _ = fs::write(&path, json);
            }
        }
    }

    pub fn load() -> Option<Self> {
        let path = settings_path()?;
        let data = fs::read_to_string(&path).ok()?;
        serde_json::from_str(&data).ok()
    }
}

fn settings_path() -> Option<PathBuf> {
    let dirs = directories::ProjectDirs::from("", "", "maple-limit")?;
    Some(dirs.config_dir().join("settings.json"))
}
