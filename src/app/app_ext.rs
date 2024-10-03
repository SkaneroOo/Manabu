use rand::Rng;

use super::Manabu;

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
}