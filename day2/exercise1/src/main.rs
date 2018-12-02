use std::string::String;
use std::vec::Vec;
use std::env::args;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashSet;
use std::collections::HashMap;

fn get_input_data() -> Vec<String> {
    let file_path = args().nth(1).expect("Input file path is required");
    let file = File::open(file_path).expect("Could not open file");
    let reader = BufReader::new(file);

    return reader.lines().map(|line| line.expect("Could not read line")).collect();
}

fn get_occurance_set(input: &String) -> HashSet<i32> {
    let mut set: HashSet<char> = HashSet::new();
    let mut occurances: HashSet<i32> = HashSet::new();
    for ch in input.chars() {
        if set.insert(ch) {
            let matches: Vec<char> = input.chars().filter(|&item| item == ch).collect();
            let number_of_matches = matches.len() as i32;
            if number_of_matches > 1 {
                occurances.insert(matches.len() as i32);
            }
        }
    }

    return occurances;
}

fn calculate_check_sum(input: &Vec<HashSet<i32>>) -> i32 {
    let mut occurance_map: HashMap<&i32, i32> = HashMap::new();
    for occurance_set in input.iter() {
        for occurance in occurance_set.iter() {
            let next_value = match occurance_map.get(occurance) {
                None => 1,
                Some(x) => x + 1,
            };

            occurance_map.insert(occurance, next_value);
        }
    }

    return occurance_map.values().fold(1, |acc, val| acc * val);
}

fn main() {
    let ids = get_input_data();
    let occurance_set_list: Vec<HashSet<i32>> = ids.iter().map(get_occurance_set).collect();
    let checksum = calculate_check_sum(&occurance_set_list);

    println!("{}", checksum)
}

#[cfg(test)]
mod test {
    use ::get_occurance_set;
    use ::calculate_check_sum;
    use std::collections::HashSet;
    use std::vec::Vec;

    #[test]
    fn test_get_occurances_no_repeats() {
        let result = get_occurance_set(&String::from("abcdef"));
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_get_ocucrances_two_and_three() {
        let result = get_occurance_set(&String::from("bababc"));
        assert_eq!(result.len(), 2);
        assert!(result.contains(&2));
        assert!(result.contains(&3));
    }

    #[test]
    fn test_calculate_check_sum() {
        let mut input: Vec<HashSet<i32>> = Vec::new();
        let mut first_set: HashSet<i32> = HashSet::new();
        first_set.insert(2);
        first_set.insert(3);

        let mut second_set: HashSet<i32> = HashSet::new();
        second_set.insert(2);
        second_set.insert(3);

        input.push(first_set);
        input.push(second_set);

        assert_eq!(calculate_check_sum(&input), 4);
    }
}
