use std::io::{self, BufRead};
use std::process;

fn main() {
    let mut increases_count = 0;
    let mut values: Vec<u32> = Vec::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let current_depth = match line.as_ref().unwrap().parse::<u32>() {
            Ok(depth) => depth,
            Err(error) => {
                eprintln!("Invalid input: '{}': {}", line.unwrap(), error);
                process::exit(1);
            }
        };
        values.push(current_depth);
    }
    if values.len() >= 3 {
        for starting_idx in 0..values.len() - 3 {
            let sum1: u32 = values[starting_idx..starting_idx + 3].iter().sum();
            let sum2: u32 = values[starting_idx + 1..starting_idx + 4].iter().sum();
            if sum2 > sum1 {
                increases_count += 1;
            }
        }
    }
    println!("Found {} increases", increases_count);
}
