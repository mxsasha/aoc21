use std::io::{self, Read};

fn calculate_cost(distance: i32) -> i32 {
    if distance < 2 {
        1
    } else {
        distance + calculate_cost(distance - 1)
    }
}
fn calculate(input: &str) -> i32 {
    let numbers: Vec<i32> = input
        .split(',')
        .map(|s| s.trim().parse().unwrap())
        .collect();
    let mut best_cost = None;

    for potential_position in *numbers.iter().min().unwrap()..*numbers.iter().max().unwrap() {
        let mut cost = 0;
        for number in &numbers {
            cost += calculate_cost((number - potential_position).abs());
        }
        println!("evaluated position {} at cost {}", potential_position, cost);
        if best_cost.is_none() || Some(cost) < best_cost {
            println!("NEW BEST");
            best_cost = Some(cost)
        }
    }
    println!("found best cost {:?}", best_cost);
    best_cost.unwrap()
}

fn main() {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    let score = calculate(&input);
    println!("result: {:?}", score);
}

#[cfg(test)]
mod tests {
    use super::calculate;

    #[test]
    fn test_calculate() {
        let count = calculate("16,1,2,0,4,2,7,1,2,14");
        assert_eq!(count, 168);
    }
}
