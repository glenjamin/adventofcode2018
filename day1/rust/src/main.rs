use std::collections::HashSet;
use std::io::{self, BufRead};

fn main() {
    let numbers = load_numbers(io::stdin().lock());

    println!("Total: {}", calculate_frequency(&numbers));
    println!("Repeat: {:?}", calculate_repeat(&numbers));
}

fn calculate_frequency(numbers: &[i32]) -> i32 {
    numbers.iter().sum()
}

fn calculate_repeat(numbers: &[i32]) -> i32 {
    let mut frequency = 0;
    let mut seen: HashSet<i32> = HashSet::new();

    for n in numbers.iter().cycle() {
        frequency += n;

        if seen.contains(&frequency) {
            return frequency;
        }

        seen.insert(frequency);
    }

    unreachable!();
}

fn load_numbers(stream: impl BufRead) -> Vec<i32> {
    stream.lines().flatten().flat_map(|s| s.parse()).collect()
}
