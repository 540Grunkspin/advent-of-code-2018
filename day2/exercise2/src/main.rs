use std::env::args;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::option::Option;

fn get_input_data() -> Vec<String> {
    let file_path = args().nth(1).expect("Input file path is required");
    let file = File::open(file_path).expect("Could not open file");
    let reader = BufReader::new(file);

    return reader
        .lines()
        .map(|line| line.expect("Could not read line"))
        .collect();
}

fn get_matches(current_id: &String, all_ids: &Vec<String>) -> Option<String> {
    for id in all_ids {
        let mut character_pairs = current_id.chars().zip(id.chars());
        let same: String = character_pairs
            .filter(|(x, y)| x == y)
            .map(|(x, _)| x)
            .collect();

        if current_id.len() - same.len() == 1 {
            return Some(same);
        }
    }

    return None;
}

fn find_first_match(sets: &mut Vec<String>) -> Option<String> {
    loop {
        let found_match = match sets.pop() {
            Some(set) => get_matches(&set, sets),
            None => break,
        };

        if found_match.is_some() {
            return found_match;
        }
    }

    return None;
}

fn main() {
    let mut ids = get_input_data();
    let matched_chars = find_first_match(&mut ids);
    println!("{}", matched_chars.expect("No matches found"));
}
