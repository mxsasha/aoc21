use std::io::{self, BufRead, Lines};
use std::process;

#[derive(Default, Debug, PartialEq)]
struct Location {
    position: u32,
    depth: u32,
}

impl Location {
    fn new() -> Self {
        Default::default()
    }
}

fn dive<B: BufRead>(lines: Lines<B>) -> Result<Location, String> {
    let mut location = Location::new();
    let mut aim = 0;

    for line in lines {
        let v: Vec<&str> = line.as_ref().unwrap().trim().split(' ').collect();
        let (command, parameter_str) = (v[0], v[1]);

        let parameter = match parameter_str.parse::<u32>() {
            Ok(parameter) => parameter,
            Err(_error) => {
                return Err(format!("Invalid input for parameter: '{}'", parameter_str,));
            }
        };
        match command {
            "forward" => {
                location.position += parameter;
                location.depth += parameter * aim;
            }
            "down" => aim += parameter,
            "up" => aim -= parameter,
            _ => return Err(format!("Unknown command: {}", command)),
        }
    }
    Ok(location)
}

fn main() {
    let stdin = io::stdin();
    match dive(stdin.lock().lines()) {
        Ok(location) => println!(
            "Final location: {:?} - multiplied is {}",
            location,
            location.position * location.depth
        ),
        Err(error) => {
            eprintln!("{}", error);
            process::exit(1);
        }
    };
}

#[cfg(test)]
mod tests {
    use super::dive;
    use std::io::{BufRead, Cursor};
    #[test]
    fn it_works() {
        let lines = Cursor::new(String::from(
            "forward 5
            down 5
            forward 8
            up 3
            down 8
            forward 2",
        ));
        let location = dive(lines.lines()).unwrap();
        assert_eq!(location.position, 15);
        assert_eq!(location.depth, 60);
    }
    #[test]
    fn invalid_command() {
        let lines = Cursor::new(String::from("unknown 5"));
        let result = dive(lines.lines());
        assert_eq!(result, Err(String::from("Unknown command: unknown")));
    }
    #[test]

    fn invalid_parameter() {
        let lines = Cursor::new(String::from("forward invalid"));
        let result = dive(lines.lines());
        assert_eq!(
            result,
            Err(String::from("Invalid input for parameter: 'invalid'"))
        );
    }
}
