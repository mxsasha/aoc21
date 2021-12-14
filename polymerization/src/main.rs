use std::{
    collections::HashMap,
    fmt,
    io::{self, Read},
};
use counter::Counter;

struct Polymer {
    template: Vec<char>,
    rules: HashMap<(char, char), char>,
}

impl fmt::Display for Polymer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.template.iter().collect::<String>())
    }
}

impl Polymer {
    fn new(input: &str) -> Self {
        let parts: Vec<&str> = input.trim().split("\n\n").collect();
        let template = parts[0].chars().collect();

        let mut rules = HashMap::new();
        parts[1].lines().for_each(|rule| {
            let chars: Vec<char> = rule.chars().collect();
            rules.insert((chars[0], chars[1]), chars[6]);
        });

        Polymer { template, rules }
    }
    fn step(&mut self) {
        let mut new_template = vec![];
        for idx_first in 0..self.template.len()-1 {
            let pair = (self.template[idx_first], self.template[idx_first + 1]);
            let mut insertion = match self.rules.get(&pair) {
                Some(insertion) => vec![pair.0, *insertion, pair.1],
                None => vec![pair.0, pair.1],
            };
            if idx_first != 0 {
                insertion.remove(0);
            }
            new_template.extend(insertion);
        }
        self.template = new_template;
    }
}

fn calculate(input: &str) -> usize {
    let mut polymer = Polymer::new(input);
    for step in 0..10 {
        println!("polymer at step {}: {}", step, polymer);
        polymer.step();
    }
    let char_counts = polymer.to_string().chars().collect::<Counter<_>>().most_common_ordered();
    let most_common_count = char_counts.first().unwrap().1;
    let least_common_count = char_counts.last().unwrap().1;

    most_common_count - least_common_count
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
        let count = calculate(
            "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C",
        );
        assert_eq!(count, 1588);
    }
}
