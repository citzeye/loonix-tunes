/* --- LOONIX-TUNES src/ui/theme.rs --- */
use qmetaobject::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

macro_rules! c {
    ($map:expr, { $($key:literal, $val:literal),* $(,)? }) => {
        $( $map.insert($key, $val); )*
    };
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AppConfig {
    pub theme: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            theme: "Default".to_string(),
        }
    }
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
}

impl ThemeManager {
    pub fn new() -> Self {
        let mut s = Self::default();
        let cfg: AppConfig = confy::load("loonix-tunes", "config").unwrap_or_default();

        let theme_name = if cfg.theme.is_empty() {
            "Default".to_string()
        } else {
            cfg.theme
        };
        s.set_theme(theme_name);
        s
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
        let cfg = AppConfig {
            theme: name.clone(),
        };
        let _ = confy::store("loonix-tunes", "config", cfg);

        let mut map: HashMap<&str, &str> = HashMap::new();

        match name.as_str() {
            "Blue" => {
                c!(map, {
                    // BACKGROUNDS
                    "bgmain", "#121212",
                    "bgoverlay", "#1e1e1e",
                    "graysolid", "#333333",
                    "contextmenubg", "#181818",
                    "overlay", "#80000000",
                    // HEADER
                    "headerbg", "#1e1e1e",
                    "headericon", "#6d6d6d",
                    "headertext", "#6d6d6d",
                    "headerhover", "#00ddff",
                    // PLAYER
                    "playertitle", "#00ffdd",
                    "playersubtext", "#6d6d6d",
                    "playeraccent", "#00ffdd",
                    "playerhover", "#843ff3",
                    // TABS
                    "tabtext", "#d1d8e6",
                    "tabborder", "#8a8a8a",
                    "tabhover", "#00ffdd",
                    // PLAYLIST
                    "playlisttext", "#d1d8e6",
                    "playlistfolder", "#f5a623",
                    "playlistactive", "#843ff3",
                    "playlisticon", "#00ffdd",
                    // EQ
                    "eqbg", "#121a2f",
                    "eqborder", "#8a8a8a",
                    "eqtext", "#00e5ff",
                    "eqsubtext", "#6d6d6d",
                    "eqicon", "#00ffdd",
                    "eqhover", "#843ff3",
                    "eqactive", "#00e5ff",
                    "eqsliderbg", "#121a2f",
                    "eqfader", "#f5a623",
                    "eqmix", "#843ff3",
                    "eqhandle", "#00ffdd",
                    // FX
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
                    // BACKGROUNDS
                    "bgmain", "#121212",
                    "bgoverlay", "#1e1e1e",
                    "graysolid", "#333333",
                    "contextmenubg", "#181818",
                    "overlay", "#80000000",
                    // HEADER
                    "headerbg", "#1e1e1e",
                    "headericon", "#6d6d6d",
                    "headertext", "#6d6d6d",
                    "headerhover", "#00ff26",
                    // PLAYER
                    "playertitle", "#00ff26",
                    "playersubtext", "#6d6d6d",
                    "playeraccent", "#00ff26",
                    "playerhover", "#ffcc00",
                    // TABS
                    "tabtext", "#d1e6d8",
                    "tabborder", "#6d6d6d",
                    "tabhover", "#00ff26",
                    // PLAYLIST
                    "playlisttext", "#d1e6d8",
                    "playlistfolder", "#00ff26",
                    "playlistactive", "#ffcc00",
                    "playlisticon", "#00ff26",
                    // EQ
                    "eqbg", "#121c15",
                    "eqborder", "#6d6d6d",
                    "eqtext", "#00ff66",
                    "eqsubtext", "#6d6d6d",
                    "eqicon", "#00ff26",
                    "eqhover", "#ffcc00",
                    "eqactive", "#00ff66",
                    "eqsliderbg", "#121c15",
                    "eqfader", "#ff3300",
                    "eqmix", "#ff00ff",
                    "eqhandle", "#00ff26",
                    // FX
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
                    // BACKGROUNDS
                    "bgmain", "#121212",
                    "bgoverlay", "#1e1e1e",
                    "graysolid", "#333333",
                    "contextmenubg", "#181818",
                    "overlay", "#80000000",
                    // HEADER
                    "headerbg", "#1e1e1e",
                    "headericon", "#6d6d6d",
                    "headertext", "#6d6d6d",
                    "headerhover", "#ffffff",
                    // PLAYER
                    "playertitle", "#ffffff",
                    "playersubtext", "#6d6d6d",
                    "playeraccent", "#555555",
                    "playerhover", "#ffffff",
                    // TABS
                    "tabtext", "#e0e0e0",
                    "tabborder", "#ffffff",
                    "tabhover", "#ffffff",
                    // PLAYLIST
                    "playlisttext", "#e0e0e0",
                    "playlistfolder", "#d4d4d4",
                    "playlistactive", "#ffffff",
                    "playlisticon", "#d4d4d4",
                    // EQ
                    "eqbg", "#1e1e1e",
                    "eqborder", "#ffffff",
                    "eqtext", "#ffffff",
                    "eqsubtext", "#8b8b8b",
                    "eqicon", "#d4d4d4",
                    "eqhover", "#ffffff",
                    "eqactive", "#ffffff",
                    "eqsliderbg", "#1e1e1e",
                    "eqfader", "#d4d4d4",
                    "eqmix", "#ffffff",
                    "eqhandle", "#d4d4d4",
                    // FX
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
                    // BACKGROUNDS
                    "bgmain", "#121212",
                    "bgoverlay", "#1e1e1e",
                    "graysolid", "#333333",
                    "contextmenubg", "#181818",
                    "overlay", "#80000000",
                    // HEADER
                    "headerbg", "#1e1e1e",
                    "headericon", "#6d6d6d",
                    "headertext", "#6d6d6d",
                    "headerhover", "#ffea00",
                    // PLAYER
                    "playertitle", "#ff5500",
                    "playersubtext", "#6d6d6d",
                    "playeraccent", "#ff5500",
                    "playerhover", "#ffea00",
                    // TABS
                    "tabtext", "#ecdcd9",
                    "tabborder", "#6d6d6d",
                    "tabhover", "#ff5500",
                    // PLAYLIST
                    "playlisttext", "#ecdcd9",
                    "playlistfolder", "#ffea00",
                    "playlistactive", "#ff5500",
                    "playlisticon", "#ff5500",
                    // EQ
                    "eqbg", "#1c1210",
                    "eqborder", "#6d6d6d",
                    "eqtext", "#ff5500",
                    "eqsubtext", "#6d6d6d",
                    "eqicon", "#ff5500",
                    "eqhover", "#ffea00",
                    "eqactive", "#ff5500",
                    "eqsliderbg", "#1c1210",
                    "eqfader", "#00e5ff",
                    "eqmix", "#ffea00",
                    "eqhandle", "#ff5500",
                    // FX
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
                    // BACKGROUNDS
                    "bgmain", "#121212",
                    "bgoverlay", "#1e1e1e",
                    "graysolid", "#333333",
                    "contextmenubg", "#181818",
                    "overlay", "#80000000",
                    // HEADER
                    "headerbg", "#1e1e1e",
                    "headericon", "#6d6d6d",
                    "headertext", "#6d6d6d",
                    "headerhover", "#00ffcc",
                    // PLAYER
                    "playertitle", "#f965d9",
                    "playersubtext", "#6d6d6d",
                    "playeraccent", "#f965d9",
                    "playerhover", "#00ffcc",
                    // TABS
                    "tabtext", "#eedef2",
                    "tabborder", "#6d6d6d",
                    "tabhover", "#f965d9",
                    // PLAYLIST
                    "playlisttext", "#eedef2",
                    "playlistfolder", "#d59407",
                    "playlistactive", "#65f996",
                    "playlisticon", "#f965d9",
                    // EQ
                    "eqbg", "#1b101f",
                    "eqborder", "#6d6d6d",
                    "eqtext", "#f965d9",
                    "eqsubtext", "#6d6d6d",
                    "eqicon", "#f965d9",
                    "eqhover", "#00ffcc",
                    "eqactive", "#f965d9",
                    "eqsliderbg", "#1b101f",
                    "eqfader", "#ccff00",
                    "eqmix", "#00ffcc",
                    "eqhandle", "#f965d9",
                    // FX
                    "fxbg", "#1e1e1e",
                    "fxborder", "#6d6d6d",
                    "fxtext", "#f965d9",
                    "fxsubtext", "#6d6d6d",
                    "fxicon", "#f965d9",
                    "fxhover", "#00ffcc",
                    "fxactive", "#f965d9",
                    "fxslider", "#f965d9",
                    "fxsliderbg", "#121212",
                    "fxhandle", "#f965d9",
                });
            }
            "Red" => {
                c!(map, {
                    // BACKGROUNDS
                    "bgmain", "#121212",
                    "bgoverlay", "#1e1e1e",
                    "graysolid", "#333333",
                    "contextmenubg", "#181818",
                    "overlay", "#80000000",
                    // HEADER
                    "headerbg", "#1e1e1e",
                    "headericon", "#6d6d6d",
                    "headertext", "#6d6d6d",
                    "headerhover", "#ff003c",
                    // PLAYER
                    "playertitle", "#ff003c",
                    "playersubtext", "#bdbdbd",
                    "playeraccent", "#ff003c",
                    "playerhover", "#2b00ff",
                    // TABS
                    "tabtext", "#bdbdbd",
                    "tabborder", "#6d6d6d",
                    "tabhover", "#ff003c",
                    // PLAYLIST
                    "playlisttext", "#bdbdbd",
                    "playlistfolder", "#d59407",
                    "playlistactive", "#ff003c",
                    "playlisticon", "#2b00ff",
                    // EQ
                    "eqbg", "#1c0d0d",
                    "eqborder", "#6d6d6d",
                    "eqtext", "#ff003c",
                    "eqsubtext", "#bdbdbd",
                    "eqicon", "#ff003c",
                    "eqhover", "#2b00ff",
                    "eqactive", "#ff003c",
                    "eqsliderbg", "#1c0d0d",
                    "eqfader", "#bdbdbd",
                    "eqmix", "#2b00ff",
                    "eqhandle", "#ff003c",
                    // FX
                    "fxbg", "#1e1e1e",
                    "fxborder", "#6d6d6d",
                    "fxtext", "#ff003c",
                    "fxsubtext", "#bdbdbd",
                    "fxicon", "#ff003c",
                    "fxhover", "#2b00ff",
                    "fxactive", "#ff003c",
                    "fxslider", "#ff003c",
                    "fxsliderbg", "#121212",
                    "fxhandle", "#ff003c",
                });
            }
            "Yellow" => {
                c!(map, {
                    // BACKGROUNDS
                    "bgmain", "#0d1012",
                    "bgoverlay", "#15191c",
                    "graysolid", "#2d353b",
                    "contextmenubg", "#0a0c0e",
                    "overlay", "#000000",
                    // HEADER
                    "headerbg", "#15191c",
                    "headericon", "#6d6d6d",
                    "headertext", "#6d6d6d",
                    "headerhover", "#f965d9",
                    // PLAYER
                    "playertitle", "#ffea00",
                    "playersubtext", "#6d6d6d",
                    "playeraccent", "#ffea00",
                    "playerhover", "#f965d9",
                    // TABS
                    "tabtext", "#dde0d1",
                    "tabborder", "#6d6d6d",
                    "tabhover", "#ffea00",
                    // PLAYLIST
                    "playlisttext", "#dde0d1",
                    "playlistfolder", "#d59407",
                    "playlistactive", "#ffea00",
                    "playlisticon", "#f965d9",
                    // EQ
                    "eqbg", "#15191c",
                    "eqborder", "#6d6d6d",
                    "eqtext", "#ffea00",
                    "eqsubtext", "#6d6d6d",
                    "eqicon", "#ffea00",
                    "eqhover", "#f965d9",
                    "eqactive", "#ffea00",
                    "eqsliderbg", "#15191c",
                    "eqfader", "#b000ff",
                    "eqmix", "#f965d9",
                    "eqhandle", "#ffea00",
                    // FX
                    "fxbg", "#15191c",
                    "fxborder", "#6d6d6d",
                    "fxtext", "#ffea00",
                    "fxsubtext", "#6d6d6d",
                    "fxicon", "#ffea00",
                    "fxhover", "#f965d9",
                    "fxactive", "#ffea00",
                    "fxslider", "#ffea00",
                    "fxsliderbg", "#0d1012",
                    "fxhandle", "#ffea00",
                });
            }
            _ => {
                c!(map, {
                    // BACKGROUNDS
                    "bgmain", "#15141b",
                    "bgoverlay", "#201f2b",
                    "graysolid", "#6d6d6d",
                    "contextmenubg", "#2d2d2d",
                    "overlay", "#000000",
                    // HEADER
                    "headerbg", "#201f2b",
                    "headericon", "#6d6d6d",
                    "headertext", "#6d6d6d",
                    "headerhover", "#ff1ae0",
                    // PLAYER
                    "playertitle", "#00ffa2",
                    "playersubtext", "#57caab",
                    "playeraccent", "#9442ff",
                    "playerhover", "#ff1ae0",
                    // TABS
                    "tabtext", "#c6c6c6",
                    "tabborder", "#00ffa2",
                    "tabhover", "#ff1ae0",
                    // PLAYLIST
                    "playlisttext", "#c6c6c6",
                    "playlistfolder", "#ff881a",
                    "playlistactive", "#00ffa2",
                    "playlisticon", "#ff881a",
                    // EQ
                    "eqbg", "#201f2b",
                    "eqborder", "#00ffa2",
                    "eqtext", "#00ffa2",
                    "eqsubtext", "#57caab",
                    "eqicon", "#ff881a",
                    "eqhover", "#ff1ae0",
                    "eqactive", "#9442ff",
                    "eqsliderbg", "#201f2b",
                    "eqfader", "#ff881a",
                    "eqmix", "#ff1ae0",
                    "eqhandle", "#9442ff",
                    // FX
                    "fxbg", "#201f2b",
                    "fxborder", "#00ffa2",
                    "fxtext", "#00ffa2",
                    "fxsubtext", "#57caab",
                    "fxicon", "#9442ff",
                    "fxhover", "#ff1ae0",
                    "fxactive", "#9442ff",
                    "fxslider", "#9442ff",
                    "fxsliderbg", "#15141b",
                    "fxhandle", "#ff1ae0",
                });
            }
        }

        let qmap: QVariantMap = map
            .into_iter()
            .map(|(k, v)| (QString::from(k), QVariant::from(QString::from(v))))
            .collect();

        self.colormap = qmap;
        self.current_theme = QString::from(name);
        self.colormap_changed();
        self.current_theme_changed();
    }
}
