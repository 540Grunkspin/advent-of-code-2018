extern crate regex;

use regex::Regex;
use std::collections::HashSet;

#[derive(PartialEq, Debug)]
pub struct Step {
    name: String,
    children: HashSet<String>
}

impl Step {
    pub fn new(name: String) -> Step {
        Step {
            name: name,
            children: HashSet::new(),
        }
    }

    fn add_child(&mut self, child: String) {
        self.children.insert(child);
    }

    pub fn merge(mut self, other: Step) -> Result<Step, String> {
        if (self.name != other.name) {
            return Err(String::from("Can not merge nodes of different name"));
        }

        for item in other.children.into_iter() {
            self.children.insert(item);
        }


        Ok(Step { name: self.name, children: self.children})
    }
}

impl<'a> From<String> for Step {
    fn from(input: String) -> Step {
        let re = Regex::new(r"^Step ([A-Z]) must be finished before step ([A-Z]) can begin.$").unwrap();

        let captures = re.captures(input.as_str()).unwrap();
        let mut step = Step::new(captures[1].to_string());

        step.add_child(captures[2].to_string());
        return step;
    }
}

#[cfg(test)]
mod test {
    use super::Step;
    use std::collections::HashSet;
    use std::iter::FromIterator;

    #[test]
    fn test_step_from() {
        let step = Step::from("Step G must be finished before step T can begin.".to_string());
        assert_eq!(
            step,
            Step {
                name: "G".to_string(),
                children: HashSet::from_iter(vec!["T".to_string()].into_iter())
            }
        );
    }

    #[test]
    fn test_merge() {
        let original = Step { name: "G".to_string(), children: HashSet::from_iter(vec!["T".to_string()].into_iter()) };
        let to_merge = Step { name: "G".to_string(), children: HashSet::from_iter(vec!["H".to_string()].into_iter()) };

        let merged = original.merge(to_merge).expect("Could not merge");

        assert_eq!(merged, Step { name: "G".to_string(), children: HashSet::from_iter(vec!["T".to_string(), "H".to_string()].into_iter()) });
    }
}
