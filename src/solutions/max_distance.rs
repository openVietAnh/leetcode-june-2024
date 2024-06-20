pub fn range_valid(position: &Vec<i32>, m: i32, range: i32) -> bool {
    let mut count = 1;
    let mut current = 0;
    while count < m {
        let mut ind = current + 1;
        while ind < position.len() && position[ind] - position[current] < range {
            ind += 1;
        }
        if ind == position.len() {
            return false;
        } else {
            count += 1;
            current = ind;
        }
    }
    true
}

pub fn max_distance(mut position: Vec<i32>, m: i32) -> i32 {
    position.sort();
    let mut low = 1;
    let mut high = position.last().unwrap() - position[0];
    while low < high {
        let mid = low + (high - low) / 2;
        println!("{} {} {}", low, mid, high);
        if !range_valid(&position, m, mid) {
            if mid == low {
                if range_valid(&position, m, low - 1) {
                    return low - 1;
                }
            }
            high = mid;
        } else {
            if mid + 1 == high {
                if !range_valid(&position, m, mid + 1) {
                    return low;
                }
            }
            low = mid + 1;
        }
    }
    low
}
