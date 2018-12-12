use std::io::{self, BufRead};

fn main() {
    let n = io::stdin()
        .lock()
        .lines()
        .filter_map(|res| res.ok())
        .filter_map(|s| s.trim().parse().ok())
        .fold(0, |acc, x: i32| acc + x);
    println!("Total: {}", n);
}
