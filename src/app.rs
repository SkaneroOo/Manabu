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

use std::fmt;

use iced::Theme;
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
    pub theme: ManabuTheme,
    pub text_scale: f32,
    pub additional_kanji_info: AdditionalKanjiInfo,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            theme: ManabuTheme::default(),
            text_scale: 1.0,
            additional_kanji_info: AdditionalKanjiInfo::default()
        }
    }
}

#[derive(Serialize, Deserialize, Default, PartialEq, Eq, Debug, Clone)]
pub enum ManabuTheme {
    #[default]
    System,
    Dark,
    Light,
    Dracula,
    Nord,
    SolarizedLight,
    SolarizedDark,
    GruvboxLight,
    GruvboxDark,
    CatppuccinLatte,
    CatppuccinFrappe,
    CatppuccinMacchiato,
    CatppuccinMocha,
    TokyoNight,
    TokyoNightStorm,
    TokyoNightLight,
    KanagawaWave,
    KanagawaDragon,
    KanagawaLotus,
    Moonfly,
    Nightfly,
    Oxocarbon,
    Ferra,
}

impl ManabuTheme {
    pub const ALL: &'static [Self] = &[
        Self::System,
        Self::Light,
        Self::Dark,
        Self::Dracula,
        Self::Nord,
        Self::SolarizedLight,
        Self::SolarizedDark,
        Self::GruvboxLight,
        Self::GruvboxDark,
        Self::CatppuccinLatte,
        Self::CatppuccinFrappe,
        Self::CatppuccinMacchiato,
        Self::CatppuccinMocha,
        Self::TokyoNight,
        Self::TokyoNightStorm,
        Self::TokyoNightLight,
        Self::KanagawaWave,
        Self::KanagawaDragon,
        Self::KanagawaLotus,
        Self::Moonfly,
        Self::Nightfly,
        Self::Oxocarbon,
        Self::Ferra,
    ];
}

impl From<ManabuTheme> for Theme {
    fn from(m: ManabuTheme) -> Self {
        match m {
            ManabuTheme::System => Theme::default(),
            ManabuTheme::Dark => Theme::Dark,
            ManabuTheme::Light => Theme::Light,
            ManabuTheme::Dracula => Theme::Dracula,
            ManabuTheme::Nord => Theme::Nord,
            ManabuTheme::SolarizedLight => Theme::SolarizedLight,
            ManabuTheme::SolarizedDark => Theme::SolarizedDark,
            ManabuTheme::GruvboxLight => Theme::GruvboxLight,
            ManabuTheme::GruvboxDark => Theme::GruvboxDark,
            ManabuTheme::CatppuccinLatte => Theme::CatppuccinLatte,
            ManabuTheme::CatppuccinFrappe => Theme::CatppuccinFrappe,
            ManabuTheme::CatppuccinMacchiato => Theme::CatppuccinMacchiato,
            ManabuTheme::CatppuccinMocha => Theme::CatppuccinMocha,
            ManabuTheme::TokyoNight => Theme::TokyoNight,
            ManabuTheme::TokyoNightStorm => Theme::TokyoNightStorm,
            ManabuTheme::TokyoNightLight => Theme::TokyoNightLight,
            ManabuTheme::KanagawaWave => Theme::KanagawaWave,
            ManabuTheme::KanagawaDragon => Theme::KanagawaDragon,
            ManabuTheme::KanagawaLotus => Theme::KanagawaLotus,
            ManabuTheme::Moonfly => Theme::Moonfly,
            ManabuTheme::Nightfly => Theme::Nightfly,
            ManabuTheme::Oxocarbon => Theme::Oxocarbon,
            ManabuTheme::Ferra => Theme::Ferra,
        }
    }
}

impl fmt::Display for ManabuTheme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Light => write!(f, "Light"),
            Self::Dark => write!(f, "Dark"),
            Self::Dracula => write!(f, "Dracula"),
            Self::Nord => write!(f, "Nord"),
            Self::SolarizedLight => write!(f, "Solarized Light"),
            Self::SolarizedDark => write!(f, "Solarized Dark"),
            Self::GruvboxLight => write!(f, "Gruvbox Light"),
            Self::GruvboxDark => write!(f, "Gruvbox Dark"),
            Self::CatppuccinLatte => write!(f, "Catppuccin Latte"),
            Self::CatppuccinFrappe => write!(f, "Catppuccin Frappé"),
            Self::CatppuccinMacchiato => write!(f, "Catppuccin Macchiato"),
            Self::CatppuccinMocha => write!(f, "Catppuccin Mocha"),
            Self::TokyoNight => write!(f, "Tokyo Night"),
            Self::TokyoNightStorm => write!(f, "Tokyo Night Storm"),
            Self::TokyoNightLight => write!(f, "Tokyo Night Light"),
            Self::KanagawaWave => write!(f, "Kanagawa Wave"),
            Self::KanagawaDragon => write!(f, "Kanagawa Dragon"),
            Self::KanagawaLotus => write!(f, "Kanagawa Lotus"),
            Self::Moonfly => write!(f, "Moonfly"),
            Self::Nightfly => write!(f, "Nightfly"),
            Self::Oxocarbon => write!(f, "Oxocarbon"),
            Self::Ferra => write!(f, "Ferra"),
            Self::System => write!(f, "System"),
        }
    }
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
    SetTheme(ManabuTheme),
    TextScale(f32),
    SaveScale,
    AdditionalKanjiInfo(bool),
    DisplayKanji(bool),
    DisplayKunyomi(bool),
    DisplayOnyomi(bool),
    DisplayNanori(bool),
    DisplayMeaning(bool),
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