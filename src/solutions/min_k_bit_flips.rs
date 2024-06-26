pub fn min_k_bit_flips(nums: Vec<i32>, k: i32) -> i32 {
    let mut result = 0;
    let mut is_flipped = vec![0; nums.len()];
    let mut flipped = 0;
    for i in 0..nums.len() {
        if i >= k as usize {
            flipped ^= is_flipped[i - k as usize];
        }
        if flipped == nums[i] {
            if i + k as usize > nums.len() {
                return -1;
            }
            is_flipped[i] = 1;
            flipped ^= 1;
            result += 1;
        }
    }
    result
}
