use std::io::{self, Read};

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
struct HeightPoint {
    height: u32,
    left: Option<u32>,
    right: Option<u32>,
    above: Option<u32>,
    below: Option<u32>,
}

impl HeightPoint {
    fn is_low(&self) -> bool {
        [self.left, self.right, self.above, self.below]
            .into_iter()
            .flatten()
            .all(|value| value > self.height)
    }
}
#[derive(Debug, PartialEq, Clone, Hash, Eq)]
struct HeightMap {
    points: Vec<Vec<u32>>,
}

impl From<&str> for HeightMap {
    fn from(input: &str) -> Self {
        HeightMap {
            points: input
                .trim()
                .lines()
                .map(|line| {
                    line.trim()
                        .chars()
                        .map(|c| c.to_digit(10).unwrap())
                        .collect()
                })
                .collect(),
        }
    }
}

impl HeightMap {
    fn width(&self) -> usize {
        self.points[0].len()
    }
    fn height(&self) -> usize {
        self.points.len()
    }
    fn height_point(&self, x: usize, y: usize) -> HeightPoint {
        let left = if x >= 1 {
            self.get_height(x - 1, y)
        } else {
            None
        };
        let above = if y >= 1 {
            self.get_height(x, y - 1)
        } else {
            None
        };
        HeightPoint {
            height: self.get_height(x, y).unwrap(),
            right: self.get_height(x + 1, y),
            below: self.get_height(x, y + 1),
            left,
            above,
        }
    }
    fn get_height(&self, x: usize, y: usize) -> Option<u32> {
        if let Some(row) = self.points.get(y) {
            if let Some(point) = row.get(x) {
                return Some(*point);
            }
        }
        None
    }
}

fn calculate(input: &str) -> u32 {
    let heightmap = HeightMap::from(input);
    let mut count = 0;
    for y in 0..heightmap.height() {
        for x in 0..heightmap.width() {
            let point = heightmap.height_point(x, y);
            if point.is_low() {
                count += 1 + point.height;
            }
        }
    }
    count
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
            "2199943210
3987894921
9856789892
8767896789
9899965678",
        );
        assert_eq!(count, 15);
    }
}
