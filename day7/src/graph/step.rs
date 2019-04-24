extern crate regex;

use regex::Regex;

#[derive(PartialEq, Debug, Clone)]
pub struct Step {
    pub target: char,
    pub dependency: char,
}

impl Step {
    pub fn new(name: char, dependency: char) -> Step {
        Step {
            target: name,
            dependency: dependency,
        }
    }
}

impl From<&str> for Step {
    fn from(input: &str) -> Step {
        lazy_static! {
            static ref STEP_MATCHER: Regex =
                Regex::new(r"^Step ([A-Z]) must be finished before step ([A-Z]) can begin.$")
                    .unwrap();
        }

        let captures = STEP_MATCHER.captures(input).unwrap();

        return Step::new(
            captures.get(2).unwrap().as_str().chars().next().unwrap(),
            captures.get(1).unwrap().as_str().chars().next().unwrap(),
        );
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
                target: 'T',
                dependency: 'G'
            }
        );
    }
}
