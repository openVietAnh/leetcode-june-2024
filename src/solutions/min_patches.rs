pub fn min_patches(nums: Vec<i32>, n: i32) -> i32 {
    let mut target: i64 = 1;
    let mut count = 0;
    let mut i = 0;
    while target <= n as i64 {
        if i < nums.len() && nums[i] as i64 <= target {
            target += nums[i] as i64;
            i += 1;
        } else {
            count += 1;
            target += target;
        }
    }
    count
}
