use std::collections::HashMap;

pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
    let mut modulo: HashMap<i32, usize> = HashMap::new();
    let mut prefix_sum = 0;
    for i in 0..nums.len() {
        prefix_sum += nums[i];
        if prefix_sum % k == 0 && i > 0 {
            return true;
        } else {
            if modulo.contains_key(&(prefix_sum % k)) {
                if i - *modulo.get(&(prefix_sum % k)).unwrap() > 1 {
                    return true;
                }
            } else {
                modulo.insert(prefix_sum % k, i);
            }
        }
    }
    false
}
