mod graph;
mod worker;

use std::env::args;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use self::graph::Entry;
use self::graph::Graph;

use self::worker::BasicWorker;
use self::worker::CompositeWorker;
use self::worker::Worker;

fn get_input_data() -> Vec<String> {
    let file_path = args().nth(1).expect("Input file path is required");
    let file = File::open(file_path).expect("Could not open file");
    let reader = BufReader::new(file);

    return reader
        .lines()
        .map(|line| line.expect("Could not read line"))
        .collect();
}

fn exercise_one(graph: Graph) -> String {
    graph.iter().collect()
}

fn exercise_two(graph: Graph) -> Result<u32, &'static str> {
    let mut workers = CompositeWorker::new();
    for _ in 0..5 {
        workers.add_worker(Box::new(BasicWorker::new()));
    }

    let mut iter = graph.iter();

    loop {
        for task in iter.available_tasks() {
            if workers
                .working_on()
                .map_or(true, |working_tasks| !working_tasks.contains(&task))
                && workers.is_available()
            {
                workers.add_work(task)?;
            }
        }

        if let Some(completed_tasks) = workers.work() {
            for &task in completed_tasks.iter() {
                iter.complete(task);
            }
        }

        if iter.next_task().is_none() {
            break;
        }
    }
    return Ok(workers.time_worked());
}

fn main() -> Result<(), &'static str> {
    let entries = get_input_data().into_iter().map(|input| Entry::from(input));
    let mut graph = Graph::new();

    for entry in entries {
        graph.add_entry(entry);
    }

    println!("Exercise one: {}", exercise_one(graph.clone()));
    println!("Exercise two: {}", exercise_two(graph.clone())?);

    return Ok(());
}
