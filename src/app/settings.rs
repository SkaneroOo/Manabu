use iced::widget::{button, checkbox, column, horizontal_rule as hr, pick_list, row, scrollable, slider, text, Space};
use iced::Element;

use super::utils::save_config;
use super::{Manabu, ChangedSettings, Message, ManabuTheme};

impl Manabu {
    pub fn view_settings(&self) -> Element<Message> {
        let back = button(text("Back")
            .size(30.0 * self.settings.text_scale))
            .on_press(Message::Menu);

        let title = text("Settings").size(50.0 * self.settings.text_scale);
        let theme_text = text("Theme").size(30.0 * self.settings.text_scale);
        let text_scale_text = text("Text scale:").size(30.0 * self.settings.text_scale);

        // let theme_system = button(text("System").size(30.0 * self.settings.text_scale)).on_press(ChangedSettings::ThemeSystem.into()).style(if self.settings.theme != ManabuTheme::System { iced::widget::button::secondary } else { iced::widget::button::primary }).padding(10);
        // let theme_light = button(text("Light").size(30.0 * self.settings.text_scale)).on_press(ChangedSettings::ThemeLight.into()).style(if self.settings.theme != ManabuTheme::Light { iced::widget::button::secondary } else { iced::widget::button::primary }).padding(10);
        // let theme_dark = button(text("Dark").size(30.0 * self.settings.text_scale)).on_press(ChangedSettings::ThemeDark.into()).style(if self.settings.theme != ManabuTheme::Dark { iced::widget::button::secondary } else { iced::widget::button::primary }).padding(10);
        // let theme_row = row![theme_system, theme_light, theme_dark].align_y(iced::Alignment::Center).spacing(10);

        let theme_selector = pick_list(ManabuTheme::ALL, Some(self.settings.theme.clone()), |t| ChangedSettings::SetTheme(t).into())
            .text_size(25.0 * self.settings.text_scale);

        let text_scale_row = row![
            text(format!("{:.1}", self.temp_scale)).size(30.0 * self.settings.text_scale).width(iced::Length::FillPortion(1)).align_x(iced::alignment::Horizontal::Right),
            slider(0.5..=2.0, self.temp_scale, 
                |v| ChangedSettings::TextScale(v).into())
                .on_release(ChangedSettings::SaveScale.into())
                .step(0.1).width(iced::Length::FillPortion(6)),
            text("").width(iced::Length::FillPortion(1))
        ].align_y(iced::Alignment::Center).spacing(15);

        let additional_kanji_info_text = text("Additional kanji info:").size(30.0 * self.settings.text_scale);

        let kanji_info = &self.settings.additional_kanji_info;
        let kanji_row_left = column![
            row![
                text("Display additional kanji info").size(30.0 * self.settings.text_scale).align_x(iced::alignment::Horizontal::Right),
                checkbox("", kanji_info.display_additional_kanji_info)
                    .size(30).on_toggle(|v| ChangedSettings::AdditionalKanjiInfo(v).into()),
            ].align_y(iced::Alignment::Center).spacing(10).height(iced::Length::Fill),
        ];

        let kanji_row_right = if kanji_info.display_additional_kanji_info {
            column![
                row![
                    checkbox("", kanji_info.display_kanji)
                        .size(30).on_toggle(|v| ChangedSettings::DisplayKanji(v).into()),
                    text("Display kanji").size(30.0 * self.settings.text_scale).align_x(iced::alignment::Horizontal::Left),
                ].align_y(iced::Alignment::Center).spacing(10).height(iced::Length::Fill),
                row![
                    checkbox("", kanji_info.display_kunyomi)
                        .size(30).on_toggle(|v| ChangedSettings::DisplayKunyomi(v).into()),
                    text("Display kunyomi").size(30.0 * self.settings.text_scale).align_x(iced::alignment::Horizontal::Left),
                ].align_y(iced::Alignment::Center).spacing(10).height(iced::Length::Fill),
                row![
                    checkbox("", kanji_info.display_onyomi)
                        .size(30).on_toggle(|v| ChangedSettings::DisplayOnyomi(v).into()),
                    text("Display onyomi").size(30.0 * self.settings.text_scale).align_x(iced::alignment::Horizontal::Left),
                ].align_y(iced::Alignment::Center).spacing(10).height(iced::Length::Fill),
                row![
                    checkbox("", kanji_info.display_nanori)
                        .size(30).on_toggle(|v| ChangedSettings::DisplayNanori(v).into()),
                    text("Display nanori").size(30.0 * self.settings.text_scale).align_x(iced::alignment::Horizontal::Left),
                ].align_y(iced::Alignment::Center).spacing(10).height(iced::Length::Fill),
                row![
                    checkbox("", kanji_info.display_meaning)
                        .size(30).on_toggle(|v| ChangedSettings::DisplayMeaning(v).into()),
                    text("Display meaning").size(30.0 * self.settings.text_scale).align_x(iced::alignment::Horizontal::Left),
                ].align_y(iced::Alignment::Center).spacing(10).height(iced::Length::Fill),
            ]
        } else {
            column![Space::with_width(iced::Length::Fill)]
        };
                
        let additional_kanji_info_row = row![
            kanji_row_left.align_x(iced::alignment::Horizontal::Right).width(iced::Length::FillPortion(1)),
            kanji_row_right.align_x(iced::alignment::Horizontal::Left).width(iced::Length::FillPortion(1))
        ].spacing(10).height(250.0 * self.settings.text_scale).align_y(iced::Alignment::Center);

        let main_body = row![
            scrollable(
                column![
                    title, hr(5), 
                    theme_text, theme_selector, hr(2), 
                    text_scale_text, text_scale_row, hr(2),
                    additional_kanji_info_text, additional_kanji_info_row
                ].align_x(iced::Alignment::Center).spacing(5)
            )
        ];

        column![back, main_body].width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .padding(10).into()
            
    }

    pub fn update_settings(&mut self, message: ChangedSettings) {
        match message {
            ChangedSettings::SetTheme(theme) => {
                self.settings.theme = theme;
            }
            ChangedSettings::ThemeSystem => {
                self.settings.theme = ManabuTheme::default();
            }
            ChangedSettings::ThemeLight => {
                self.settings.theme = ManabuTheme::Light;
            }
            ChangedSettings::ThemeDark => {
                self.settings.theme = ManabuTheme::Dark;
            }
            ChangedSettings::TextScale(s) => {
                self.temp_scale = s;
            }
            ChangedSettings::SaveScale => {
                self.settings.text_scale = self.temp_scale;
            },
            ChangedSettings::AdditionalKanjiInfo(v) => {
                self.settings.additional_kanji_info.display_additional_kanji_info = v;
            },
            ChangedSettings::DisplayKanji(v) => {
                self.settings.additional_kanji_info.display_kanji = v;
            },
            ChangedSettings::DisplayKunyomi(v) => {
                self.settings.additional_kanji_info.display_kunyomi = v;
            },
            ChangedSettings::DisplayOnyomi(v) => {
                self.settings.additional_kanji_info.display_onyomi = v;
            },
            ChangedSettings::DisplayNanori(v) => {
                self.settings.additional_kanji_info.display_nanori = v;
            },
            ChangedSettings::DisplayMeaning(v) => {
                self.settings.additional_kanji_info.display_meaning = v;
            },
        };
        save_config(&self.settings);
    }
}