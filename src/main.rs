pub mod solutions;

pub use solutions::*;

fn main() {
    println!(
        "{}",
        find_maximized_capital(2, 0, vec![2, 3, 3], vec![0, 2, 1])
    );
}
