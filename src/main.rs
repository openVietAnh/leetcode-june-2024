pub mod solutions;

pub use solutions::*;

fn main() {
    println!("{}", score_of_string(String::from("hello")));
}
