pub fn judge_square_sum(c: i32) -> bool {
    let upper_limit = (c as f64).sqrt();
    for a in 0..=(upper_limit.floor() as i32) {
        let b = c - a * a;
        if b >= 0 {
            let sqrt_b = (b as f64).sqrt();
            if sqrt_b == sqrt_b.floor() {
                return true;
            }
        }
    }
    false
}
