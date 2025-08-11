use crate::config::{default_themes, Styles, Themes};
use crate::terminal::{Color, Style};

pub struct Highlighter {
    pub highlight: Option<Themes>,
}

impl Default for Highlighter {
    fn default() -> Self {
        Highlighter { 
            highlight: Some(default_themes()), 
        }
    }
}

impl Highlighter {
    pub fn new(theme_config: Option<Themes>) -> Self {
        if theme_config.is_none() {
            Highlighter::default()
        } else {
            Self {
                highlight: theme_config
            }
        }
    }
    
    pub fn style(&self, key:&str) -> Styles {
        if self.highlight.is_none() {
            Styles::default()
        } else {
            self.highlight.as_ref().unwrap().get(key).unwrap_or(&Styles::default()).clone()
        }
    }

    pub fn dimmed(&self) -> Style {
        Style {
            fg: Color::Default,
            bg: Color::Default,
            inverted: false,
            bold: false,
            dimmed: true,
        }
    }

    pub fn default_style(&self) -> Style {
        Style {
            fg: Color::Default,
            bg: Color::Default,
            inverted: false,
            bold: false,
            dimmed: false,
        }
    }
}