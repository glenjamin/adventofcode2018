use std::collections::HashSet;
use std::io::{self, BufRead};

fn main() {
    let numbers = load_numbers();

    let total: i32 = numbers.iter().sum();

    let repeat = calculate_repeat(&numbers);

    println!("Total: {}", total);
    println!("Repeat: {:?}", repeat);
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

fn load_numbers() -> Vec<i32> {
    io::stdin()
        .lock()
        .lines()
        .filter_map(|res| res.ok())
        .filter_map(|s| s.trim().parse::<i32>().ok())
        .collect()
}
