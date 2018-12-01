use std::env::args;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::iter::Iterator;
use std::vec::Vec;

fn last_or_zero(vec: &Vec<i32>) -> i32 {
    return vec.iter().last().unwrap_or(&0).clone();
}

fn main() {
     let file_path = args().nth(1).expect("You need to provide a path to the file");
     let file = File::open(file_path).expect("Could not open file");
     let reader = BufReader::new(file);

     let numbers: Vec<i32> = reader.lines().map(|line| line.expect("Could not parse number").parse::<i32>().unwrap()).collect();

    let mut freq_at_stage: Vec<i32> = Vec::new();
    let mut result: Option<i32> = None;
    while result.is_none() {
        for number in numbers.iter() {
            let last_freq: i32 = last_or_zero(&freq_at_stage);
            let next_freq = last_freq + number;
            let does_contain = freq_at_stage.iter().position(|&i| i == next_freq);
            if does_contain.is_some() {
                result = Some(next_freq);
                break;
            } else {
                freq_at_stage.push(next_freq);
            }
        }
    }

     println!("{}", result.expect("Could not find any repetitions"));
}