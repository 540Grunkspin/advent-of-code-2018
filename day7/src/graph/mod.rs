pub mod step;
mod walker;

use std::collections::BTreeMap;
use std::collections::HashSet;

use self::step::Step;
use self::walker::SimpleWalker;
use self::walker::Walker;

pub struct GraphBuilder<'a> {
  walker: Box<Walker<'a>>,
  nodes: BTreeMap<&'a str, HashSet<&'a str>>,
}

impl<'a> GraphBuilder<'a> {
  pub fn new() -> GraphBuilder<'a> {
    let nodes = BTreeMap::new();
    GraphBuilder {
      nodes: nodes,
      walker: Box::new(SimpleWalker::new()),
    }
  }

  pub fn with_walker(&mut self, walker: Box<Walker<'a>>) -> &mut GraphBuilder<'a> {
    self.walker = walker;
    self
  }

  pub fn add_step(&mut self, step: Step<'a>) -> &mut GraphBuilder<'a> {
    self
      .nodes
      .entry(step.target)
      .or_insert(HashSet::new())
      .insert(step.dependency);

    self.nodes.entry(step.dependency).or_insert(HashSet::new());

    self
  }

  pub fn build(self) -> Graph<'a> {
    Graph::new(self.walker, self.nodes)
  }
}

pub struct Graph<'a> {
  nodes: BTreeMap<&'a str, HashSet<&'a str>>,
  walker: Box<Walker<'a>>,
}

impl<'a> Graph<'a> {
  fn new(walker: Box<Walker<'a>>, nodes: BTreeMap<&'a str, HashSet<&'a str>>) -> Graph<'a> {
    Graph {
      nodes: nodes,
      walker: walker,
    }
  }

  pub fn walk(&self) -> String {
    let mut completed_steps: HashSet<&'a str> = HashSet::new();
    let mut visited: Vec<&'a str> = Vec::new();
    let all_nodes = self.nodes.keys().map(|x| *x).collect::<HashSet<&'a str>>();

    while completed_steps != all_nodes {
      let next_target = self.walker.walk_to_next(&self.nodes, &mut completed_steps);
      completed_steps.insert(next_target);
      visited.push(next_target);
    }

    return visited.join("");
  }
}

#[cfg(test)]
mod test {
  use super::step::Step;
  use super::GraphBuilder;

  #[test]
  fn test_walk() {
    let steps = vec![
      Step {
        target: "A",
        dependency: "C",
      },
      Step {
        target: "F",
        dependency: "C",
      },
      Step {
        target: "B",
        dependency: "A",
      },
      Step {
        target: "D",
        dependency: "A",
      },
      Step {
        target: "E",
        dependency: "B",
      },
      Step {
        target: "E",
        dependency: "D",
      },
      Step {
        target: "E",
        dependency: "F",
      },
    ];

    let mut builder = GraphBuilder::new();

    for step in steps {
      builder.add_step(step);
    }

    let graph = builder.build();

    assert_eq!(String::from("CABDFE"), graph.walk());
  }
}
