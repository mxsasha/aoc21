use std::cell::RefCell;
use std::io::{self, Read};

#[derive(Debug)]
struct Octopus<'a> {
    has_flashed: bool,
    energy: usize,
    neighbors: Vec<&'a RefCell<Octopus<'a>>>,
}

impl<'a> From<char> for Octopus<'a> {
    fn from(input: char) -> Self {
        Octopus {
            has_flashed: false,
            energy: input.to_digit(10).unwrap() as usize,
            neighbors: vec![],
        }
    }
}

impl<'a> Octopus<'a> {
    fn prepare_round(&mut self) {
        self.energy += 1;
        self.has_flashed = false;
    }

    fn update_state(&mut self) {
        if self.energy <= 9 {
            return;
        }
        self.has_flashed = true;
        self.energy = 0;
        self.neighbors.iter().for_each(|octopus| {
            if let Ok(mut n) = octopus.try_borrow_mut() {
                n.neighbor_flashed()
            }
        });
    }

    fn neighbor_flashed(&mut self) {
        if self.has_flashed {
            return;
        }
        self.energy += 1;
        self.update_state();
    }
}

fn calculate(input: &str) -> usize {
    let grid: Vec<Vec<RefCell<Octopus>>> = input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| RefCell::new(Octopus::from(c)))
                .collect()
        })
        .collect();
    let x_max = grid.len() - 1;
    let y_max = grid[0].len() - 1;
    for x in 0..=x_max {
        for y in 0..=y_max {
            let neighbors: Vec<&RefCell<Octopus>> = neighbor_coordinates(x, y, x_max, y_max)
                .into_iter()
                .map(|(x, y)| &grid[x][y])
                .collect();
            grid[x][y].borrow_mut().neighbors = neighbors;
        }
    }

    let mut loop_count = 0;
    let mut flash_count = 0;
    while flash_count < grid.len() * grid[0].len() {
        grid.iter()
            .flatten()
            .for_each(|octopus| octopus.borrow_mut().prepare_round());
        grid.iter()
            .flatten()
            .for_each(|octopus| octopus.borrow_mut().update_state());
        flash_count = grid
            .iter()
            .flatten()
            .filter(|octopus| octopus.borrow().has_flashed)
            .count();
        loop_count += 1;
        println!("loop {} found {} flashes", loop_count, flash_count);
    }
    loop_count
}

// TODO: return iterator
fn neighbor_coordinates(x: usize, y: usize, x_max: usize, y_max: usize) -> Vec<(usize, usize)> {
    let mut neighbors = vec![];
    let (x, y) = (x as isize, y as isize);

    for x_offset in -1..=1 {
        for y_offset in -1..=1 {
            let (neighbor_x, neighbor_y) = (x + x_offset, y + y_offset);
            if neighbor_x < 0
                || neighbor_x > x_max as isize
                || neighbor_y < 0
                || neighbor_y > y_max as isize
                || x_offset == 0 && y_offset == 0
            {
                continue;
            }
            neighbors.push((neighbor_x as usize, neighbor_y as usize));
        }
    }
    neighbors
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
            "
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526",
        );
        assert_eq!(count, 195);
    }
}
