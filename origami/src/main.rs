use std::{io::{self, Read}, fmt};

#[derive(Debug)]
enum Fold {
    X(usize),
    Y(usize),
}

impl Fold {
    fn new(fold: &str) -> Self {
        let mut parts = fold[11..].trim().split('=');
        let direction = parts.next().unwrap();
        let size: usize = parts.next().unwrap().parse().unwrap();
        match direction {
            "x" => Fold::X(size),
            "y" => Fold::Y(size),
            _ => panic!("Invalid fold direction"),
        }
    }
}

struct Paper {
    points: Vec<Vec<bool>>,
    x_size: usize,
    y_size: usize,
}

impl fmt::Display for Paper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut status: Vec<Vec<String>> = vec![vec![String::from("."); self.x_size]; self.y_size];
        for (x, column) in self.points.iter().enumerate() {
            for (y, entry) in column.iter().enumerate() {
                if *entry {
                    status[y][x] = String::from("#");
                }
            }
        }
        let row_strings: Vec<String> = status.iter().map(|column| column.join("")).collect();
        let output: String = row_strings.join("\n");
        write!(f, "{}", output)
    }
}

impl Paper {
    fn new(points: &str) -> Self {
        let points_tuples: Vec<(usize, usize)> = points
            .trim()
            .lines()
            .map(|line| {
                let parts: Vec<usize> = line
                    .trim()
                    .split(',')
                    .map(|coord| coord.parse().unwrap())
                    .collect();
                (parts[0], parts[1])
            })
            .collect();
        let x_size = points_tuples.iter().map(|(x, _y)| x).max().unwrap() + 1;
        let y_size = points_tuples.iter().map(|(_x, y)| y).max().unwrap() + 1;
        let mut points_vec = vec![vec![false; y_size]; x_size];
        for (x, y) in points_tuples {
            points_vec[x][y] = true;
        }
        Paper {
            points: points_vec,
            x_size,
            y_size,
        }
    }

    fn count_points(&self) -> usize {
        self.points
            .iter()
            .map(|column| column.iter().filter(|point| **point).count())
            .sum()
    }

    fn fold(&mut self, fold: &Fold) {
        if let Fold::Y(fold_size) = fold {
            let y_size1 = *fold_size;
            let y_size2 = self.y_size - fold_size - 1;
            let y_size_new = y_size1.max(y_size2);
            let y_offset1 = y_size_new - y_size1;
            let y_offset2 = y_size_new - y_size2;
            let mut points_vec = vec![vec![false; y_size_new]; self.x_size];
            for (x, column) in self.points.iter().enumerate() {
                let (column1, column2) = column.split_at(*fold_size + 1);
                for (y, value) in column1.iter().take(y_size1).enumerate() {
                    points_vec[x][y + y_offset1] = *value;
                }
                for (y, value) in column2.iter().rev().enumerate().filter(|(_y, value)| **value) {
                    points_vec[x][y + y_offset2] = *value;
                }
            }
            self.points = points_vec;
            self.y_size = y_size_new;
        }
    }
}
fn calculate(input: &str) -> usize {
    let mut parts = input.trim().split("\n\n");
    let (points_str, folds_str) = (parts.next().unwrap(), parts.next().unwrap());
    let mut paper = Paper::new(points_str);
    let folds: Vec<Fold> = folds_str.lines().map(Fold::new).collect();
    println!("Initial:\n===\n{}\n===", paper);
    paper.fold(&folds[0]);
    println!("After fold 0:\n===\n{}\n===", paper);

    paper.count_points()
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
6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5",
        );
        assert_eq!(count, 17);
    }
}
