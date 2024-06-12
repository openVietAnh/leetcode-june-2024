use std::collections::HashMap;

pub fn sort_colors(nums: &mut Vec<i32>) {
    let mut count: HashMap<i32, usize> = HashMap::from([(0, 0), (1, 0), (2, 0)]);
    for item in nums.clone() {
        *count.get_mut(&item).unwrap() += 1;
    }
    for i in 0..*count.get(&0).unwrap() {
        nums[i] = 0;
    }
    for i in *count.get(&0).unwrap()..*count.get(&0).unwrap() + *count.get(&1).unwrap() {
        nums[i] = 1;
    }
    for i in (*count.get(&0).unwrap() + *count.get(&1).unwrap())..nums.len() {
        nums[i] = 2;
    }
}
