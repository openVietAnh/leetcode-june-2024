pub fn score_of_string(s: String) -> i32 {
    let mut result = 0;
    let chars: Vec<char> = s.chars().collect();
    for i in 0..chars.len() - 1 {
        result += (chars[i] as u8).abs_diff(chars[i + 1] as u8) as i32;
    }
    result
}