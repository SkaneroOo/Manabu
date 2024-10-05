use iced::Element;

use wana_kana::ConvertJapanese;

use super::no_data::Download;
use super::utils::{load_config, load_kanji, check_correctness};
use super::Manabu;
use super::Theme;
use super::Message;
use super::State;


impl Manabu {

    pub fn title(&self) -> String {
        String::from("JP Practice Tool by Skaner")
    }

    pub fn view(&self) -> Element<Message> {
        match self.state {
            State::Loading => {
                self.view_loading()
            }
            State::NoData => {
                self.view_no_data()
            }
            State::Menu => {
                self.view_menu()
            },
            State::Hiragana => {
                self.view_hiragana()
            },
            State::Katakana => {
                self.view_katakana()
            },
            State::Settings => {
                self.view_settings()
            },
            _ => {
                unimplemented!()
            }
        }
    }

    pub fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            Message::UpdateInput(s) => {
                self.input = s;
                iced::Task::none()
            },
            Message::StartHiragana => {
                self.state = State::Hiragana;
                self.select_random_word();
                self.practiced_word = self.practiced_word.to_hiragana();
                iced::Task::none()
            },
            Message::StartKatakana => {
                self.state = State::Katakana;
                self.select_random_word();
                self.practiced_word = self.practiced_word.to_katakana();
                iced::Task::none()
            },
            Message::Menu => {
                self.state = State::Menu;
                self.correct_answer = None;
                self.input = String::new();
                iced::Task::none()
            },
            Message::NewHiragana => {
                self.select_random_word();
                self.practiced_word = self.practiced_word.to_hiragana();
                self.correct_answer = None;
                self.input = String::new();
                iced::Task::none()
            },
            Message::CheckAnswerHiragana => {
                let ans = check_correctness(&self.input, &self.practiced_word.to_romaji());
                self.correct_answer = Some(ans.0);
                self.correct_answer_marked_samples = ans.1;
                self.user_answer_marked_samples = ans.2;
                iced::Task::none()
            },
            Message::NewKatakana => {
                self.select_random_word();
                self.practiced_word = self.practiced_word.to_katakana();
                self.correct_answer = None;
                self.input = String::new();
                iced::Task::none()
            },
            Message::CheckAnswerKatakana => {
                let ans = check_correctness(&self.input, &self.practiced_word.to_romaji());
                self.correct_answer = Some(ans.0);
                self.correct_answer_marked_samples = ans.1;
                self.user_answer_marked_samples = ans.2;
                iced::Task::none()
            },
            Message::SubmitAnswer => {
                match (&self.state, self.correct_answer) {
                    (State::Hiragana, Some(_)) => {
                        self.select_random_word();
                        self.practiced_word = self.practiced_word.to_hiragana();
                        self.correct_answer = None;
                        self.input = String::new();
                    },
                    (State::Hiragana, None) => {
                        let ans = check_correctness(&self.input, &self.practiced_word.to_romaji());
                        self.correct_answer = Some(ans.0);
                        self.correct_answer_marked_samples = ans.1;
                        self.user_answer_marked_samples = ans.2;
                    },
                    (State::Katakana, Some(_)) => {
                        self.select_random_word();
                        self.practiced_word = self.practiced_word.to_katakana();
                        self.correct_answer = None;
                        self.input = String::new();
                    },
                    (State::Katakana, None) => {
                        let ans = check_correctness(&self.input, &self.practiced_word.to_romaji());
                        self.correct_answer = Some(ans.0);
                        self.correct_answer_marked_samples = ans.1;
                        self.user_answer_marked_samples = ans.2;
                    },
                    _ => {}
                };
                iced::Task::none()
            },
            Message::Quit => {
                std::process::exit(0);
            },
            Message::Settings => {
                self.state = State::Settings;
                iced::Task::none()
            },
            Message::ChangeSettings(s) => {
                self.update_settings(s);
                iced::Task::none()
            },
            Message::KanjiLoaded(data) => {
                match data {
                    Some(data) => {
                        self.state = State::Menu;
                        self.kanji_list = data
                    },
                    None => {
                        self.state = State::NoData;
                        self.downloads.push(Download::new(self.last_download_id));
                    }
                }
                iced::Task::none()
            },
            Message::Download(index) => {
                if let Some(download) = self.downloads.get_mut(index) {
                    download.start();
                }
                iced::Task::none()
            }
            Message::DownloadProgressed((id, progress)) => {
                if let Some(download) = self.downloads.get_mut(id) {
                    download.progress(progress);
                    if download.state == crate::app::no_data::State::Finished {
                        iced::Task::perform(load_kanji(), Message::KanjiLoaded)
                    } else {
                        iced::Task::none()
                    }
                } else {
                    iced::Task::none()
                }
            }
            #[allow(unreachable_patterns)]
            _ => iced::Task::none(),
        }
    }

    pub fn subscription(&self) -> iced::Subscription<Message> {

        iced::Subscription::batch(
            self.downloads.iter().map(Download::subscription)
        )
    }

    pub fn theme(&self) -> iced::Theme {
        self.settings.theme.clone().into()
    }

    pub fn new() -> (Self, iced::Task<Message>) {
        let cfg = load_config();
        (Self {
            temp_scale: cfg.text_scale,
            settings: cfg,
            ..Default::default()
        }, iced::Task::perform(load_kanji(), Message::KanjiLoaded))
    }

    
}

