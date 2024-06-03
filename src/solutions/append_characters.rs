pub fn append_characters(s: String, t: String) -> i32 {
    let (c1, c2): (Vec<char>, Vec<char>) = (s.chars().collect(), t.chars().collect());
    let mut ind = 0;
    for i in 0..c1.len() {
        if c1[i] == c2[ind] {
            ind += 1;
            if ind == c2.len() {
                break;
            }
        }
    }
    (c2.len() - ind) as i32
}
