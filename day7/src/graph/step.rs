extern crate regex;

use regex::Regex;

#[derive(PartialEq, Debug, Clone)]
pub struct Step {
    pub target: String,
    pub dependency: String,
}

impl Step {
    pub fn new(name: &str, dependency: &str) -> Step {
        Step {
            target: String::from(name),
            dependency: String::from(dependency),
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
            captures.get(2).unwrap().as_str(),
            captures.get(1).unwrap().as_str(),
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
                target: String::from("T"),
                dependency: String::from("G")
            }
        );
    }
}
