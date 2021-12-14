use std::{
    collections::HashMap,
    io::{self, Read},
};

struct Polymer {
    pair_counts: HashMap<(char, char), usize>,
    rules: HashMap<(char, char), char>,
}

impl Polymer {
    fn new(input: &str) -> Self {
        let parts: Vec<&str> = input.trim().split("\n\n").collect();

        let mut pair_counts = HashMap::new();
        let initial: Vec<char> = parts[0].chars().collect();
        for idx_first in 0..initial.len() - 1 {
            let pair = (initial[idx_first], initial[idx_first + 1]);
            *pair_counts.entry(pair).or_default() += 1
        }

        let mut rules = HashMap::new();
        parts[1].lines().for_each(|rule| {
            let chars: Vec<char> = rule.chars().collect();
            rules.insert((chars[0], chars[1]), chars[6]);
        });

        Polymer { pair_counts, rules }
    }
    fn step(&mut self) {
        let mut new_pair_counts = HashMap::new();

        for (&existing_pair, &count) in self.pair_counts.iter() {
            let new_pairs = match self.rules.get(&existing_pair) {
                Some(insertion) => {
                    vec![(existing_pair.0, *insertion), (*insertion, existing_pair.1)]
                }
                None => vec![(existing_pair.0, existing_pair.1)],
            };
            for pair in new_pairs {
                *new_pair_counts.entry(pair).or_default() += count;
            }
        }
        self.pair_counts = new_pair_counts;
    }
    fn char_counts(&self) -> Vec<(char, usize)> {
        let mut char_counts = HashMap::new();
        for (idx, (&pair, &count)) in self.pair_counts.iter().enumerate() {
            if idx == 0 {
                *char_counts.entry(pair.0).or_default() += count;
            }
            *char_counts.entry(pair.1).or_default() += count;
        }
        let mut counts_vec: Vec<(char, usize)> = char_counts.into_iter().collect();
        counts_vec.sort_by(|a, b| b.1.cmp(&a.1));
        counts_vec
    }
}

fn calculate(input: &str) -> usize {
    let mut polymer = Polymer::new(input);
    for _step in 0..10 {
        polymer.step();
    }
    let char_counts = polymer.char_counts();
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
