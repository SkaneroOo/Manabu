use std::path::Path;

use super::constants::hiragana::*;
use super::{AnswerSample, KanjiEntry, Message, Settings};
use iced::widget::Row;
use rand::RngCore;
use similar::{ChangeTag, TextDiff};

#[deprecated]
#[allow(dead_code)]
pub fn generate_hiragana(length: usize) -> String {
    let mut random_word = String::new();
    let mut rng = rand::thread_rng();
    let mut prev = 0;
    for i in 0..length {
        let mut c = rng.next_u32() % (0x3093-0x3040) + 0x3041;
        if c == CHISAI_TSU && i == length - 1{
            c += 1;
        }
        if CHISAI_YS.contains(&c) && !COMBINING.contains(&prev) {
            c += 1;
        }
        if CHISAI_VOWELS.contains(&c) {
            c += 1;
        }
        if CHISAI_VOWELS.contains(&(c - 1)) && prev == CHISAI_TSU {
            c += 9;
        }
        if prev == CHISAI_TSU && c >= 0x3083 {
            c -= 17;
        }
        if UNUSED_KANA.contains(&c) {
            c -= 4;
        }
        random_word.push(char::from_u32(c).unwrap());
        prev = c;
    }
    random_word
}

#[deprecated]
#[allow(dead_code)]
pub fn generate_katakana(length: usize) -> String {
    let mut random_word = String::new();
    let mut rng = rand::thread_rng();
    let mut prev = 0;
    for i in 0..length {
        let mut c = rng.next_u32() % (0x3093-0x3040) + 0x3041;
        if c == CHISAI_TSU && i == length - 1{
            c += 1;
        }
        if CHISAI_YS.contains(&c) && !COMBINING.contains(&prev) {
            c += 1;
        }
        if CHISAI_VOWELS.contains(&c) {
            c += 1;
        }
        if CHISAI_VOWELS.contains(&(c - 1)) && prev == CHISAI_TSU {
            c += 9;
        }
        if prev == CHISAI_TSU && c >= 0x3083 {
            c -= 17;
        }
        if UNUSED_KANA.contains(&c) {
            c -= 4;
        }
        random_word.push(char::from_u32(c + 0x60).unwrap());
        prev = c;
    }
    random_word
}

pub fn load_config() -> Settings {
    let app_path = dirs::config_local_dir().unwrap().join("manabu");
    if !Path::new(&app_path).exists() {
        std::fs::create_dir(&app_path).unwrap();
    }
    let cfg_path = app_path.join("config.json");
    if !Path::new(&cfg_path).exists() {
        save_config(&Settings::default());
    }
    match std::fs::read_to_string(cfg_path) {
        Ok(s) => serde_json::from_str(&s).unwrap_or(Settings::default()),
        Err(_) => Settings::default()
    }
}

pub fn save_config(settings: &Settings) {
    let app_path = dirs::config_local_dir().unwrap().join("manabu");
    if !Path::new(&app_path).exists() {
        std::fs::create_dir(&app_path).unwrap();
    }
    let cfg_path = app_path.join("config.json");
    std::fs::write(cfg_path, serde_json::to_string_pretty(settings).unwrap()).unwrap();
}

pub struct ColoredText<'a> {
    inner: Row<'a, Message>,
    size: f32
}

impl<'a> ColoredText<'a> {
    pub fn new(size: f32) -> Self {
        ColoredText { 
            inner: Row::new(),
            size
        }
    }

    pub fn push(self, text: &str, r: u8, g: u8, b: u8) -> Self {
        ColoredText {
            inner: self.inner.push(iced::widget::Text::new(text.to_owned()).style(move |_|
                iced::widget::text::Style{color: Some(iced::Color::from_rgb8(r, g, b))}
            ).size(self.size)),
            ..self
        }
    }

    pub fn finalize(self) -> iced::widget::Row<'a, Message> {
        self.inner
    }
}

pub fn check_correctness(text: &str, answer: &str) -> (bool, Vec<AnswerSample>, Vec<AnswerSample>) {
    let diff = TextDiff::from_chars(text, answer);

    let mut correct = true;
    let mut samples_text: Vec<AnswerSample> = Vec::new();
    let mut samples_answer: Vec<AnswerSample> = Vec::new();
    let mut sample = String::new();
    let mut last_correctness = false;
    let mut last_insert = false;
    for change in diff.iter_all_changes() {
        match change.tag() {
            ChangeTag::Equal => {
                if last_correctness {
                    sample.push(change.to_string().chars().nth(0).unwrap());
                } else {
                    last_correctness = true;
                    if last_insert {
                        if !sample.is_empty() {
                            if let Some(last_sample) = samples_answer.last_mut() {
                                if (*last_sample).text == sample && (*last_sample).correct {
                                    (*last_sample).correct = false;
                                    samples_answer.push(AnswerSample { text: sample, correct: true });
                                } else {
                                    samples_answer.push(AnswerSample { text: sample, correct: false });
                                }
                            } else {
                                samples_answer.push(AnswerSample { text: sample, correct: false });
                            }
                        }
                    } else {
                        if !sample.is_empty() {
                            samples_text.push(AnswerSample { text: sample, correct: false });
                        }
                    }
                    sample = String::new();
                    sample.push(change.to_string().chars().nth(0).unwrap());
                }
            }
            ChangeTag::Delete => {
                correct = false;
                if !last_correctness {
                    if last_insert {
                        last_insert = false;
                        samples_answer.push(AnswerSample { text: sample, correct: false });
                        sample = String::new();
                        sample.push(change.to_string().chars().nth(0).unwrap());
                    } else {
                        sample.push(change.to_string().chars().nth(0).unwrap());
                    }
                } else {
                    last_insert = false;
                    last_correctness = false;
                    samples_text.push(AnswerSample { text: sample.clone(), correct: true });
                    samples_answer.push(AnswerSample { text: sample, correct: true });
                    sample = String::new();
                    sample.push(change.to_string().chars().nth(0).unwrap());
                }
            }
            ChangeTag::Insert => {
                correct = false;
                if !last_correctness {
                    
                    if !last_insert {
                        last_insert = true;
                        if !sample.is_empty() {
                            samples_text.push(AnswerSample { text: sample, correct: false });
                        }
                        sample = String::new();
                        sample.push(change.to_string().chars().nth(0).unwrap());
                    } else {
                        sample.push(change.to_string().chars().nth(0).unwrap());
                    }
                } else {
                    last_insert = true;
                    last_correctness = false;
                    samples_text.push(AnswerSample { text: sample.clone(), correct: true });
                    samples_answer.push(AnswerSample { text: sample, correct: true });
                    sample = String::new();
                    sample.push(change.to_string().chars().nth(0).unwrap());
                }
            }
        }
    }

    if !sample.is_empty() {
        if last_correctness {
            samples_text.push(AnswerSample { text: sample.clone(), correct: true });
            samples_answer.push(AnswerSample { text: sample, correct: true });
        } else {
            if last_insert {
                samples_answer.push(AnswerSample { text: sample, correct: false });
            } else {
                samples_text.push(AnswerSample { text: sample, correct: false });
            }
        }
    }

    (correct, samples_answer, samples_text)
}

pub async fn load_kanji() -> Option<Vec<KanjiEntry>> {
    let app_path = dirs::config_local_dir().unwrap().join("manabu");
    let cfg_path = app_path.join("kanji.json");
    match std::fs::read_to_string(cfg_path) {
        Ok(s) => match serde_json::from_str(&s) {
            Ok(kanji) => Some(kanji),
            Err(_) => None
        },
        Err(_) => None
    }
}