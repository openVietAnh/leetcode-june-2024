use std::collections::HashMap;

pub fn subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32 {
    let mut count: HashMap<i32, i32> = HashMap::new();
    let mut result = 0;
    let mut sum = 0;
    for item in nums {
        sum += item;
        if sum % k == 0 {
            result += 1;
        }
        result += match count.get(&(sum % k)) {
            Some(value) => { value },
            None => { &0 },
        };
        if sum % k < 0 {
            result += match count.get(&(k + (sum % k))) {
                Some(value) => { value },
                None => { &0 },
            };
        }
        result += match count.get(&(sum % k - k)) {
            Some(value) => { value },
            None => { &0 },
        };
        count.entry(sum % k).and_modify(|v| *v += 1).or_insert(1);
    }
    result
}