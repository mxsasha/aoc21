use std::io::{self, Read};
use std::iter::Iterator;

#[derive(Default, Debug, PartialEq)]
struct Board {
    content: Vec<u32>,
}

impl From<&str> for Board {
    fn from(input: &str) -> Self {
        let content: Vec<u32> = input
            .trim()
            .replace("\n", " ")
            .split(' ')
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .map(|x| x.parse().unwrap())
            .collect();
        if content.len() != 25 {
            panic!(
                "Failed to parse board, ended up with {} items: {:?}",
                content.len(),
                content
            );
        }
        Board { content }
    }
}

impl Board {
    fn has_hit(&self, draws: &[u32]) -> bool {
        for row in self.content.chunks(5) {
            if row.iter().all(|num| draws.contains(num)) {
                return true;
            }
        }
        for column_idx in 0..5 {
            let column: Vec<&u32> = self.content.iter().skip(column_idx).step_by(5).collect();
            if column.iter().all(|num| draws.contains(num)) {
                return true;
            }
        }
        false
    }
    fn score(&self, draws: &[u32]) -> Option<u32> {
        if !self.has_hit(draws) {
            None
        } else {
            Some(self.content.iter().filter(|num| !draws.contains(num)).sum())
        }
    }
}

fn bingo(input: &str) -> Option<u32> {
    let blocks: Vec<&str> = input.split("\n\n").collect();
    let draws: Vec<u32> = blocks[0].split(',').map(|x| x.parse().unwrap()).collect();
    let boards: Vec<Board> = blocks[1..].iter().map(|x| Board::from(*x)).collect();

    let mut all_draws: Vec<u32> = vec![];
    for draw in draws {
        all_draws.push(draw);
        for board in &boards {
            if let Some(score) = board.score(&all_draws) {
                return Some(score * draw);
            }
        }
    }
    None
}

fn bingo_opposite(input: &str) -> Option<u32> {
    let blocks: Vec<&str> = input.split("\n\n").collect();
    let draws: Vec<u32> = blocks[0].split(',').map(|x| x.parse().unwrap()).collect();
    let boards: Vec<Board> = blocks[1..].iter().map(|x| Board::from(*x)).collect();
    
    let mut winning_scores: Vec<u32> = vec![];
    let mut previously_won_boards: Vec<&Board> = vec![];
    let mut all_draws: Vec<u32> = vec![];
    for draw in draws {
        all_draws.push(draw);
        for board in &boards {
            if let Some(score) = board.score(&all_draws) {
                if !previously_won_boards.contains(&board) {
                    winning_scores.push(score * draw);
                    previously_won_boards.push(board);
                }
            }
        }
    }
    winning_scores.last().copied()
}

fn main() {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    let score = bingo(&input);
    println!("score regular: {:?}", score);
    let score = bingo_opposite(&input);
    println!("score opposite: {:?}", score);
}

#[cfg(test)]
mod tests {
    use super::{Board, bingo, bingo_opposite};

    #[test]
    fn test_board() {
        let board_content = "
14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
";
        let board = Board::from(board_content);
        assert_eq!(
            board.content,
            vec![
                14, 21, 17, 24, 4, 10, 16, 15, 9, 19, 18, 8, 23, 26, 20, 22, 11, 13, 6, 5, 2, 0,
                12, 3, 7
            ]
        );
        assert_eq!(
            board.has_hit(&vec![7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21]),
            false
        );
        assert_eq!(board.has_hit(&vec![24, 9, 26, 6, 3]), true);
        assert_eq!(
            board.has_hit(&vec![7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24]),
            true
        );
        assert_eq!(
            board.has_hit(&vec![
                7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8,
                19, 3, 26, 1
            ]),
            true
        );
        assert!(board
            .score(&vec![7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21])
            .is_none());
        assert_eq!(board.score(&vec![24, 9, 26, 6, 3]).unwrap(), 257);
        assert_eq!(
            board
                .score(&vec![7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24])
                .unwrap(),
            188
        );
        assert_eq!(
            board
                .score(&vec![
                    7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18,
                    20, 8, 19, 3, 26, 1
                ])
                .unwrap(),
            0
        );
    }

    #[test]
    fn test_bingo() {
        let input = String::from(
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

    22 13 17 11  0
     8  2 23  4 24
    21  9 14 16  7
     6 10  3 18  5
     1 12 20 15 19

     3 15  0  2 22
     9 18 13 17  5
    19  8  7 25 23
    20 11 10 24  4
    14 21 16 12  6

    14 21 17 24  4
    10 16 15  9 19
    18  8 23 26 20
    22 11 13  6  5
     2  0 12  3  7",
        );
        let score = bingo(&input);
        assert_eq!(score.unwrap(), 4512);
    }

    #[test]
    fn test_bingo_opposite() {
        let input = String::from(
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

    22 13 17 11  0
     8  2 23  4 24
    21  9 14 16  7
     6 10  3 18  5
     1 12 20 15 19

     3 15  0  2 22
     9 18 13 17  5
    19  8  7 25 23
    20 11 10 24  4
    14 21 16 12  6

    14 21 17 24  4
    10 16 15  9 19
    18  8 23 26 20
    22 11 13  6  5
     2  0 12  3  7",
        );
        let score = bingo_opposite(&input);
        assert_eq!(score.unwrap(), 1924);
    }
}
