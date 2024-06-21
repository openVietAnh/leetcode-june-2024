pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
    let mut result = 0;
    let mut sum = 0;
    for i in 0..customers.len() {
        if grumpy[i] != 1 {
            sum += customers[i];
        }
    }
    for i in 0..minutes as usize {
        if grumpy[i] == 1 {
            sum += customers[i];
        }
    }
    result = std::cmp::max(result, sum);
    for i in minutes as usize..customers.len() {
        if grumpy[i] == 1 {
            sum += customers[i];
        }
        if grumpy[i - minutes as usize] == 1 {
            sum -= customers[i - minutes as usize];
        }
        result = std::cmp::max(result, sum);
    }
    result
}
