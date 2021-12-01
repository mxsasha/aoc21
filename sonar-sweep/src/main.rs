use std::io::{self, BufRead};
use std::process;


fn main() {
    let mut increases_count = 0;
    let mut last_depth = None;

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let current_depth = match line.as_ref().unwrap().parse::<u32>() {
            Ok(depth) => depth,
            Err(error) => {
                eprintln!("Invalid input: '{}': {}", line.unwrap(), error);
                process::exit(1);
            },
        };
        if let Some(l) = last_depth {
            if l < current_depth {
                increases_count += 1;
            }
        }
        
        last_depth = Some(current_depth);
    }
    println!("Found {} increases", increases_count);
}
