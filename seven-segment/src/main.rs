use std::io::{self, Read};

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
struct SegmentDisplay {
    a: bool,
    b: bool,
    c: bool,
    d: bool,
    e: bool,
    f: bool,
    g: bool,
}

impl SegmentDisplay {
    fn segment_count(&self) -> u32 {
        [self.a, self.b, self.c, self.d, self.e, self.f, self.g]
            .iter()
            .filter(|v| **v)
            .count() as u32
    }
}

impl From<&str> for SegmentDisplay {
    fn from(input: &str) -> Self {
        SegmentDisplay {
            a: input.contains('a'),
            b: input.contains('b'),
            c: input.contains('c'),
            d: input.contains('d'),
            e: input.contains('e'),
            f: input.contains('f'),
            g: input.contains('g'),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
struct SegmentLine {
    signal_patterns: Vec<SegmentDisplay>,
    output: Vec<SegmentDisplay>,
}

impl From<&str> for SegmentLine {
    fn from(input: &str) -> Self {
        let parts: Vec<&str> = input.split('|').collect();
        SegmentLine {
            signal_patterns: parts[0].split(' ').map(SegmentDisplay::from).collect(),
            output: parts[1].split(' ').map(SegmentDisplay::from).collect(),
        }
    }
}

fn calculate(input: &str) -> usize {
    let lines: Vec<SegmentLine> = input.trim().lines().map(SegmentLine::from).collect();
    let unique_digits: Vec<u32> = vec![2, 3, 4, 7, 8];
    let mut matches = 0;
    for line in lines {
        for output in line.output {
            if unique_digits.contains(&output.segment_count()) {
                matches += 1;
            }
        }
    }
    matches
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
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
",
        );
        assert_eq!(count, 26);
    }
}
