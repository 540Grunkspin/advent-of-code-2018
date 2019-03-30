extern crate regex;

use regex::Regex;

#[derive(PartialEq, Debug)]
pub struct Step<'a> {
    pub target: &'a str,
    pub dependency: &'a str,
}

impl<'a> Step<'a> {
    pub fn new(name: &'a str, dependency: &'a str) -> Step<'a> {
        Step {
            target: name,
            dependency: dependency,
        }
    }
}

impl<'a> From<&'a str> for Step<'a> {
    fn from(input: &'a str) -> Step<'a> {
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
                target: "T",
                dependency: "G"
            }
        );
    }
}
