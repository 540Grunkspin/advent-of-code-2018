#[macro_use]
extern crate lazy_static;

mod graph;

use std::env::args;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::iter::FromIterator;

use self::graph::step::Step;
use self::graph::Graph;
use self::graph::GraphIterator;

fn get_input_data() -> Vec<String> {
    let file_path = args().nth(1).expect("Input file path is required");
    let file = File::open(file_path).expect("Could not open file");
    let reader = BufReader::new(file);

    return reader
        .lines()
        .map(|line| line.expect("Could not read line"))
        .collect();
}

fn main() {
    let lines = get_input_data();
    let mut graph = Graph::new();

    for line in lines.iter() {
        let step = Step::from(line.as_str());
        graph.add_step(step);
    }
    
    println!("Exercise one: {}", String::from_iter(GraphIterator::from(&graph)));
}
