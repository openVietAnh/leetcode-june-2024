pub fn height_checker(heights: Vec<i32>) -> i32 {
    let mut sorted = heights.clone();
    sorted.sort();
    (0..heights.len()).into_iter().filter(
        |i| heights[*i] != sorted[*i]
    ).count() as i32
}