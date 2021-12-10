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
    fn mismatch_score(&self) -> usize {
        match self {
            CharType::AngleBracket => 25137,
            CharType::Parenthesis => 3,
            CharType::CurlyBrace => 1197,
            CharType::SquareBracket => 57,
        }
    }
}

fn process_line(line: &str) -> Option<CharType> {
    let mut symbol_stack: Vec<CharType> = vec![];
    let chars: Vec<Char> = line.chars().map(|c| Char::try_from(c).unwrap()).collect();
    for ch in chars {
        match ch {
            Char::Opening(char_type) => symbol_stack.push(char_type),
            Char::Closing(char_type) => {
                if symbol_stack.pop().as_ref() != Some(&char_type) {
                    return Some(char_type);
                }
            }
        };
    }
    None
}

fn calculate(input: &str) -> usize {
    let mismatches: Vec<Option<CharType>> = input.trim().lines().map(process_line).collect();
    println!("mismatches: {:?}", mismatches);
    mismatches
        .iter()
        .flatten()
        .map(|ct| ct.mismatch_score())
        .sum()
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
