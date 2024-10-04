use iced::widget::{column, row, text, Column, Space};
use rand::Rng;

use super::{Manabu, Message};

impl Manabu {
    pub fn select_random_word(&mut self) {
        self.valid_word_found = false;
        let mut rng = rand::thread_rng();
        while !self.valid_word_found {
            let entry = self.kanji_list.get(rng.gen_range(0..self.kanji_list.len())).unwrap();
            if !entry.kunyomi.is_empty() {
                self.practiced_word = entry.kunyomi.get(rng.gen_range(0..entry.kunyomi.len())).unwrap().to_string();
                self.valid_word_found = true;
            } else if !entry.name_readings.is_empty() {
                self.practiced_word = entry.name_readings.get(rng.gen_range(0..entry.name_readings.len())).unwrap().to_string();
                self.valid_word_found = true;
            } else if !entry.onyomi.is_empty() {
                self.practiced_word = entry.onyomi.get(rng.gen_range(0..entry.onyomi.len())).unwrap().to_string();
                self.valid_word_found = true;
            }
            self.practiced_kanji_entry = Some(entry.clone());
        }
        self.practiced_word = self.practiced_word.replace(".", "");
    }

    pub fn get_kanji_info(&self) -> Column<Message> {
        let additional_info = &self.settings.additional_kanji_info;
        if additional_info.display_additional_kanji_info {
            let kanji = self.practiced_kanji_entry.as_ref().unwrap();
            
            let space = Space::with_width(iced::Length::Fill).height(25);

            let kanji_top_text = text("Additional word info:").size(30.0 * self.settings.text_scale).align_x(iced::alignment::Horizontal::Center);
            
            let kanji_info_kanji = if additional_info.display_kanji { 
                row![
                    text("Kanji: ").size(20.0 * self.settings.text_scale).align_x(iced::alignment::Horizontal::Center),
                    text(kanji.char.clone()).shaping(iced::widget::text::Shaping::Advanced).size(30.0 * self.settings.text_scale).align_x(iced::alignment::Horizontal::Center)
                ].align_y(iced::Alignment::Center)
            } else { row![] };

            let kunyomi_reading = if additional_info.display_kunyomi && kanji.kunyomi.len() > 0 {
                row![
                    text("Kunyomi reading: ").size(20.0 * self.settings.text_scale).align_x(iced::alignment::Horizontal::Center),
                    text(kanji.kunyomi.join(", ")).shaping(iced::widget::text::Shaping::Advanced).size(20.0 * self.settings.text_scale).align_x(iced::alignment::Horizontal::Center)
                ]
            } else { row![] };

            let onyomi_reading = if additional_info.display_onyomi && kanji.onyomi.len() > 0 {
                row![
                    text("Onyomi reading: ").size(20.0 * self.settings.text_scale).align_x(iced::alignment::Horizontal::Center),
                    text(kanji.onyomi.join(", ")).shaping(iced::widget::text::Shaping::Advanced).size(20.0 * self.settings.text_scale).align_x(iced::alignment::Horizontal::Center)
                ]
            } else { row![] };

            let nanori_reading = if additional_info.display_nanori && kanji.name_readings.len() > 0 {
                row![
                    text("Nanori reading: ").size(20.0 * self.settings.text_scale).align_x(iced::alignment::Horizontal::Center),
                    text(kanji.name_readings.join(", ")).shaping(iced::widget::text::Shaping::Advanced).size(20.0 * self.settings.text_scale).align_x(iced::alignment::Horizontal::Center)
                ]
            } else { row![] };

            let meaning = if additional_info.display_meaning && kanji.meanings.len() > 0 {
                row![
                    text("Meaning: ").size(20.0 * self.settings.text_scale).align_x(iced::alignment::Horizontal::Center),
                    text(kanji.meanings.join(", ")).size(20.0 * self.settings.text_scale).align_x(iced::alignment::Horizontal::Center)
                ]
            } else { row![] };

            column![
                space,
                kanji_top_text,
                kanji_info_kanji,
                kunyomi_reading,
                onyomi_reading,
                nanori_reading,
                meaning
            ].align_x(iced::Alignment::Center).width(iced::Length::FillPortion(1))
        } else { column![] }
    }
}