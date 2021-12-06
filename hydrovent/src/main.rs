use std::cmp::{max, min};
use std::io::{self, Read};
use std::iter::Iterator;
use std::collections::HashMap;

#[derive(Default, Debug, PartialEq, Clone, Copy, Hash, Eq)]
struct Coordinate {
    x: u32,
    y: u32,
}

impl From<&str> for Coordinate {
    // 5,5
    fn from(input: &str) -> Self {
        let elements: Vec<u32> = input
            .split(',')
            .map(|coord| coord.trim().parse().unwrap())
            .collect();
        Coordinate {
            x: elements[0],
            y: elements[1],
        }
    }
}
impl PartialEq<(u32, u32)> for Coordinate {
    fn eq(&self, other: &(u32, u32)) -> bool {
        (self.x, self.y) == *other
    }
}

#[derive(Default, Debug, PartialEq)]
struct VentLine {
    coord1: Coordinate,
    coord2: Coordinate,
}

impl From<&str> for VentLine {
    // 5,5 -> 8,2
    fn from(input: &str) -> Self {
        let coordinates: Vec<Coordinate> = input.trim().split("->").map(Coordinate::from).collect();
        VentLine {
            coord1: coordinates[0],
            coord2: coordinates[1],
        }
    }
}

impl VentLine {
    fn coords(&self) -> Vec<Coordinate> {
        if self.coord1.y == self.coord2.y {
            let y = self.coord1.y;
            return (min(self.coord1.x, self.coord2.x)..max(self.coord1.x, self.coord2.x) + 1)
                .map(|x| Coordinate { x, y })
                .collect();
        }
        if self.coord1.x == self.coord2.x {
            let x = self.coord1.x;
            return (min(self.coord1.y, self.coord2.y)..max(self.coord1.y, self.coord2.y) + 1)
                .map(|y| Coordinate { x, y })
                .collect();
        }

        println!("Ignoring {:?} - not a straight line", self);
        vec![]
    }
}

fn calculate(input: &str) -> usize {
    let mut coordinate_counts: HashMap<Coordinate, usize> = HashMap::new();
    for coordinate in input.lines().map(VentLine::from).flat_map(|l| l.coords()) {
        *coordinate_counts.entry(coordinate).or_default() += 1;
    }
    let multiples = coordinate_counts.iter().filter(|&(_k, v)| *v > 1).count();
    println!("multiples: {:?}", multiples);
    multiples
}

fn main() {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    let score = calculate(&input);
    println!("result regular: {:?}", score);
}

#[cfg(test)]
mod tests {
    use super::{calculate, VentLine};

    #[test]
    fn test_ventline() {
        let content = "5,5 -> 8,2 ";
        let vent_line = VentLine::from(content);
        assert_eq!(vent_line.coord1.x, 5);
        assert_eq!(vent_line.coord1.y, 5);
        assert_eq!(vent_line.coord2.x, 8);
        assert_eq!(vent_line.coord2.y, 2);
        assert_eq!(vent_line.coords().len(), 0);
        assert_eq!(
            VentLine::from("5,5 -> 7,5").coords(),
            [(5, 5), (6, 5), (7, 5)]
        );
        assert_eq!(
            VentLine::from("5,7 -> 5,5").coords(),
            [(5, 5), (5, 6), (5, 7)]
        );
    }

    #[test]
    fn test_calculate() {
        let input = String::from(
            "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2",
        );
        let count = calculate(&input);
        assert_eq!(count, 5);
    }
}
