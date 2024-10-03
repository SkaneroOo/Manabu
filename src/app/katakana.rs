use iced::widget::{button, column, row, text, text_input, Space};
use iced::Element;
use wana_kana::ConvertJapanese;

use super::utils::ColoredText;
use super::{Manabu, Message};

impl Manabu {
    pub fn view_katakana(&self) -> Element<Message> {

        let back = button(text("Back").size(30.0 * self.settings.text_scale)).on_press(Message::Menu);
        let button_row = row![
            button(text("Check answer").size(30.0 * self.settings.text_scale)).on_press(Message::CheckAnswerKatakana).style(iced::widget::button::primary).padding(30),
            button(text("Next").size(30.0 * self.settings.text_scale)).on_press(Message::NewKatakana).style(iced::widget::button::primary).padding(30)
        ].spacing(10);

        let write_word_text = text("Write the word below:").size(30.0 * self.settings.text_scale).align_x(iced::alignment::Horizontal::Center);
        let word_to_write = text(&self.practiced_word).shaping(iced::widget::text::Shaping::Advanced).size(50.0 * self.settings.text_scale).align_x(iced::alignment::Horizontal::Center);

        let centered_text_input = row![
            text("").width(iced::Length::FillPortion(1)),
            text_input("", &self.input).on_input(|s| Message::UpdateInput(s)).on_submit(Message::SubmitAnswer).size(30.0 * self.settings.text_scale).width(iced::Length::FillPortion(2)),
            text("").width(iced::Length::FillPortion(1)),
        ];

        let col = if self.correct_answer.is_none() {
            column![
                write_word_text, word_to_write, centered_text_input,
                column![button_row].align_x(iced::Alignment::Center).width(iced::Length::FillPortion(2)).spacing(10),
            ].align_x(iced::Alignment::Center).height(iced::Length::Fill).width(iced::Length::Fill).spacing(10)
        } else {
            let correct = self.correct_answer.unwrap();
            let (text_top, marked) = if correct {
                (
                    text("Correct!").size(30.0 * self.settings.text_scale).align_x(iced::alignment::Horizontal::Center),
                    row![
                        text("Correct answer: ").size(30.0 * self.settings.text_scale).align_x(iced::alignment::Horizontal::Center),
                        ColoredText::new(45.0 * self.settings.text_scale).push(&self.practiced_word.to_romaji(), 0, 255, 0).finalize()
                    ]
                )
            } else {
                (
                    text("Incorrect!").size(30.0 * self.settings.text_scale).align_x(iced::alignment::Horizontal::Center),
                    row![column![
                        row![
                            text("Your answer: ").size(30.0 * self.settings.text_scale).align_x(iced::alignment::Horizontal::Center),
                            {
                                let mut t = ColoredText::new(30.0 * self.settings.text_scale);
                                for part in self.user_answer_marked_samples.iter() {
                                    t = t.push(&part.text, if part.correct { 0 } else { 255 }, if part.correct { 255 } else { 0 }, 0);
                                }
                                t.finalize()
                            }
                        ],
                        row![
                            text("Correct answer: ").size(30.0 * self.settings.text_scale).align_x(iced::alignment::Horizontal::Center),
                            {
                                let mut t = ColoredText::new(30.0 * self.settings.text_scale);
                                for part in self.correct_answer_marked_samples.iter() {
                                    t = t.push(&part.text, if part.correct { 0 } else { 255 }, if part.correct { 255 } else { 0 }, 0);
                                }
                                t.finalize()
                            }
                        ]
                    ]]
                )
            };
            let kanji = self.practiced_kanji_entry.as_ref().unwrap();

            let space = Space::with_width(iced::Length::Fill).height(25);

            let kanji_top_text = text("Additional word info:").size(30.0 * self.settings.text_scale).align_x(iced::alignment::Horizontal::Center);
            
            let kanji_info_kanji = row![
                text("Kanji: ").size(20.0 * self.settings.text_scale).align_x(iced::alignment::Horizontal::Center),
                text(kanji.char.clone()).shaping(iced::widget::text::Shaping::Advanced).size(30.0 * self.settings.text_scale).align_x(iced::alignment::Horizontal::Center)
            ].align_y(iced::Alignment::Center);

            let kunyomi_reading = row![
                text("Kunyomi reading: ").size(20.0 * self.settings.text_scale).align_x(iced::alignment::Horizontal::Center),
                text(kanji.kunyomi.join(", ")).shaping(iced::widget::text::Shaping::Advanced).size(20.0 * self.settings.text_scale).align_x(iced::alignment::Horizontal::Center)
            ];

            let onyomi_reading = row![
                text("Onyomi reading: ").size(20.0 * self.settings.text_scale).align_x(iced::alignment::Horizontal::Center),
                text(kanji.onyomi.join(", ")).shaping(iced::widget::text::Shaping::Advanced).size(20.0 * self.settings.text_scale).align_x(iced::alignment::Horizontal::Center)
            ];

            let nanori_reading = row![
                text("Nanori reading: ").size(20.0 * self.settings.text_scale).align_x(iced::alignment::Horizontal::Center),
                text(kanji.name_readings.join(", ")).shaping(iced::widget::text::Shaping::Advanced).size(20.0 * self.settings.text_scale).align_x(iced::alignment::Horizontal::Center)
            ];

            let meaning = row![
                text("Meaning: ").size(20.0 * self.settings.text_scale).align_x(iced::alignment::Horizontal::Center),
                text(kanji.meanings.join(", ")).size(20.0 * self.settings.text_scale).align_x(iced::alignment::Horizontal::Center)
            ];


            let kanji_info = column![
                space,
                kanji_top_text,
                kanji_info_kanji,
                kunyomi_reading,
                onyomi_reading,
                nanori_reading,
                meaning
            ].align_x(iced::Alignment::Center).width(iced::Length::FillPortion(1));

            let bottom = column![text_top, marked, kanji_info].align_x(iced::Alignment::Center);
            column![
                write_word_text, word_to_write, centered_text_input,
                column![button_row].align_x(iced::Alignment::Center).width(iced::Length::FillPortion(2)).spacing(10),
                bottom
            ].align_x(iced::Alignment::Center).height(iced::Length::Fill).width(iced::Length::Fill).spacing(10)
        };
        
        let main_body = row![
            col
        ].align_y(iced::Alignment::Center).width(iced::Length::Fill).height(iced::Length::Fill);

        column![
            back,
            main_body
        ].padding(10).into()
    }
}