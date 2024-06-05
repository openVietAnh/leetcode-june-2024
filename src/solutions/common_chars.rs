use std::collections::HashMap;

pub fn common_chars(words: Vec<String>) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    let mut count: Vec<HashMap<char, usize>> = vec![HashMap::new(); words.len()];
    for i in 0..words.len() - 1 {
        for c in words[i].chars() {
            count[i].entry(c).and_modify(|v| *v += 1).or_insert(1);
        }
    }
    for c in words.last().unwrap().chars() {
        let mut valid = true;
        for i in 0..words.len() - 1 {
            if !count[i].contains_key(&c) || *count[i].get(&c).unwrap() == 0 {
                valid = false;
                break;
            }
        }
        if valid {
            result.push(c.to_string());
            for i in 0..words.len() - 1 {
                *count[i].get_mut(&c).unwrap() -= 1;
            }
        }
    }
    result
}
