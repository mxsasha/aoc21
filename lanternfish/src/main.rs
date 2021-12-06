use std::io::{self, Read};
use std::iter::Iterator;

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
struct Fish {
    timer: usize,
}

impl From<&str> for Fish {
    fn from(input: &str) -> Self {
        Fish {
            timer: input.trim().parse().unwrap(),
        }
    }
}
impl Default for Fish {
    fn default() -> Self {
        Fish { timer: 8 }
    }
}

#[derive(Default, Debug, PartialEq, Clone, Hash, Eq)]
struct School {
    fishes: Vec<Fish>,
}

impl Fish {
    fn advance_day_check_spawn(&mut self) -> bool {
        if self.timer == 0 {
            self.timer = 6;
            return true;
        }
        self.timer -= 1;
        false
    }
}

impl From<&str> for School {
    fn from(input: &str) -> Self {
        let fishes: Vec<Fish> = input.trim().split(',').map(Fish::from).collect();
        School { fishes }
    }
}

impl School {
    fn size(&self) -> usize {
        self.fishes.len()
    }
    fn _timers(&self) -> Vec<usize> {
        self.fishes.iter().map(|f| f.timer).collect()
    }
    fn advance_day(&mut self) {
        let mut new_fishes: Vec<Fish> = vec![];
        for fish in &mut self.fishes {
            if fish.advance_day_check_spawn() {
                new_fishes.push(Fish::default());
            }
        }
        self.fishes.append(&mut new_fishes);
    }
}

fn calculate(input: &str, days: usize) -> usize {
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
    use super::{calculate, School};

    #[test]
    fn test_school() {
        let content = "3,4,3,1,2";
        let mut school = School::from(content);
        assert_eq!(school.size(), 5);
        assert_eq!(school._timers(), [3, 4, 3, 1, 2]);
        school.advance_day();
        assert_eq!(school.size(), 5);
        assert_eq!(school._timers(), [2, 3, 2, 0, 1]);
        school.advance_day();
        assert_eq!(school.size(), 6);
        assert_eq!(school._timers(), [1, 2, 1, 6, 0, 8]);
    }

    #[test]
    fn test_calculate() {
        let count = calculate("3,4,3,1,2", 80);
        assert_eq!(count, 5934);
    }
}
