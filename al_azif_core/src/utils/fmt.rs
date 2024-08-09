use crate::_prelude::*;

pub fn mark_thousands(num: i64) -> String {
    let mut formatted_num = num
        .abs()
        .to_string()
        .as_bytes()
        .rchunks(3)
        .rev()
        .map(std::str::from_utf8)
        .collect::<std::result::Result<Vec<&str>, _>>()
        .unwrap()
        .join(".");

    if num < 0 {
        formatted_num = format!("-{formatted_num}");
    }

    formatted_num
}

pub fn mark_thousands_and_show_sign(num: i64) -> String {
    let mut formatted_num = mark_thousands(num);
    if num > 0 {
        formatted_num = f!("+{formatted_num}");
    }
    formatted_num
}

pub fn join_with_and(words: &[impl AsRef<str>]) -> String { _join_with_and(words.iter().map(|word| word.as_ref()).collect()) }
fn _join_with_and(words: Vec<&str>) -> String {
    let len = words.len();
    match len {
        0 => String::new(),
        1 => words[0].to_string(),
        2 => format!("{} e {}", words[0], words[1]),
        _ => {
            let (head, tail) = words.split_at(len - 1);
            f!("{} e {}", head.join(", "), tail[0])
        },
    }
}
