extern crate regex;

use regex::Regex;
use std::env::args;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::string::String;

fn get_input_data() -> Vec<String> {
    let file_path = args().nth(1).expect("Input file path is required");
    let file = File::open(file_path).expect("Could not open file");
    let reader = BufReader::new(file);

    return reader
        .lines()
        .map(|line| line.expect("Could not read line"))
        .collect();
}

#[derive(PartialEq, Debug)]
struct Claim {
    id: i32,
    left: i32,
    top: i32,
    width: i32,
    height: i32,
}

impl From<String> for Claim {
    fn from(input: String) -> Claim {
        let pattern = Regex::new(r"#(\d{3})\s+@\s+(\d),(\d):\s+(\d)x(\d)").unwrap();
        let matches = pattern.captures_iter(&input).next().unwrap();

        let id = &matches[1].parse::<i32>().unwrap();
        let left = &matches[2].parse::<i32>().unwrap();
        let top = &matches[3].parse::<i32>().unwrap();
        let width = &matches[4].parse::<i32>().unwrap();
        let height = &matches[5].parse::<i32>().unwrap();

        return Claim {
            id: id.clone(),
            left: left.clone(),
            top: top.clone(),
            width: width.clone(),
            height: height.clone(),
        };
    }
}

fn main() {}

#[cfg(test)]
mod test {
    use Claim;

    #[test]
    fn test_claim_from_string() {
        let parsed = Claim::from("#123 @ 3,2: 5x4".to_string());
        let expected = Claim {
            id: 123,
            left: 3,
            top: 2,
            width: 5,
            height: 4,
        };
        assert_eq!(expected, parsed);
    }
}
