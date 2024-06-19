pub mod solutions;

pub use solutions::*;

fn main() {
    println!("{}", min_days(vec![1, 10, 3, 10, 2], 3, 1));
    println!("{}", min_days(vec![7, 7, 7, 7, 12, 7, 7], 2, 3));
}
