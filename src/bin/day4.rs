use std::{fs::read_to_string, str::FromStr, string::ParseError};

#[derive(Debug)]
struct Range {
    start: u32,
    end: u32,
}

impl FromStr for Range {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once('-').unwrap();
        Ok(Range {
            start: x.parse().unwrap(),
            end: y.parse().unwrap(),
        })
    }
}

fn main() {
    let input_text = read_to_string("src/input/day4_1.test").unwrap();
    for line in input_text.split('\n') {
        let tuple = line.split_once(',').unwrap();
        let range1: Range = tuple.0.parse().unwrap();
        let range2: Range = tuple.1.parse().unwrap();
        println!("{:?}", (range1, range2));
    }
}
