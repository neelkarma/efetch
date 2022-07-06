use crate::config::Config;

mod colors;
mod excuse;

pub use colors::ColorsWidget;
pub use excuse::ExcusesWidget;

pub trait Widget {
    fn render(config: &Config);
    fn is_disabled(config: &Config) -> bool;
    fn run(config: &Config) {
        if !Self::is_disabled(config) {
            Self::render(config);
        }
    }
}
