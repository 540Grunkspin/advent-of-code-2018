extern crate regex;

use regex::Regex;
use std::collections::HashSet;

#[derive(PartialEq, Debug)]
pub struct Step<'a> {
    name: &'a str,
    children: HashSet<&'a str>,
}

impl<'a> Step<'a> {
    pub fn new(name: &'a str) -> Step<'a> {
        Step {
            name: name,
            children: HashSet::new(),
        }
    }

    fn add_child(&mut self, child: &'a str) {
        self.children.insert(child);
    }

    pub fn merge(mut self, other: Step<'a>) -> Result<Step, &'a str> {
        if self.name != other.name {
            return Err("Can not merge nodes of different name");
        }

        for item in other.children.iter() {
            self.children.insert(item);
        }

        Ok(Step {
            name: self.name,
            children: self.children,
        })
    }
}

impl<'a> From<&'a str> for Step<'a> {
    fn from(input: &'a str) -> Step<'a> {
        let re =
            Regex::new(r"^Step ([A-Z]) must be finished before step ([A-Z]) can begin.$").unwrap();

        let captures = re.captures(input).unwrap();
        let mut step = Step::new(captures.get(1).unwrap().as_str());

        step.add_child(captures.get(2).unwrap().as_str());
        return step;
    }
}

#[cfg(test)]
mod test {
    use super::Step;

    #[test]
    fn test_step_from() {
        let step = Step::from("Step G must be finished before step T can begin.");
        assert_eq!(
            step,
            Step {
                name: "G",
                children: ["T"].iter().cloned().collect()
            }
        );
    }

    #[test]
    fn test_merge() {
        let original = Step {
            name: "G",
            children: ["T"].iter().cloned().collect(),
        };
        let to_merge = Step {
            name: "G",
            children: ["H"].iter().cloned().collect(),
        };

        let merged = original.merge(to_merge).expect("Could not merge");

        assert_eq!(
            merged,
            Step {
                name: "G",
                children: ["T", "H"].iter().cloned().collect()
            }
        );
    }
}
