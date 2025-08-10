use std::collections::HashMap;
use std::fs;

use serde::{Deserialize, Deserializer};
use std::str::FromStr;
use crate::terminal::{Color, Style};

#[derive(Deserialize)]
pub struct Config {
    pub themes: ThemesConfig
}

#[derive(Copy, Clone, Deserialize)]
pub struct Styles {
    pub default: Style,
    pub default_matched: Style,
    pub focused: Style,
    pub focused_matched: Style,
}
impl Default for Styles {
    fn default() -> Self {
        Styles {
            default: Style::default(),
            default_matched: Style::default(),
            focused: Style::default(),
            focused_matched: Style::default(),
        }
    }
}

pub type ThemesConfig = HashMap<String, Vec<Option<StyleConfig>>>;
pub type Themes = HashMap<String, Styles>;

fn themes_config_to_themes(value: ThemesConfig) -> Themes {
    let mut themes = HashMap::new();
    for (name, style_configs) in value {
        let mut i = 0;
        let mut styles = Styles::default();
        for config in style_configs {
            let style = cfg_to_style(config);
            match i {
                0 => styles.default = style,
                1 => styles.default_matched = style,
                2 => styles.focused = style,
                3 => styles.focused_matched = style,
                _ => {}
            }
            i += 1;
        }
        themes.insert(name, styles);
    }
    themes
}

impl From<StyleConfig> for Style {
    fn from(style_cfg: StyleConfig) -> Self {
        Style {
            fg: style_cfg.fg.unwrap_or(Color::Default),
            bg: style_cfg.bg.unwrap_or(Color::Default),
            inverted: style_cfg.inverted.unwrap_or(false),
            bold: style_cfg.bold.unwrap_or(false),
            dimmed: style_cfg.dimmed.unwrap_or(false),
        }
    }
}

fn cfg_to_style(style_config: Option<StyleConfig>) -> Style {
    match style_config {
        Some(cfg) => Style::from(cfg),
        None => Style::default(),
    }
}


#[derive(Copy, Clone, Deserialize)]
pub struct StyleConfig {
    #[serde(default)]
    pub fg: Option<Color>,
    #[serde(default)]
    pub bg: Option<Color>,
    #[serde(default)]
    pub inverted: Option<bool>,
    #[serde(default)]
    pub bold: Option<bool>,
    #[serde(default)]
    pub dimmed: Option<bool>,
}

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Color::from_str(&s).map_err(serde::de::Error::custom)
    }
}

impl FromStr for Color {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "default" => Ok(Color::Default),
            s if s.starts_with("C16(") => {
                let c = s.trim_start_matches("C16(").trim_end_matches(')');
                c.parse::<u8>()
                    .map(Color::C16)
                    .map_err(|_| format!("Invalid ANSI color: {}", s))
            }
            s if s.starts_with('#') || s.len() == 6 => {
                Color::from_hex(s).ok_or_else(|| format!("Invalid hex color: {}", s))
            }
            _ => Err(format!("Unknown color format: {}", s)),
        }
    }
}

pub fn load_hl_config(path: &str) -> Result<Option<Themes>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let config = toml::from_str::<Config>(&content).unwrap();
    Ok(Some(themes_config_to_themes(config.themes)))
}
