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

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdditionalKanjiInfo {
    pub display_additional_kanji_info: bool,
    pub display_kanji: bool,
    pub display_kunyomi: bool,
    pub display_onyomi: bool,
    pub display_nanori: bool,
    pub display_meaning: bool
}

impl Default for AdditionalKanjiInfo {
    fn default() -> Self {
        AdditionalKanjiInfo {
            display_additional_kanji_info: true,
            display_kanji: true,
            display_kunyomi: true,
            display_onyomi: true,
            display_nanori: true,
            display_meaning: true
        }
    }
}

#[derive(Default, Debug, PartialEq, Eq)]
pub struct AnswerSample {
    pub text: String,
    pub correct: bool
}

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub theme: Theme,
    pub text_scale: f32,
    pub additional_kanji_info: AdditionalKanjiInfo,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            theme: Theme::default(),
            text_scale: 1.0,
            additional_kanji_info: AdditionalKanjiInfo::default()
        }
    }
}

#[derive(Serialize, Deserialize, Default, PartialEq, Eq)]
pub enum Theme {
    #[default]
    System,
    Dark,
    Light,
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
    AdditionalKanjiInfo(bool),
    DisplayKanji(bool),
    DisplayKunyomi(bool),
    DisplayOnyomi(bool),
    DisplayNanori(bool),
    DisplayMeaning(bool),
    // ToggleKanaRange,
    // ChangeMinKanaChars(String),
    // ChangeMaxKanaChars(String)
}

impl From<ChangedSettings> for Message {
    fn from(c: ChangedSettings) -> Self {
        Message::ChangeSettings(c)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct KanjiEntry {
    char: String,
    onyomi: Vec<String>,
    kunyomi: Vec<String>,
    meanings: Vec<String>,
    name_readings: Vec<String>,
}