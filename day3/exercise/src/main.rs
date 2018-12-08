extern crate regex;
#[macro_use] extern crate lazy_static;

use regex::Regex;
use std::env::args;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::string::String;
use std::collections::HashSet;
use std::vec::Vec;
use std::iter::FromIterator;

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
    left: usize,
    top: usize,
    width: usize,
    height: usize,
}

struct FabricSheet {
    sheet: Vec<Vec<i32>>,
    all_claims: HashSet<i32>,
    invalid_claims: HashSet<i32>,
    claimed_twice: HashSet<(usize, usize)>
}

impl FabricSheet {
    fn new() -> FabricSheet {
        return FabricSheet { 
            sheet: vec![vec![0 ; 1000] ; 1000],
            claimed_twice: HashSet::new(),
            all_claims: HashSet::new(),
            invalid_claims: HashSet::new(),
        }
    }

    fn add_claim(&mut self, claim: &Claim) {
        for y in 0..claim.height {
            for x in 0..claim.width {
                let y_position = y + claim.top;
                let x_position = x + claim.left;
                let mut sheet_square: &mut i32 = &mut self.sheet[y_position][x_position]; 
                if *sheet_square == 0 || *sheet_square == claim.id {
                    *sheet_square = claim.id;
                    self.all_claims.insert(claim.id);
                } else {
                    self.invalid_claims.insert(claim.id);
                    self.invalid_claims.insert(*sheet_square);
                    self.claimed_twice.insert((x_position, y_position));
                    *sheet_square = -1;
                }
            }
        }
    }

    fn get_valid_claims(&self) -> HashSet<&i32> {
        return HashSet::from_iter(self.all_claims.difference(&self.invalid_claims).into_iter());
    }

    fn nsquares_claimed_twice(&self) -> usize {
        return self.claimed_twice.len();
    }
}

impl From<String> for Claim {
    fn from(input: String) -> Claim {
        lazy_static! {
            static ref pattern: Regex = Regex::new(r"#(\d*?)\s+@\s+(\d*),(\d*):\s+(\d*)x(\d*)").unwrap();
        }
        let matches = pattern.captures_iter(&input).next().unwrap();

        let id = &matches[1].parse::<i32>().unwrap();
        let left = &matches[2].parse::<usize>().unwrap();
        let top = &matches[3].parse::<usize>().unwrap();
        let width = &matches[4].parse::<usize>().unwrap();
        let height = &matches[5].parse::<usize>().unwrap();

        return Claim {
            id: id.clone(),
            left: left.clone(),
            top: top.clone(),
            width: width.clone(),
            height: height.clone(),
        };
    }
}

fn main() {
    let mut fabric_sheet = FabricSheet::new();
    let claim_descriptions = get_input_data();

    let claims: Vec<Claim> = claim_descriptions.into_iter()
        .map(|description| Claim::from(description))
        .collect();

    for claim in &claims {
        fabric_sheet.add_claim(claim);
    }

    println!("Square meter claimed twice: {}", fabric_sheet.nsquares_claimed_twice());
    println!("Valid claim: {}", fabric_sheet.get_valid_claims().iter().next().unwrap());
}

#[cfg(test)]
mod test {
    use Claim;
    use FabricSheet;

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

    #[test]
    fn test_add_claim() {
        let mut sheet = FabricSheet::new();
        let claim = Claim { id: 1, left: 0, top: 0, width: 1, height: 1 };

        sheet.add_claim(&claim);

        assert_eq!(sheet.sheet[0][0], 1);
    }

    #[test]
    fn test_add_twice() {
        let mut sheet = FabricSheet::new();
        let first_claim = Claim { id: 1, left: 0, top: 0, width: 1, height: 1 };
        let second_claim = Claim { id: 2, left: 0, top: 0, width: 1, height: 1 };

        sheet.add_claim(&first_claim);
        sheet.add_claim(&second_claim);

        assert_eq!(*sheet.claimed_twice.iter().next().unwrap(), (0, 0));
        assert_eq!(sheet.get_valid_claims().len(), 0)
    }

    #[test]
    fn add_same_twice() {
        let mut sheet = FabricSheet::new();
        let claim = Claim { id: 1, left: 0, top: 0, width: 1, height: 1 };

        sheet.add_claim(&claim);
        sheet.add_claim(&claim);

        assert_eq!(sheet.nsquares_claimed_twice(), 0);
        assert_eq!(sheet.get_valid_claims().len(), 1)
    }
}
