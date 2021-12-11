use std::io::{self, Read};

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
struct HeightMap {
    points: Vec<Vec<u32>>,
    basin_assignments: Vec<Vec<u32>>,
}

impl From<&str> for HeightMap {
    fn from(input: &str) -> Self {
        HeightMap {
            basin_assignments: vec![],
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
    fn size(&self) -> (usize, usize) {
        (self.points[0].len(), self.points.len())
    }
    fn is_low(&self, x: usize, y: usize) -> bool {
        let mut is_low = true;
        let (len_x, len_y) = self.size();
        if x > 0 && self.points[y][x - 1] < self.points[y][x] {
            is_low = false;
        }
        if y > 0 && self.points[y - 1][x] < self.points[y][x] {
            is_low = false;
        }
        if x < len_x - 1 && self.points[y][x + 1] < self.points[y][x] {
            is_low = false;
        }
        if y < len_y - 1 && self.points[y + 1][x] < self.points[y][x] {
            is_low = false;
        }

        is_low
    }
    fn get(&self, x: usize, y: usize) -> u32 {
        self.points[y][x]
    }
}

fn calculate(input: &str) -> u32 {
    let heightmap = HeightMap::from(input);
    let mut low_point_score = 0;
    let (len_x, len_y) = heightmap.size();

    for x in 0..len_x {
        for y in 0..len_y {
            if heightmap.is_low(x, y) {
                low_point_score += heightmap.get(x, y) + 1;
            }
        }
    }
    low_point_score
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
