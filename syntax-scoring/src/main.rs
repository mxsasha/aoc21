use std::io::{self, Read};

enum Char {
    Opening(CharType),
    Closing(CharType),
}

#[derive(Debug, PartialEq)]
enum CharType {
    AngleBracket,
    Parenthesis,
    CurlyBrace,
    SquareBracket,
}

impl TryFrom<char> for Char {
    type Error = ();

    fn try_from(value: char) -> Result<Char, Self::Error> {
        match value {
            '<' => Ok(Char::Opening(CharType::AngleBracket)),
            '>' => Ok(Char::Closing(CharType::AngleBracket)),
            '(' => Ok(Char::Opening(CharType::Parenthesis)),
            ')' => Ok(Char::Closing(CharType::Parenthesis)),
            '{' => Ok(Char::Opening(CharType::CurlyBrace)),
            '}' => Ok(Char::Closing(CharType::CurlyBrace)),
            '[' => Ok(Char::Opening(CharType::SquareBracket)),
            ']' => Ok(Char::Closing(CharType::SquareBracket)),
            _ => Err(()),
        }
    }
}

impl CharType {
    fn incomplete_score(&self) -> usize {
        match self {
            CharType::AngleBracket => 4,
            CharType::Parenthesis => 1,
            CharType::CurlyBrace => 3,
            CharType::SquareBracket => 2,
        }
    }
}

fn process_line(line: &str) -> Option<Vec<CharType>> {
    let mut symbol_stack: Vec<CharType> = vec![];
    let chars: Vec<Char> = line.chars().map(|c| Char::try_from(c).unwrap()).collect();
    for ch in chars {
        match ch {
            Char::Opening(char_type) => symbol_stack.push(char_type),
            Char::Closing(char_type) => {
                if symbol_stack.pop().as_ref() != Some(&char_type) {
                    return None;
                }
            }
        };
    }
    symbol_stack.reverse();
    Some(symbol_stack)
}

fn calculate(input: &str) -> usize {
    let mut scores: Vec<usize> = input
        .trim()
        .lines()
        .flat_map(process_line)
        .map(|chars| {
            chars
                .iter()
                .fold(0, |acc, char| (acc * 5) + char.incomplete_score())
        })
        .collect();
    scores.sort_unstable();
    scores[scores.len() / 2]
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
        assert_eq!(count, 288957);
    }
}
