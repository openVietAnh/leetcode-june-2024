pub fn min_moves_to_seat(mut seats: Vec<i32>, mut students: Vec<i32>) -> i32 {
    seats.sort();
    students.sort();
    (0..seats.len())
        .map(|i| seats[i].abs_diff(students[i]) as i32)
        .sum()
}
