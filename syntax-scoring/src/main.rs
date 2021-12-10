use std::io::{self, Read};

fn process_line(line: &str) -> Option<char> {
    let mut symbol_stack: Vec<char> = vec![];

    for ch in line.chars() {
        if ['<', '{', '(', '['].contains(&ch) {
            symbol_stack.push(ch);
        } else {
            let top = symbol_stack.pop();
            let valid = match top {
                Some('<') => ch == '>',
                Some('{') => ch == '}',
                Some('(') => ch == ')',
                Some('[') => ch == ']',
                _ => false,
            };
            if !valid {
                return Some(ch);
            }
        }
    }
    None
}

fn calculate(input: &str) -> usize {
    let mismatches: Vec<Option<char>> = input.trim().lines().map(process_line).collect();
    println!("mismatches: {:?}", mismatches);
    let scores: Vec<usize> = mismatches
        .iter()
        .flatten()
        .map(|ch| match ch {
            '>' => 25137,
            '}' => 1197,
            ')' => 3,
            ']' => 57,
            _ => panic!("aaaaaah"),
        })
        .collect();
    scores.iter().sum()
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
        assert_eq!(calculate("<[()]>"), 0);
        assert_eq!(calculate(">"), 25137);
        assert_eq!(calculate("[)"), 3);
        assert_eq!(calculate("{{[)}}"), 3);

        let count = calculate(
            "
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]",
        );
        assert_eq!(count, 26397);
    }
}
