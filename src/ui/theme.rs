/* --- LOONIX-TUNES src/ui/theme.rs --- */
use qmetaobject::*;
use serde_json::Value;
use std::collections::HashMap;
use std::process::Command;
use std::sync::{Arc, Mutex};

use crate::audio::config::{AppConfig, CustomTheme};

macro_rules! c {
    ($map:expr, { $($key:expr, $val:expr),* $(,)? }) => {
        $( $map.insert($key.to_string(), $val.to_string()); )*
    };
}

#[derive(QObject, Default)]
pub struct ThemeManager {
    base: qt_base_class!(trait QObject),
    pub colormap: qt_property!(QVariantMap; NOTIFY colormap_changed),
    pub colormap_changed: qt_signal!(),
    pub current_theme: qt_property!(QString; NOTIFY current_theme_changed),
    pub current_theme_changed: qt_signal!(),
    pub set_theme: qt_method!(fn(&mut self, name: String)),
    pub cycle_theme: qt_method!(fn(&mut self)),
    pub get_custom_theme_count: qt_method!(fn(&self) -> i32),
    pub get_custom_theme_name: qt_method!(fn(&self, index: i32) -> QString),
    pub set_custom_theme_name: qt_method!(fn(&mut self, index: i32, name: String)),
    pub custom_themes_changed: qt_signal!(),
    pub get_custom_theme_colors: qt_method!(fn(&self, index: i32) -> QVariantMap),
    pub set_custom_theme_colors: qt_method!(fn(&mut self, index: i32, colors: QVariantMap)),
    pub get_default_colors: qt_method!(fn(&self) -> QVariantMap),
    pub get_editor_starter_colors:
        qt_method!(fn(&self, is_edit_mode: bool, index: i32) -> QVariantMap),
    pub sync_with_wallpaper: qt_method!(fn(&mut self)),
    pub is_matugen_available: qt_method!(fn(&self) -> bool),

    custom_themes: Vec<CustomTheme>,
    current_raw_colors: HashMap<String, String>,
    config: Option<Arc<Mutex<AppConfig>>>,
    matugen_available: bool,
}

impl ThemeManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_config(&mut self, config: Arc<Mutex<AppConfig>>) {
        let cfg = config.lock().unwrap();

        let theme_name = if cfg.theme.is_empty() {
            "Default".to_string()
        } else {
            cfg.theme.clone()
        };

        let custom_themes = cfg.custom_themes.clone();
        drop(cfg);

        self.custom_themes = custom_themes;

        while self.custom_themes.len() < 3 {
            self.custom_themes.push(CustomTheme {
                name: format!("Custom {}", self.custom_themes.len() + 1),
                colors: HashMap::new(),
            });
        }

