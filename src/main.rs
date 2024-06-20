pub mod solutions;

pub use solutions::*;

fn main() {
    println!("{}", max_distance(vec![1, 2, 3, 4, 7], 3));
    println!("{}", max_distance(vec![79, 74, 57, 22], 4));
}
