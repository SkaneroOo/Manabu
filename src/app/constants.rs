#[deprecated]
pub mod hiragana {
    pub const COMBINING: [u32; 12] = [
        0x304D, // ki
        0x3057, // shi
        0x3061, // chi
        0x306B, // ni
        0x3072, // hi
        0x307F, // mi
        0x308A, // ri
        0x304E, // gi
        0x3058, // ji
        0x3062, // ji (chi)
        0x3073, // bi
        0x3074, // pi
    ];

    pub const CHISAI_TSU: u32 = 0x3063;

    pub const CHISAI_YS: [u32; 3] = [
        0x3083, // ya
        0x3085, // yu
        0x3087  // yo
    ];

    pub const CHISAI_VOWELS: [u32; 5] = [
        0x3041, // a
        0x3043, // i
        0x3045, // u
        0x3047, // e
        0x3049, // o
    ];

    pub const UNUSED_KANA: [u32; 3] = [
        0x308E,
        0x3090,
        0x3091
    ];
}
