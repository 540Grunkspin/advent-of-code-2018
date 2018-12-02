use std::env::args;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashSet;
use std::convert::From;
use std::option::Option;

fn get_input_data() -> Vec<String> {
    let file_path = args().nth(1).expect("Input file path is required");
    let file = File::open(file_path).expect("Could not open file");
    let reader = BufReader::new(file);

    return reader.lines().map(|line| line.expect("Could not read line")).collect();
}

fn get_matches(current_id: &String, all_ids: &Vec<String>) -> Option<String> {
    for id in all_ids {
        let mut diff_count = 0;
        let mut same = String::new();

        let mut current_iter = current_id.chars();
        let mut id_iter = id.chars();
        let mut current_next = current_iter.next();
        let mut id_next = id_iter.next();
        while current_next.is_some() && id_next.is_some() {
            if current_next.unwrap() != id_next.unwrap() { diff_count += 1 }
            else { same.push(current_next.unwrap().clone()) }

            current_next = current_iter.next();
            id_next = id_iter.next();
        }

        if diff_count <= 1 {
            return Some(same);
        }
    }

    return None;
}

fn find_first_match(sets: &mut Vec<String>) -> Option<String> {
    let mut current_set = sets.pop();
    while current_set.is_some() {
        let found_match = get_matches(&current_set.unwrap(), sets);
        if found_match.is_some() {
            return found_match;
        } else {
            current_set = sets.pop();
        }
    } 

    return None
}

fn main() {
    let mut ids = get_input_data();
    let matched_chars = find_first_match(&mut ids);
    println!("{}", matched_chars.expect("No matches found"));
}
