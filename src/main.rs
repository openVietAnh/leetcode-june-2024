pub mod solutions;

pub use solutions::*;

fn main() {
    println!("{}", subarrays_div_by_k(vec![7,-5,5,-8,-6,6,-4,7,-8,-7], 7));
}
