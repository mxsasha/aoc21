use std::io::{self, BufRead, Lines};
use std::process;
use math::round;

#[derive(Default, Debug, PartialEq)]
struct Report {
    gamma: isize,
    epsilon: isize,
    oxygen: isize,
    co2: isize,
}

fn determine_majorities(rows: &Vec<Vec<char>>, width: usize) -> Vec<usize> {
    let mut one_count = vec![0; width];
    for row in rows {
        for i in 0..width {
            if row[i] == '1' {
                one_count[i] += 1;
            }
        }
    }
    one_count
}

fn diagnose<B: BufRead>(lines: Lines<B>) -> Result<Report, String> {
    let rows: Vec<Vec<char>> = lines
        .map(|line| line.as_ref().unwrap().trim().chars().collect())
        .collect();
    let width = rows[0].len();
    let height = rows.len();
    // let flattened: Vec<char> = rows.into_iter().flatten().collect();

    // let mut output: Vec<Vec<char>> = vec![vec![]; width];
    let one_count = determine_majorities(&rows, width);

    let mut gamma_binary = String::new();
    let mut epsilon_binary = String::new();

    for idx in 0..width {
        let majority_one: bool = one_count[idx] > (height / 2);
        if majority_one {
            gamma_binary.push('1');
            epsilon_binary.push('0');
        } else {
            gamma_binary.push('0');
            epsilon_binary.push('1');
        }
    }

    let mut filtered_rows = rows.clone();
    for idx in 0..width {
        let filtered_one_count = determine_majorities(&filtered_rows, width);
        let majority_one = filtered_one_count[idx] >= (round::ceil(filtered_rows.len() as f64 / 2f64, 0) as usize);
        let mut filter_char = '0';
        if majority_one {
            filter_char = '1';
        }
        filtered_rows = filtered_rows.iter().filter(|row| row[idx] == filter_char).cloned().collect();
        if filtered_rows.len() == 1 {
            break;
        }
    }
    let oxygen_binary: String = filtered_rows[0].clone().into_iter().collect();
    let oxygen = isize::from_str_radix(&oxygen_binary, 2).unwrap();

    let mut filtered_rows = rows.clone();
    for idx in 0..width {
        let filtered_one_count = determine_majorities(&filtered_rows, width);
        let majority_one = filtered_one_count[idx] >= (round::ceil(filtered_rows.len() as f64 / 2f64, 0) as usize);
        let mut filter_char = '1';
        if majority_one {
            filter_char = '0';
        }
        filtered_rows = filtered_rows.iter().filter(|row| row[idx] == filter_char).cloned().collect();
        if filtered_rows.len() == 1 {
            break;
        }
    }
    let co2_binary: String = filtered_rows[0].clone().into_iter().collect();
    let co2 = isize::from_str_radix(&co2_binary, 2).unwrap();

    let gamma = isize::from_str_radix(&gamma_binary, 2).unwrap();
    let epsilon = isize::from_str_radix(&epsilon_binary, 2).unwrap();
    Ok(Report {
        gamma,
        epsilon,
        co2,
        oxygen,
    })
}

fn main() {
    let stdin = io::stdin();
    match diagnose(stdin.lock().lines()) {
        Ok(report) => println!(
            "Final report: {:?} - gamma*epsilon {} - oxygen*co2 {}",
            report,
            report.gamma * report.epsilon,
            report.oxygen * report.co2,
        ),
        Err(error) => {
            eprintln!("{}", error);
            process::exit(1);
        }
    };
}

#[cfg(test)]
mod tests {
    use super::diagnose;
    use std::io::{BufRead, Cursor};
    #[test]
    fn it_works() {
        let lines = Cursor::new(String::from(
            "00100
            11110
            10110
            10111
            10101
            01111
            00111
            11100
            10000
            11001
            00010
            01010",
        ));
        let report = diagnose(lines.lines()).unwrap();
        assert_eq!(report.gamma, 22);
        assert_eq!(report.epsilon, 9);
        assert_eq!(report.oxygen, 23);
        assert_eq!(report.co2, 10);
    }
}
