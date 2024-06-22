use std::collections::HashMap;

pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
    let mut count_prefix: HashMap<i32, i32> = HashMap::new();
    let mut odd = 0;
    let mut result = 0;
    for item in nums {
        odd += if item % 2 == 1 { 1 } else { 0 };
        count_prefix.entry(odd).and_modify(|v| *v += 1).or_insert(1);
        if odd == k {
            result += 1;
        }
        if count_prefix.contains_key(&(odd - k)) {
            result += *count_prefix.get(&(odd - k)).unwrap();
        }
    }
    result
}
