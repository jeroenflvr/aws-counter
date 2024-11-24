pub fn bytes_to_human_readable_string(bytes: i64) -> String {
    if bytes == 0 {
        return "0 B".to_string();
    }

    let kb = 1024f64;

    let units = ["B", "KB", "MB", "GB", "TB", "PB"];
    let index = (bytes as f64).log(kb).floor() as i32;
    let readable = bytes as f64 / kb.powi(index);

    return format!("{:.2} {}", readable, units[index as usize]);
}
