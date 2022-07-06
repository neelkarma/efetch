use ansi_term::{Color, Style};

use crate::config::Config;

use super::Widget;

pub struct ColorsWidget;

impl Widget for ColorsWidget {
    fn render(_: &Config) {
        for color in [
            Color::Red,
            Color::Green,
            Color::Yellow,
            Color::Blue,
            Color::Purple,
            Color::Cyan,
            Color::White,
        ] {
            print!("{}", Style::new().on(color).paint("    "));
        }
        println!();
    }

    fn is_disabled(config: &Config) -> bool {
        config.colors.disabled
    }
}
