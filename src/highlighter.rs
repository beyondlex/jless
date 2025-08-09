use crate::config::{Themes};
use crate::terminal::{Color, Style};

pub struct Highlighter {
    pub highlight: Option<Themes>,
}

impl Highlighter {
    pub fn new(theme_config: Option<Themes>) -> Self {
        Self {
            highlight: theme_config
        }
    }
    
    pub fn style(&self, key:&str) -> Style {
        if self.highlight.is_none() {
            self.default_style()
        } else {
            self.highlight.as_ref().unwrap().get(key).unwrap_or(&self.default_style()).clone()
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