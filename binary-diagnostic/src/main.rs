use std::io::{self, Read};
use std::process;
use std::str::Lines;

#[derive(Default, Debug, PartialEq)]
struct Report {
    gamma: usize,
    epsilon: usize,
    oxygen: usize,
    co2: usize,
}

enum LifeSupportMetric {
    CO2,
    Oxygen,
}
fn one_count_per_column(rows: &[Vec<char>], width: usize) -> Vec<usize> {
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

fn life_support_rating(rows: &[Vec<char>], width: usize, metric: LifeSupportMetric) -> usize {
    let mut filtered_rows = rows.to_owned();
    for idx in 0..width {
        let filtered_one_count = one_count_per_column(&filtered_rows, width);
        let majority_one =
            filtered_one_count[idx] >= ((filtered_rows.len() as f64 / 2f64).ceil() as usize);
        let filter_char = match metric {
            LifeSupportMetric::CO2 => {
                if majority_one {
                    '0'
                } else {
                    '1'
                }
            }
            LifeSupportMetric::Oxygen => {
                if majority_one {
                    '1'
                } else {
                    '0'
                }
            }
        };
        println!(
            "about to filter {} rows for {} at {}",
            filtered_rows.len(),
            filter_char,
            idx
        );

        filtered_rows = filtered_rows
            .iter()
            .filter(|row| row[idx] == filter_char)
            .cloned()
            .collect();
        println!("remaning after filter {} rows", filtered_rows.len());
        if filtered_rows.len() == 1 {
            break;
        }
    }
    let binary: String = filtered_rows[0].clone().into_iter().collect();
    usize::from_str_radix(&binary, 2).unwrap()
}

fn diagnose(lines: Lines) -> Result<Report, String> {
    let rows: Vec<Vec<char>> = lines.map(|line| line.trim().chars().collect()).collect();
    let width = rows[0].len();
    let height = rows.len();

    let one_count = one_count_per_column(&rows, width);

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

    let gamma = usize::from_str_radix(&gamma_binary, 2).unwrap();
    let epsilon = usize::from_str_radix(&epsilon_binary, 2).unwrap();
    Ok(Report {
        gamma,
        epsilon,
        co2: life_support_rating(&rows, width, LifeSupportMetric::CO2),
        oxygen: life_support_rating(&rows, width, LifeSupportMetric::Oxygen),
    })
}

fn main() {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    match diagnose(input.lines()) {
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
    #[test]
    fn it_works() {
        let lines = String::from(
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
        );
        let report = diagnose(lines.lines()).unwrap();
        assert_eq!(report.gamma, 22);
        assert_eq!(report.epsilon, 9);
        assert_eq!(report.oxygen, 23);
        assert_eq!(report.co2, 10);
    }
}
