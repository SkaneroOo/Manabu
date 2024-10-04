use iced::widget::{column, row, text};
use iced::Element;

use super::{Manabu, Message};

impl Manabu {
    pub fn view_loading(&self) -> Element<Message> {
        column![
            row![
                text("Loading...").size(50.0 * self.settings.text_scale),
            ].align_y(iced::Alignment::Center),
        ].align_x(iced::Alignment::Center).into()
    }
}