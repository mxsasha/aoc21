use std::io::{self, Read};
use std::iter::Iterator;

#[derive(Default, Debug, PartialEq, Clone, Hash, Eq)]
struct School {
    fish_per_count: [u128; 9],
}

impl From<&str> for School {
    fn from(input: &str) -> Self {
        let mut school = School::zero();
        input
            .trim()
            .split(',')
            .map(|c| c.parse().unwrap())
            .for_each(|counter: usize| school.fish_per_count[counter] += 1);
        school
    }
}

impl School {
    fn zero() -> Self {
        School {
            fish_per_count: [0; 9],
        }
    }
    fn advance_day(&mut self) {
        let spawning_fish = self.fish_per_count[0];
        for idx in 1..=8 {
            self.fish_per_count[idx - 1] = self.fish_per_count[idx];
        }
        self.fish_per_count[6] += spawning_fish;
        self.fish_per_count[8] = spawning_fish;
    }
    fn size(&self) -> u128 {
        self.fish_per_count.iter().sum()
    }
}

fn calculate(input: &str, days: usize) -> u128 {
    let mut school = School::from(input);
    for day in 0..days {
        println!("starting day {} with {} fish", day, school.size());
        school.advance_day();
    }
    school.size()
}

fn main() {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    let score = calculate(&input, 256);
    println!("result: {:?}", score);
}

#[cfg(test)]
mod tests {
    use super::calculate;

    #[test]
    fn test_calculate() {
        let count = calculate("3,4,3,1,2", 80);
        assert_eq!(count, 5934);
    }
}
