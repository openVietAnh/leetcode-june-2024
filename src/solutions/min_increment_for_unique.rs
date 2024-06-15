pub fn min_increment_for_unique(mut nums: Vec<i32>) -> i32 {
    let mut result = 0;
    nums.sort();
    for i in 1..nums.len() {
        if nums[i] <= nums[i - 1] {
            result += nums[i - 1] - nums[i] + 1;
            nums[i] = nums[i - 1] + 1;
        }
    }
    result
}
