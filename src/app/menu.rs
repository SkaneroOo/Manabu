use iced::widget::{button, column, row, text, Space};
use iced::{Element, Font};

use super::{Manabu, Message};

impl Manabu {
    pub fn view_menu(&self) -> Element<Message> {
        row(vec![
            column(vec![
                Space::with_height(25).into(),
                text("Manabu").size(60.0 * self.settings.text_scale).font(Font::with_name("New Walt Disney UI")).into(),
                text("A Japanese Learning App").size(20.0 * self.settings.text_scale).into(),
                text("by Skaner").size(25.0 * self.settings.text_scale).into(),
                button(text("Practice Hiragana").size(30.0 * self.settings.text_scale)).on_press(Message::StartHiragana)
                    .style(iced::widget::button::primary).padding(30).into(),
                button(text("Practice Katakana").size(30.0 * self.settings.text_scale)).on_press(Message::StartKatakana)
                    .style(iced::widget::button::primary).padding(30).into(),
                button(text("Settings").size(30.0 * self.settings.text_scale)).on_press(Message::Settings)
                    .style(iced::widget::button::secondary).padding(30).into(),
                button(text("Quit").size(30.0 * self.settings.text_scale)).on_press(Message::Quit)
                    .style(iced::widget::button::danger).padding(30).into(),
            ]).align_x(iced::Alignment::Center).height(iced::Length::Fill).width(iced::Length::Fill).spacing(10).into()
        ]).align_y(iced::Alignment::Center).width(iced::Length::Fill).height(iced::Length::Fill).into()
    }
}