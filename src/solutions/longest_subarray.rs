use std::collections::VecDeque;

pub fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
    nums.iter()
        .enumerate()
        .fold(
            (VecDeque::new(), VecDeque::new(), 0usize, 1usize),
            |(mut max, mut min, mut i, ans), (j, &x)| {
                while let Some(y) = max.pop_back() {
                    if y >= x {
                        max.push_back(y);
                        break;
                    }
                }
                max.push_back(x);

                while let Some(y) = min.pop_back() {
                    if y <= x {
                        min.push_back(y);
                        break;
                    }
                }
                min.push_back(x);

                while *max.front().unwrap() - *min.front().unwrap() > limit {
                    if *max.front().unwrap() == nums[i] {
                        max.pop_front();
                    }
                    if *min.front().unwrap() == nums[i] {
                        min.pop_front();
                    }
                    i += 1;
                }
                (max, min, i, ans.max(j - i + 1))
            },
        )
        .3 as i32
}
