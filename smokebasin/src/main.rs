use std::io::{self, Read};
use std::collections::HashMap;


#[derive(Debug, PartialEq, Clone, Hash, Eq)]
struct HeightMap {
    points: Vec<Vec<u32>>,
    basin_assignments: Vec<Vec<u32>>,
}

impl From<&str> for HeightMap {
    fn from(input: &str) -> Self {
        let points: Vec<Vec<u32>> = input
            .trim()
            .lines()
            .map(|line| {
                line.trim()
                    .chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect()
            })
            .collect();
        let (len_x, len_y) = (points[0].len(), points.len());
        HeightMap {
            points,
            basin_assignments: vec![vec![0; len_x]; len_y],
        }
    }
}

impl HeightMap {
    fn size(&self) -> (usize, usize) {
        (self.points[0].len(), self.points.len())
    }

    fn is_low(&self, x: usize, y: usize) -> bool {
        let mut is_low = true;
        for (neigh_x, neigh_y) in self.get_neighbors(x, y) {
            if self.points[neigh_y][neigh_x] < self.points[y][x] {
                is_low = false;
            }
        }
        is_low
    }

    fn fill_basin(&mut self, x: usize, y: usize, basin_id: u32) {
        if self.get(x, y) == 9 || self.basin_assignments[y][x] != 0 {
            return;
        }
        self.basin_assignments[y][x] = basin_id;
        for (neigh_x, neigh_y) in self.get_neighbors(x, y) {
            self.fill_basin(neigh_x, neigh_y, basin_id);
        }
    }

    fn get_neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut neighbors = vec![];
        let (len_x, len_y) = self.size();
        if x > 0 {
            neighbors.push((x - 1, y));
        }
        if y > 0 {
            neighbors.push((x, y - 1));
        }
        if x < len_x - 1 {
            neighbors.push((x + 1, y));
        }
        if y < len_y - 1 {
            neighbors.push((x, y + 1));
        }
        neighbors
    }

    fn get(&self, x: usize, y: usize) -> u32 {
        self.points[y][x]
    }

    fn basin_sizes(&self) -> HashMap<u32, u32> {
        let mut map = HashMap::new();
        let basin_ids: Vec<u32> = self.basin_assignments.iter().flatten().copied().filter(|id| *id != 0).collect();
        for basin_id in basin_ids {
            *map.entry(basin_id).or_default() += 1;
         }
         map
    }
}

fn calculate(input: &str) -> u32 {
    let mut heightmap = HeightMap::from(input);
    let mut low_point_score = 0;
    let (len_x, len_y) = heightmap.size();

    let mut basin_id = 1;
    for x in 0..len_x {
        for y in 0..len_y {
            if heightmap.is_low(x, y) {
                heightmap.fill_basin(x, y, basin_id);
                basin_id += 1;
                low_point_score += heightmap.get(x, y) + 1;
            }
        }
    }
    let mut basin_sizes: Vec<u32> = heightmap.basin_sizes().values().copied().collect();
    basin_sizes.sort();
    basin_sizes.reverse();
    basin_sizes.iter().take(3).fold(1, |acc, x| acc * x)
    // low_point_score
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
        assert_eq!(count, 1134);
    }
}
