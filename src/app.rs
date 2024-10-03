mod core;
mod menu;
mod hiragana;
mod katakana;
mod constants;
mod utils;
mod settings;
mod app_ext;
mod no_data;
mod loading;

use no_data::{Download, Error, Progress};
use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct Manabu {
    pub input: String,
    pub state: State,
    #[allow(dead_code)]
    pub data: Vec<DataEntry>,
    #[allow(dead_code)]
    pub index: usize,
    pub kanji_list: Vec<KanjiEntry>,
    pub practiced_word: String,
    pub practiced_kanji_entry: Option<KanjiEntry>,
    pub valid_word_found: bool,
    pub correct_answer: Option<bool>,
    pub correct_answer_marked_samples: Vec<AnswerSample>,
    pub user_answer_marked_samples: Vec<AnswerSample>,
    pub settings: Settings,
    pub temp_scale: f32,
    pub downloads: Vec<Download>,
    pub last_download_id: usize,
}

#[derive(Default, Debug, PartialEq, Eq)]
pub struct AnswerSample {
    pub text: String,
    pub correct: bool
}

#[derive(Serialize, Deserialize)]
pub struct Settings {
    theme: Theme,
    text_scale: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            theme: Theme::default(),
            text_scale: 1.0
        }
    }
}

#[derive(Serialize, Deserialize, Default, PartialEq, Eq)]
enum Theme {
    #[default]
    System,
    Dark,
    Light,
}

// #[deprecated]
// #[derive(Serialize, Deserialize, Copy, Clone)]
// #[serde(untagged)]
// enum ConditionalRange {
//     Single(usize),
//     Range(usize, usize)
// }

// impl From<ConditionalRange> for bool {
//     fn from(c: ConditionalRange) -> Self {
//         match c {
//             ConditionalRange::Single(_) => false,
//             ConditionalRange::Range(_, _) => true
//         }
//     }
// }

// impl Default for ConditionalRange {
//     fn default() -> Self {
//         ConditionalRange::Single(5)
//     }
// }

// impl ConditionalRange {
//     pub fn toggle(&mut self) {
//         if self.to_owned().into() {
//             *self = ConditionalRange::Single(5);
//         } else {
//             *self = ConditionalRange::Range(3, 7);
//         }
//     }
// }

#[derive(Default, Serialize, Deserialize)]
pub struct DataEntry {
    jp: String,
    en: String,
    kana: Option<String>
}

#[derive(Default)]
pub enum State {
    #[default]
    Loading,
    NoData,
    Menu,
    Hiragana,
    Katakana,
    #[allow(dead_code)]
    Kanji,
    Settings
}

#[derive(Debug, Clone)]
pub enum Message {
    UpdateInput(String),
    StartHiragana,
    StartKatakana,
    Menu,
    CheckAnswerHiragana,
    CheckAnswerKatakana,
    NewHiragana,
    NewKatakana,
    Settings,
    SubmitAnswer,
    Quit,
    ChangeSettings(ChangedSettings),
    KanjiLoaded(Option<Vec<KanjiEntry>>),
    Download(usize),
    DownloadProgressed((usize, Result<Progress, Error>)),
}

#[derive(Debug, Clone)]
pub enum ChangedSettings {
    ThemeSystem,
    ThemeLight,
    ThemeDark,
    TextScale(f32),
    SaveScale,
    // ToggleKanaRange,
    // ChangeMinKanaChars(String),
    // ChangeMaxKanaChars(String)
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct KanjiEntry {
    char: String,
    onyomi: Vec<String>,
    kunyomi: Vec<String>,
    meanings: Vec<String>,
    name_readings: Vec<String>,
}