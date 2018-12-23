#![feature(uniform_paths)]
mod board;

use board::Board;
use std::env::args;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn get_input_data() -> Vec<String> {
    let file_path = args().nth(2).expect("Input file path is required");
    let file = File::open(file_path).expect("Could not open file");
    let reader = BufReader::new(file);

    return reader
        .lines()
        .map(|line| line.expect("Could not read line"))
        .collect();
}

fn exercise1() {
    let mut board = Board::from(get_input_data());
    board.mark_with_closest();
    let areas = board.get_point_areas();

    let max_non_infinate_area = areas.iter().map(|(_, area)| area).max();
    println!("Max area is: {}", max_non_infinate_area.unwrap());
}

fn exercise2() {
    let board = Board::from(get_input_data());
    let area_closest_to_all = board.area_closest_to_all();

    println!("Max area is: {}", area_closest_to_all.len());
}

fn main() {
    let exercise = args().nth(1).expect("Need exercise type");
    if exercise == "e1" {
        exercise1();
    } else if exercise == "e2" {
        exercise2();
    }
}
