
pub mod kana {
    pub struct Kana {
        pub romanji: &'static str,
        pub kana: &'static str
    }
    
    pub const HIRAGANA: [Kana; 46] = [
        Kana { romanji: "a", kana: "あ" },
        Kana { romanji: "i", kana: "い" },
        Kana { romanji: "u", kana: "う" },
        Kana { romanji: "e", kana: "え" },
        Kana { romanji: "o", kana: "お" },
        Kana { romanji: "ka", kana: "か" },
        Kana { romanji: "ki", kana: "き" },
        Kana { romanji: "ku", kana: "く" },
        Kana { romanji: "ke", kana: "け" },
        Kana { romanji: "ko", kana: "こ" },
        Kana { romanji: "sa", kana: "さ" },
        Kana { romanji: "shi", kana: "し" },
        Kana { romanji: "su", kana: "す" },
        Kana { romanji: "se", kana: "せ" },
        Kana { romanji: "so", kana: "そ" },
        Kana { romanji: "ta", kana: "た" },
        Kana { romanji: "chi", kana: "ち" },
        Kana { romanji: "tsu", kana: "つ" },
        Kana { romanji: "te", kana: "て" },
        Kana { romanji: "to", kana: "と" },
        Kana { romanji: "na", kana: "な" },
        Kana { romanji: "ni", kana: "に" },
        Kana { romanji: "nu", kana: "ぬ" },
        Kana { romanji: "ne", kana: "ね" },
        Kana { romanji: "no", kana: "の" },
        Kana { romanji: "ha", kana: "は" },
        Kana { romanji: "hi", kana: "ひ" },
        Kana { romanji: "fu", kana: "ふ" },
        Kana { romanji: "he", kana: "へ" },
        Kana { romanji: "ho", kana: "ほ" },
        Kana { romanji: "ma", kana: "ま" },
        Kana { romanji: "mi", kana: "み" },
        Kana { romanji: "mu", kana: "む" },
        Kana { romanji: "me", kana: "め" },
        Kana { romanji: "mo", kana: "も" },
        Kana { romanji: "ya", kana: "や" },
        Kana { romanji: "yu", kana: "ゆ" },
        Kana { romanji: "yo", kana: "よ" },
        Kana { romanji: "ra", kana: "ら" },
        Kana { romanji: "ri", kana: "り" },
        Kana { romanji: "ru", kana: "る" },
        Kana { romanji: "re", kana: "れ" },
        Kana { romanji: "ro", kana: "ろ" },
        Kana { romanji: "wa", kana: "わ" },
        Kana { romanji: "wo", kana: "を" },
        Kana { romanji: "n", kana: "ん" }
    ];

    pub const KATAKANA: [Kana; 46] = [
        Kana { romanji: "a", kana: "ア" },
        Kana { romanji: "i", kana: "イ" },
        Kana { romanji: "u", kana: "ウ" },
        Kana { romanji: "e", kana: "エ" },
        Kana { romanji: "o", kana: "オ" },
        Kana { romanji: "ka", kana: "カ" },
        Kana { romanji: "ki", kana: "キ" },
        Kana { romanji: "ku", kana: "ク" },
        Kana { romanji: "ke", kana: "ケ" },
        Kana { romanji: "ko", kana: "コ" },
        Kana { romanji: "sa", kana: "サ" },
        Kana { romanji: "shi", kana: "シ" },
        Kana { romanji: "su", kana: "ス" },
        Kana { romanji: "se", kana: "セ" },
        Kana { romanji: "so", kana: "ソ" },
        Kana { romanji: "ta", kana: "タ" },
        Kana { romanji: "chi", kana: "チ" },
        Kana { romanji: "tsu", kana: "ツ" },
        Kana { romanji: "te", kana: "テ" },
        Kana { romanji: "to", kana: "ト" },
        Kana { romanji: "na", kana: "ナ" },
        Kana { romanji: "ni", kana: "ニ" },
        Kana { romanji: "nu", kana: "ヌ" }, 
        Kana { romanji: "ne", kana: "ネ" },
        Kana { romanji: "no", kana: "ノ" },
        Kana { romanji: "ha", kana: "ハ" },
        Kana { romanji: "hi", kana: "ヒ" },
        Kana { romanji: "fu", kana: "フ" },
        Kana { romanji: "he", kana: "ヘ" },
        Kana { romanji: "ho", kana: "ホ" },
        Kana { romanji: "ma", kana: "マ" },
        Kana { romanji: "mi", kana: "ミ" },
        Kana { romanji: "mu", kana: "ム" },
        Kana { romanji: "me", kana: "メ" },
        Kana { romanji: "mo", kana: "モ" },
        Kana { romanji: "ya", kana: "ヤ" },
        Kana { romanji: "yu", kana: "ユ" },
        Kana { romanji: "yo", kana: "ヨ" },
        Kana { romanji: "ra", kana: "ラ" },
        Kana { romanji: "ri", kana: "リ" },
        Kana { romanji: "ru", kana: "ル" },
        Kana { romanji: "re", kana: "レ" },
        Kana { romanji: "ro", kana: "ロ" },
        Kana { romanji: "wa", kana: "ワ" },
        Kana { romanji: "wo", kana: "ヲ" },
        Kana { romanji: "n", kana: "ン" }
    ];
}