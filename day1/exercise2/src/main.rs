use std::env::args;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::iter::Iterator;
use std::vec::Vec;
use std::collections::HashSet;

fn find_reccuring_frequencies(numbers: &Vec<i32>) -> i32 {
    let mut frequencies: HashSet<i32> = HashSet::new();
    let mut current_frequence: i32 = 0;
    loop {
        for number in numbers.iter() {
            current_frequence += number;
            if !frequencies.insert(current_frequence) {
                return current_frequence
            }
        }
    }
}

fn main() {
     let file_path = args().nth(1).expect("You need to provide a path to the file");
     let file = File::open(file_path).expect("Could not open file");
     let reader = BufReader::new(file);

     let numbers: Vec<i32> = reader.lines().map(|line| line.expect("Could not parse number").parse::<i32>().unwrap()).collect();

     println!("{}", find_reccuring_frequencies(&numbers));
}