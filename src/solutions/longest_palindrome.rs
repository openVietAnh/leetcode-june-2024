use std::collections::HashMap;

pub fn longest_palindrome(s: String) -> i32 {
    let mut result = 0;
    let mut count: HashMap<char, i32> = HashMap::new();
    for c in s.chars() {
        count.entry(c).and_modify(|v| *v += 1).or_insert(1);
    }
    for v in count.values() {
        result += *v / 2 * 2;
    }
    if result < s.len() as i32 {
        result + 1
    } else {
        result
    }
}
