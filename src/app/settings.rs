use iced::widget::{button, column, horizontal_rule as hr, row, scrollable, slider, text};
use iced::Element;

use super::utils::save_config;
use super::{Manabu, ChangedSettings, Message, Theme};

impl Manabu {
    pub fn view_settings(&self) -> Element<Message> {
        let back = button(text("Back")
            .size(30.0 * self.settings.text_scale))
            .on_press(Message::Menu);

        let title = text("Settings").size(50.0 * self.settings.text_scale);
        let theme_text = text("Theme").size(30.0 * self.settings.text_scale);
        let text_scale_text = text("Text scale:").size(30.0 * self.settings.text_scale);

        let theme_system = button(text("System").size(30.0 * self.settings.text_scale)).on_press(Message::ChangeSettings(ChangedSettings::ThemeSystem)).style(if self.settings.theme != Theme::System { iced::widget::button::secondary } else { iced::widget::button::primary }).padding(10);
        let theme_light = button(text("Light").size(30.0 * self.settings.text_scale)).on_press(Message::ChangeSettings(ChangedSettings::ThemeLight)).style(if self.settings.theme != Theme::Light { iced::widget::button::secondary } else { iced::widget::button::primary }).padding(10);
        let theme_dark = button(text("Dark").size(30.0 * self.settings.text_scale)).on_press(Message::ChangeSettings(ChangedSettings::ThemeDark)).style(if self.settings.theme != Theme::Dark { iced::widget::button::secondary } else { iced::widget::button::primary }).padding(10);
        let theme_row = row![theme_system, theme_light, theme_dark].align_y(iced::Alignment::Center).spacing(10);

        let text_scale_row = row![
            text(format!("{:.1}", self.temp_scale)).size(30.0 * self.settings.text_scale).width(iced::Length::FillPortion(1)).align_x(iced::alignment::Horizontal::Right),
            slider(0.5..=2.0, self.temp_scale, 
                |v| Message::ChangeSettings(ChangedSettings::TextScale(v)))
                .on_release(Message::ChangeSettings(ChangedSettings::SaveScale))
                .step(0.1).width(iced::Length::FillPortion(6)),
            text("").width(iced::Length::FillPortion(1))
        ].align_y(iced::Alignment::Center).spacing(15);


        let main_body = row![
            scrollable(
                column![
                    title, hr(5), theme_text, theme_row, hr(2), text_scale_text, text_scale_row, hr(2)
                ].align_x(iced::Alignment::Center).spacing(5)
            )
        ];

        column![back, main_body].width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .padding(10).into()
            
    }

    pub fn update_settings(&mut self, message: ChangedSettings) {
        match message {
            ChangedSettings::ThemeSystem => {
                self.settings.theme = Theme::System;
            }
            ChangedSettings::ThemeLight => {
                self.settings.theme = Theme::Light;
            }
            ChangedSettings::ThemeDark => {
                self.settings.theme = Theme::Dark;
            }
            ChangedSettings::TextScale(s) => {
                self.temp_scale = s;
            }
            ChangedSettings::SaveScale => {
                self.settings.text_scale = self.temp_scale;
            }
        };
        save_config(&self.settings);
    }
}