pub mod math;

pub fn mark_thousands(num: i64) -> String {
    let mut formatted_num = num
        .abs()
        .to_string()
        .as_bytes()
        .rchunks(3)
        .rev()
        .map(std::str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap()
        .join(".");

    if num < 0 {
        formatted_num = format!("-{formatted_num}");
    }

    formatted_num
}