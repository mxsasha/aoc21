use std::io::{self, BufRead, Lines};
use std::process;

fn find_increases<B: BufRead>(lines: Lines<B>) -> Result<u32, String> {
    let mut increases_count = 0;
    let mut values: Vec<u32> = Vec::new();

    for line in lines {
        let current_depth = match line.as_ref().unwrap().replace(" ", "").parse::<u32>() {
            Ok(depth) => depth,
            Err(error) => {
                return Err(format!("Invalid input: '{}': {}", line.unwrap(), error));
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
    Ok(increases_count)
}

fn main() {
    let stdin = io::stdin();
    match find_increases(stdin.lock().lines()) {
        Ok(increases_count) => println!("Found {} increases", increases_count),
        Err(error) => {
            eprintln!("{}", error);
            process::exit(1);
        }
    };
}

#[cfg(test)]
mod tests {
    use super::find_increases;
    use std::io::{BufRead, Cursor};
    #[test]
    fn it_works() {
        let lines = Cursor::new(String::from(
            "199
            200
            208
            210
            200
            207
            240
            269
            260
            263",
        ));
        let increases_count = find_increases(lines.lines()).unwrap();
        assert_eq!(increases_count, 5);
    }
}
