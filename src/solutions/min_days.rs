pub fn valid(flower: &Vec<i32>, m: i32, k: i32, day: i32) -> bool {
    let mut groups = 0;
    let mut cur = 0;
    for item in flower {
        if item <= &day {
            cur += 1;
        } else {
            groups += cur / k;
            cur = 0;
        }
    }
    groups + cur / k >= m
}

pub fn min_days(bloom_day: Vec<i32>, m: i32, k: i32) -> i32 {
    if m as u64 * k as u64 > bloom_day.len() as u64 {
        return -1;
    }
    let max = bloom_day.clone().into_iter().max().unwrap();
    let mut low = 1;
    let mut high = max;
    while low < high {
        let mid = low + (high - low) / 2;
        if valid(&bloom_day, m, k, mid) {
            high = mid
        } else {
            low = mid + 1;
        }
    }
    low
}
