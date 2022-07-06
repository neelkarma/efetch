use config::Config;
use widgets::{ColorsWidget, ExcusesWidget, Widget};

mod config;
mod errors;
mod widgets;

fn main() {
    let config = Config::read();
    ExcusesWidget::run(&config);
    ColorsWidget::run(&config);
}
