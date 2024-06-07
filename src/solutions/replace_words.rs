use std::collections::HashSet;

pub fn replace_words(dictionary: Vec<String>, sentence: String) -> String {
    let roots: HashSet<String> = HashSet::from_iter(dictionary.into_iter());
    let mut words: Vec<String> = sentence.split(' ').map(|w| w.to_string()).collect();
    for i in 0..words.len() {
        let mut cur = String::from("");
        for c in words[i].clone().chars() {
            cur += &c.to_string();
            if roots.contains(&cur) {
                words[i] = cur;
                break;
            }
        }
    }
    words.join(" ")
}
