#[macro_export]
macro_rules! lang_diff {
    ($bot:expr, en: $en:expr, pt: $pt:expr) => {
        match $bot.get_lang() {
            Lang::En => $en,
            Lang::Pt => $pt,
        }
    };
    ($bot:expr, en: $en:expr, pt: $pt:expr,) => {
        match $bot.get_lang() {
            Lang::En => $en,
            Lang::Pt => $pt,
        }
    };
}
