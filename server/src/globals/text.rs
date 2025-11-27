pub fn crop_string(s: Option<String>, max_len: usize) -> Option<String> {
    if s.is_none() {
        return None;
    }

    let s = s.unwrap();
    if s.len() <= max_len {
        Some(s.to_string())
    } else {
        let mut output = s;
        output.truncate(max_len);
        return Some(output);
    }
}