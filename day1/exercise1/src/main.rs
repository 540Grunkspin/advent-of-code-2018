use std::env::args;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::iter::Iterator;

fn main() {
     let file_path = args().nth(1).expect("You need to provide a path to the file");
     let file = File::open(file_path).expect("Could not open file");
     let reader = BufReader::new(file);

     let numbers: Vec<i32> = reader.lines().map(|line| line.expect("Could not parse number").parse::<i32>().unwrap()).collect();

     let result: i32 = numbers.into_iter().sum();
     println!("{}", result);
}