        self.config = Some(config);
        self.set_theme(theme_name);
        self.check_matugen();
    }

    fn check_matugen(&mut self) {
        let output = Command::new("matugen").arg("--version").output();
        self.matugen_available = output.map(|o| o.status.success()).unwrap_or(false);
    }

    pub fn is_matugen_available(&self) -> bool {
        self.matugen_available
    }

    fn fetch_matugen_colors(&mut self) -> Option<HashMap<String, String>> {
        let output = Command::new("matugen")
            .args(&["wallpaper", "--json", "hex"])
            .output()
            .ok()?;

        if !output.status.success() {
            return None;
        }

        let json_str = String::from_utf8_lossy(&output.stdout);
        let v: Value = serde_json::from_str(&json_str).ok()?;

        let colors = v["colors"]["dark"].as_object()?;

        let mut map = HashMap::new();

        let primary = colors.get("primary")?.as_str()?;
        let secondary = colors
            .get("secondary")
            .or(colors.get("tertiary"))?
            .as_str()?;
        let on_surface = colors.get("on_surface")?.as_str()?;
        let on_surface_variant = colors
            .get("on_surface_variant")
            .or(colors.get("on_surface"))?
            .as_str()?;
        let surface = colors
            .get("surface_container")
            .or(colors.get("surface"))
            .or(colors.get("background"))
            .or(colors.get("base"))?
            .as_str()?;
        let outline = colors
            .get("outline")
            .or(colors.get("outline_variant"))?
            .as_str()?;
        let secondary_or_tertiary = colors
            .get("secondary")
            .or(colors.get("tertiary"))?
            .as_str()?;

        map.insert("bgmain".to_string(), surface.to_string());
        map.insert("bgoverlay".to_string(), surface.to_string());
        map.insert("graysolid".to_string(), outline.to_string());
        map.insert("contextmenubg".to_string(), surface.to_string());
        map.insert("overlay".to_string(), outline.to_string());
        map.insert("headerbg".to_string(), surface.to_string());
        map.insert("headericon".to_string(), on_surface_variant.to_string());
        map.insert("headertext".to_string(), on_surface_variant.to_string());
        map.insert("headerhover".to_string(), primary.to_string());
        map.insert("playertitle".to_string(), on_surface.to_string());
        map.insert("playersubtext".to_string(), on_surface_variant.to_string());
        map.insert("playeraccent".to_string(), primary.to_string());
        map.insert("playerhover".to_string(), secondary.to_string());
        map.insert("tabtext".to_string(), on_surface.to_string());
        map.insert("tabborder".to_string(), outline.to_string());
        map.insert("tabhover".to_string(), primary.to_string());
        map.insert("playlisttext".to_string(), on_surface.to_string());
        map.insert("playlistfolder".to_string(), secondary.to_string());
        map.insert("playlistactive".to_string(), primary.to_string());
        map.insert("playlisticon".to_string(), secondary.to_string());
        map.insert("eqbg".to_string(), surface.to_string());
        map.insert("eqborder".to_string(), outline.to_string());
        map.insert("eqtext".to_string(), on_surface.to_string());
        map.insert("eqsubtext".to_string(), on_surface_variant.to_string());
        map.insert("eqicon".to_string(), secondary.to_string());
        map.insert("eqhover".to_string(), secondary.to_string());
        map.insert("eqpresettext".to_string(), on_surface_variant.to_string());
        map.insert("eqpresetactive".to_string(), primary.to_string());
        map.insert("eq10slider".to_string(), secondary.to_string());
        map.insert("eq10handle".to_string(), primary.to_string());
        map.insert("eq10bg".to_string(), "#111111".to_string());
        map.insert("eqfaderslider".to_string(), secondary.to_string());
        map.insert("eqfaderhandle".to_string(), primary.to_string());
        map.insert("eqfaderbg".to_string(), "#111111".to_string());
        map.insert("eqmixslider".to_string(), secondary.to_string());
        map.insert("eqmixhandle".to_string(), primary.to_string());
        map.insert("eqmixbg".to_string(), "#111111".to_string());
        map.insert("fxbg".to_string(), surface.to_string());
        map.insert("fxborder".to_string(), outline.to_string());
        map.insert("fxtext".to_string(), on_surface.to_string());
        map.insert("fxsubtext".to_string(), on_surface_variant.to_string());
        map.insert("fxicon".to_string(), secondary.to_string());
        map.insert("fxhover".to_string(), secondary.to_string());
        map.insert("fxactive".to_string(), primary.to_string());
        map.insert("fxslider".to_string(), primary.to_string());
        map.insert("fxsliderbg".to_string(), surface.to_string());
        map.insert("fxhandle".to_string(), secondary.to_string());

        Some(map)
    }

    pub fn sync_with_wallpaper(&mut self) {
        if let Some(new_colors) = self.fetch_matugen_colors() {
            let qmap: QVariantMap = new_colors
                .iter()
                .map(|(k, v)| {
                    (
                        QString::from(k.as_str()),
                        QVariant::from(QString::from(v.as_str())),
                    )
                })
                .collect();

            self.colormap = qmap;
            self.current_raw_colors = new_colors;
            self.colormap_changed();

            if let Some(ref config) = self.config {
                if let Ok(mut cfg) = config.lock() {
                    cfg.use_wallpaper_theme = true;
                    cfg.save();
                }
            }
        }
    }

    pub fn get_custom_theme_count(&self) -> i32 {
        self.custom_themes.len() as i32
    }

    pub fn get_custom_theme_name(&self, index: i32) -> QString {
        if index >= 0 && index < self.custom_themes.len() as i32 {
            QString::from(self.custom_themes[index as usize].name.as_str())
        } else {
            QString::from("")
        }
    }

    pub fn set_custom_theme_name(&mut self, index: i32, name: String) {
        if index >= 0 && index < self.custom_themes.len() as i32 {
            let old_name = self.custom_themes[index as usize].name.clone();
            let is_current_theme = old_name == self.current_theme.to_string();

            self.custom_themes[index as usize].name = name.clone();
            self.save_config();
            self.custom_themes_changed();

            // Smart Apply: Refresh UI if renaming the active theme
            if is_current_theme {
                self.set_theme(name);
            }
        }
    }

    pub fn get_custom_theme_colors(&self, index: i32) -> QVariantMap {
        if index >= 0 && index < self.custom_themes.len() as i32 {
            let colors = &self.custom_themes[index as usize].colors;
            if colors.is_empty() {
                return self
                    .current_raw_colors
                    .iter()
                    .map(|(k, v)| {
                        (
                            QString::from(k.as_str()),
                            QVariant::from(QString::from(v.as_str())),
                        )
                    })
                    .collect();
            }
            colors
                .iter()
                .map(|(k, v)| {
                    (
                        QString::from(k.as_str()),
                        QVariant::from(QString::from(v.as_str())),
                    )
                })
                .collect()
        } else {
            QVariantMap::default()
        }
    }

    pub fn set_custom_theme_colors(&mut self, index: i32, colors: QVariantMap) {
        let mut color_map: HashMap<String, String> = HashMap::new();
        for (k, v) in &colors {
            color_map.insert(k.to_string(), v.to_qstring().to_string());
        }

        let idx = index as usize;
        if idx < self.custom_themes.len() {
            self.custom_themes[idx].colors = color_map;
            self.save_config();
            self.custom_themes_changed();

            let theme_name = self.custom_themes[idx].name.clone();
            self.set_theme(theme_name);
        }
    }

    pub fn get_default_colors(&self) -> QVariantMap {
        AppConfig::default_theme_colors()
            .iter()
            .map(|(k, v)| {
                (
                    QString::from(k.as_str()),
                    QVariant::from(QString::from(v.as_str())),
                )
            })
            .collect()
    }

    pub fn get_editor_starter_colors(&self, is_edit_mode: bool, index: i32) -> QVariantMap {
        if is_edit_mode {
            if index >= 0 && index < self.custom_themes.len() as i32 {
                let colors = &self.custom_themes[index as usize].colors;
                if colors.is_empty() {
                    return self.get_default_colors();
                }
                return colors
                    .iter()
                    .map(|(k, v)| {
                        (
                            QString::from(k.as_str()),
                            QVariant::from(QString::from(v.as_str())),
                        )
                    })
                    .collect();
            }
        }
        self.current_raw_colors
            .iter()
            .map(|(k, v)| {
                (
                    QString::from(k.as_str()),
                    QVariant::from(QString::from(v.as_str())),
                )
            })
            .collect()
    }

    fn save_config(&self) {
        if let Some(ref config) = self.config {
            if let Ok(mut cfg) = config.lock() {
                cfg.theme = self.current_theme.to_string();
                cfg.custom_themes = self.custom_themes.clone();
                cfg.save();
            }
        }
    }

    pub fn available_themes() -> Vec<String> {
        let mut themes = vec![
            "Blue".into(),
            "Green".into(),
            "Monochrome".into(),
            "Orange".into(),
            "Pink".into(),
            "Red".into(),
            "Yellow".into(),
        ];
        themes.sort();
        themes.insert(0, "Default".into());
        themes
    }

    pub fn cycle_theme(&mut self) {
        let themes = Self::available_themes();
        let current = self.current_theme.to_string();
        if let Some(idx) = themes.iter().position(|t| t == &current) {
            let next_idx = (idx + 1) % themes.len();
            self.set_theme(themes[next_idx].clone());
        } else {
            self.set_theme("Default".to_string());
        }
    }

    pub fn set_theme(&mut self, name: String) {
        if let Some(custom) = self.custom_themes.iter().find(|t| t.name == name) {
            if !custom.colors.is_empty() {
                let qmap: QVariantMap = custom
                    .colors
                    .iter()
                    .map(|(k, v)| {
                        (
                            QString::from(k.as_str()),
                            QVariant::from(QString::from(v.as_str())),
                        )
                    })
                    .collect();

                self.colormap = qmap;
                self.current_theme = QString::from(name);
                self.colormap_changed();
                self.current_theme_changed();

                self.current_raw_colors = custom.colors.clone();
                self.save_config();
                return;
            }
        }

        let mut map: HashMap<String, String> = HashMap::new();

        match name.as_str() {
            "Blue" => {
                c!(map, {
                    "bgmain", "#121212",
                    "bgoverlay", "#1e1e1e",
                    "graysolid", "#333333",
                    "contextmenubg", "#181818",
                    "overlay", "#80000000",
                    "headerbg", "#1e1e1e",
                    "headericon", "#6d6d6d",
                    "headertext", "#6d6d6d",
                    "headerhover", "#00ddff",
                    "playertitle", "#00ffdd",
                    "playersubtext", "#6d6d6d",
                    "playeraccent", "#00ffdd",
                    "playerhover", "#843ff3",
                    "tabtext", "#d1d8e6",
                    "tabborder", "#8a8a8a",
                    "tabhover", "#00ffdd",
                    "playlisttext", "#d1d8e6",
                    "playlistfolder", "#f5a623",
                    "playlistactive", "#843ff3",
                    "playlisticon", "#00ffdd",
                    "eqbg", "#121212",
                    "eqborder", "#8a8a8a",
                    "eqtext", "#00e5ff",
                    "eqsubtext", "#6d6d6d",
                    "eqicon", "#00ffdd",
                    "eqhover", "#843ff3",
                    "eqpresettext", "#6d6d6d",
                    "eqpresetactive", "#00e5ff",
                    "eq10slider", "#843ff3",
                    "eq10handle", "#00ffdd",
                    "eq10bg", "#1e1e1e",
                    "eqfaderslider", "#f5a623",
                    "eqfaderhandle", "#8b0000",
                    "eqfaderbg", "#1e1e1e",
                    "eqmixslider", "#00ffdd",
                    "eqmixhandle", "#843ff3",
                    "eqmixbg", "#1e1e1e",
                    "fxbg", "#1e1e1e",
                    "fxborder", "#8a8a8a",
                    "fxtext", "#00e5ff",
                    "fxsubtext", "#6d6d6d",
                    "fxicon", "#00ffdd",
                    "fxhover", "#843ff3",
                    "fxactive", "#00e5ff",
                    "fxslider", "#00e5ff",
                    "fxsliderbg", "#121212",
                    "fxhandle", "#00ffdd",
                });
            }
            "Green" => {
                c!(map, {
                    "bgmain", "#121212",
                    "bgoverlay", "#1e1e1e",
                    "graysolid", "#333333",
                    "contextmenubg", "#181818",
                    "overlay", "#80000000",
                    "headerbg", "#1e1e1e",
                    "headericon", "#6d6d6d",
                    "headertext", "#6d6d6d",
                    "headerhover", "#00ff26",
                    "playertitle", "#00ff26",
                    "playersubtext", "#6d6d6d",
                    "playeraccent", "#00ff26",
                    "playerhover", "#ffcc00",
                    "tabtext", "#d1e6d8",
                    "tabborder", "#6d6d6d",
                    "tabhover", "#00ff26",
                    "playlisttext", "#d1e6d8",
                    "playlistfolder", "#00ff26",
                    "playlistactive", "#ffcc00",
                    "playlisticon", "#00ff26",
                    "eqbg", "#121c15",
                    "eqborder", "#6d6d6d",
                    "eqtext", "#00ff66",
                    "eqsubtext", "#6d6d6d",
                    "eqicon", "#00ff26",
                    "eqhover", "#ffcc00",
                    "eqpresettext", "#6d6d6d",
                    "eqpresetactive", "#00ff66",
                    "eq10slider", "#ffcc00",
                    "eq10handle", "#00ff26",
                    "eq10bg", "#1e1e1e",
                    "eqfaderslider", "#f5a623",
                    "eqfaderhandle", "#8b0000",
                    "eqfaderbg", "#1e1e1e",
                    "eqmixslider", "#00ff26",
                    "eqmixhandle", "#ffcc00",
                    "eqmixbg", "#1e1e1e",
                    "fxbg", "#1e1e1e",
                    "fxborder", "#6d6d6d",
                    "fxtext", "#00ff66",
                    "fxsubtext", "#6d6d6d",
                    "fxicon", "#00ff26",
                    "fxhover", "#ffcc00",
                    "fxactive", "#00ff66",
                    "fxslider", "#00ff66",
                    "fxsliderbg", "#121212",
                    "fxhandle", "#00ff26",
                });
            }
            "Monochrome" => {
                c!(map, {
                    "bgmain", "#121212",
                    "bgoverlay", "#1e1e1e",
                    "graysolid", "#333333",
                    "contextmenubg", "#181818",
                    "overlay", "#80000000",
                    "headerbg", "#1e1e1e",
                    "headericon", "#6d6d6d",
                    "headertext", "#6d6d6d",
                    "headerhover", "#ffffff",
                    "playertitle", "#ffffff",
                    "playersubtext", "#6d6d6d",
                    "playeraccent", "#555555",
                    "playerhover", "#ffffff",
                    "tabtext", "#e0e0e0",
                    "tabborder", "#ffffff",
                    "tabhover", "#ffffff",
                    "playlisttext", "#e0e0e0",
                    "playlistfolder", "#d4d4d4",
                    "playlistactive", "#ffffff",
                    "playlisticon", "#d4d4d4",
                    "eqbg", "#1e1e1e",
                    "eqborder", "#ffffff",
                    "eqtext", "#ffffff",
                    "eqsubtext", "#8b8b8b",
                    "eqicon", "#d4d4d4",
                    "eqhover", "#ffffff",
                    "eqpresettext", "#8b8b8b",
                    "eqpresetactive", "#ffffff",
                    "eq10slider", "#ffffff",
                    "eq10handle", "#555555",
                    "eq10bg", "#1e1e1e",
                    "eqfaderslider", "#f5a623",
                    "eqfaderhandle", "#8b0000",
                    "eqfaderbg", "#1e1e1e",
                    "eqmixslider", "#555555",
                    "eqmixhandle", "#ffffff",
                    "eqmixbg", "#1e1e1e",
                    "fxbg", "#1e1e1e",
                    "fxborder", "#ffffff",
                    "fxtext", "#ffffff",
                    "fxsubtext", "#8b8b8b",
                    "fxicon", "#d4d4d4",
                    "fxhover", "#ffffff",
                    "fxactive", "#ffffff",
                    "fxslider", "#ffffff",
                    "fxsliderbg", "#121212",
                    "fxhandle", "#d4d4d4",
                });
            }
            "Orange" => {
                c!(map, {
                    "bgmain", "#121212",
                    "bgoverlay", "#1e1e1e",
                    "graysolid", "#333333",
                    "contextmenubg", "#181818",
                    "overlay", "#80000000",
                    "headerbg", "#1e1e1e",
                    "headericon", "#6d6d6d",
                    "headertext", "#6d6d6d",
                    "headerhover", "#ffea00",
                    "playertitle", "#ff5500",
                    "playersubtext", "#6d6d6d",
                    "playeraccent", "#ff5500",
                    "playerhover", "#ffea00",
                    "tabtext", "#ecdcd9",
                    "tabborder", "#6d6d6d",
                    "tabhover", "#ff5500",
                    "playlisttext", "#ecdcd9",
                    "playlistfolder", "#ffea00",
                    "playlistactive", "#ff5500",
                    "playlisticon", "#ff5500",
                    "eqbg", "#1c1210",
                    "eqborder", "#6d6d6d",
                    "eqtext", "#ff5500",
                    "eqsubtext", "#6d6d6d",
                    "eqicon", "#ff5500",
                    "eqhover", "#ffea00",
                    "eqpresettext", "#6d6d6d",
                    "eqpresetactive", "#ff5500",
                    "eq10slider", "#ffea00",
                    "eq10handle", "#ff5500",
                    "eq10bg", "#1e1e1e",
                    "eqfaderslider", "#f5a623",
                    "eqfaderhandle", "#8b0000",
                    "eqfaderbg", "#1e1e1e",
                    "eqmixslider", "#ff5500",
                    "eqmixhandle", "#ffea00",
                    "eqmixbg", "#1e1e1e",
                    "fxbg", "#1e1e1e",
                    "fxborder", "#6d6d6d",
                    "fxtext", "#ff5500",
                    "fxsubtext", "#6d6d6d",
                    "fxicon", "#ff5500",
                    "fxhover", "#ffea00",
                    "fxactive", "#ff5500",
                    "fxslider", "#ff5500",
                    "fxsliderbg", "#121212",
                    "fxhandle", "#ff5500",
                });
            }
            "Pink" => {
                c!(map, {
                    "bgmain", "#121212",
                    "bgoverlay", "#1e1e1e",
                    "graysolid", "#333333",
                    "contextmenubg", "#181818",
                    "overlay", "#80000000",
                    "headerbg", "#1e1e1e",
                    "headericon", "#6d6d6d",
                    "headertext", "#6d6d6d",
                    "headerhover", "#00ffcc",
                    "playertitle", "#f965d9",
                    "playersubtext", "#6d6d6d",
                    "playeraccent", "#f965d9",
                    "playerhover", "#00ffcc",
                    "tabtext", "#eedef2",
                    "tabborder", "#6d6d6d",
                    "tabhover", "#f965d9",
                    "playlisttext", "#eedef2",
                    "playlistfolder", "#d59407",
                    "playlistactive", "#65f996",
                    "playlisticon", "#f965d9",
                    "eqbg", "#1b101f",
                    "eqborder", "#6d6d6d",
                    "eqtext", "#f965d9",
                    "eqsubtext", "#6d6d6d",
                    "eqicon", "#f965d9",
                    "eqhover", "#00ffcc",
                    "eqpresettext", "#6d6d6d",
                    "eqpresetactive", "#f965d9",
                    "eq10slider", "#00ffcc",
                    "eq10handle", "#f965d9",
                    "eq10bg", "#1e1e1e",
                    "eqfaderslider", "#f5a623",
                    "eqfaderhandle", "#8b0000",
                    "eqfaderbg", "#1e1e1e",
                    "eqmixslider", "#f965d9",
                    "eqmixhandle", "#00ffcc",
                    "eqmixbg", "#1e1e1e",
                    "fxbg", "#1e1e1e",
                    "fxborder", "#6d6d6d",
                    "fxtext", "#f965d9",
                    "fxsubtext", "#6d6d6d",
                    "fxicon", "#f965d9",
                    "fxhover", "#00ffcc",
                    "fxactive", "#f965d9",
                    "fxslider", "#f965d9",
                    "fxsliderbg", "#121212",
                    "fxhandle", "#00ffcc",
                });
            }
            "Red" => {
                c!(map, {
                    "bgmain", "#121212",
                    "bgoverlay", "#1e1e1e",
                    "graysolid", "#333333",
                    "contextmenubg", "#181818",
                    "overlay", "#80000000",
                    "headerbg", "#1e1e1e",
                    "headericon", "#6d6d6d",
                    "headertext", "#6d6d6d",
                    "headerhover", "#ff003c",
                    "playertitle", "#ff003c",
                    "playersubtext", "#bdbdbd",
                    "playeraccent", "#ff003c",
                    "playerhover", "#2b00ff",
                    "tabtext", "#bdbdbd",
                    "tabborder", "#6d6d6d",
                    "tabhover", "#ff003c",
                    "playlisttext", "#bdbdbd",
                    "playlistfolder", "#d59407",
                    "playlistactive", "#ff003c",
                    "playlisticon", "#2b00ff",
                    "eqbg", "#1c0d0d",
                    "eqborder", "#6d6d6d",
                    "eqtext", "#ff003c",
                    "eqsubtext", "#bdbdbd",
                    "eqicon", "#ff003c",
                    "eqhover", "#2b00ff",
                    "eqpresettext", "#bdbdbd",
                    "eqpresetactive", "#ff003c",
                    "eq10slider", "#2b00ff",
                    "eq10handle", "#ff003c",
                    "eq10bg", "#1e1e1e",
                    "eqfaderslider", "#f5a623",
                    "eqfaderhandle", "#8b0000",
                    "eqfaderbg", "#1e1e1e",
                    "eqmixslider", "#ff003c",
                    "eqmixhandle", "#2b00ff",
                    "eqmixbg", "#1e1e1e",
                    "fxbg", "#1e1e1e",
                    "fxborder", "#6d6d6d",
                    "fxtext", "#ff003c",
                    "fxsubtext", "#bdbdbd",
                    "fxicon", "#ff003c",
                    "fxhover", "#2b00ff",
                    "fxactive", "#ff003c",
                    "fxslider", "#ff003c",
                    "fxsliderbg", "#121212",
                    "fxhandle", "#2b00ff",
                });
            }
            "Yellow" => {
                c!(map, {
                    "bgmain", "#0d1012",
                    "bgoverlay", "#15191c",
                    "graysolid", "#2d353b",
                    "contextmenubg", "#0a0c0e",
                    "overlay", "#000000",
                    "headerbg", "#15191c",
                    "headericon", "#6d6d6d",
                    "headertext", "#6d6d6d",
                    "headerhover", "#f965d9",
                    "playertitle", "#ffea00",
                    "playersubtext", "#6d6d6d",
                    "playeraccent", "#ffea00",
                    "playerhover", "#f965d9",
                    "tabtext", "#dde0d1",
                    "tabborder", "#6d6d6d",
                    "tabhover", "#ffea00",
                    "playlisttext", "#dde0d1",
                    "playlistfolder", "#d59407",
                    "playlistactive", "#ffea00",
                    "playlisticon", "#f965d9",
                    "eqbg", "#15191c",
                    "eqborder", "#6d6d6d",
                    "eqtext", "#ffea00",
                    "eqsubtext", "#6d6d6d",
                    "eqicon", "#ffea00",
                    "eqhover", "#f965d9",
                    "eqpresettext", "#6d6d6d",
                    "eqpresetactive", "#ffea00",
                    "eq10slider", "#f965d9",
                    "eq10handle", "#ffea00",
                    "eq10bg", "#1e1e1e",
                    "eqfaderslider", "#f5a623",
                    "eqfaderhandle", "#8b0000",
                    "eqfaderbg", "#1e1e1e",
                    "eqmixslider", "#ffea00",
                    "eqmixhandle", "#f965d9",
                    "eqmixbg", "#1e1e1e",
                    "fxbg", "#15191c",
                    "fxborder", "#6d6d6d",
                    "fxtext", "#ffea00",
                    "fxsubtext", "#6d6d6d",
                    "fxicon", "#ffea00",
                    "fxhover", "#f965d9",
                    "fxactive", "#ffea00",
                    "fxslider", "#ffea00",
                    "fxsliderbg", "#0d1012",
                    "fxhandle", "#f965d9",
                });
            }
            _ => {
                map = AppConfig::default_theme_colors();
                // Save config when setting to Default or unknown theme
                self.save_config();
            }
        }

        let qmap: QVariantMap = map
            .iter()
            .map(|(k, v)| {
                (
                    QString::from(k.clone()),
                    QVariant::from(QString::from(v.clone())),
                )
            })
            .collect();

        self.colormap = qmap;
        self.current_theme = QString::from(name);
        self.colormap_changed();
        self.current_theme_changed();

        self.current_raw_colors = map
            .into_iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();

        // Save config for preset themes (Blue, Green, etc.)
        self.save_config();
    }
}
