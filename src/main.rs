#![windows_subsystem = "windows"]
mod app;

use app::Manabu;

fn main() -> iced::Result {
    iced::application(Manabu::title, Manabu::update, Manabu::view)
        .font(include_bytes!("../assets/fonts/NewWaltDisneyUi-8YdA.ttf"))
        .theme(Manabu::theme)
        .subscription(Manabu::subscription)
        .run_with(Manabu::new)
}
