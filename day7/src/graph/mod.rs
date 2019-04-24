
mod node;
pub mod step;

use std::collections::BTreeSet;
use std::collections::HashMap;

use self::node::Node;
use self::step::Step;

pub struct Graph {
  nodes: HashMap<char, Node>,
}

impl Graph {
  pub fn new() -> Graph {
    Graph {
      nodes: HashMap::new(),
    }
  }

  pub fn add_step(&mut self, step: Step) {
    let target = self.get_or_insert(step.target);
    let dependency = self.get_or_insert(step.dependency);

    target.add_dependency(dependency.clone());
  }

  fn get_or_insert(&mut self, node_name: char) -> Node {
    self
      .nodes
      .entry(node_name)
      .or_insert(Node::from(node_name))
      .clone()
  }
}

pub struct GraphIterator {
  candidates: BTreeSet<Node>,
  dependencies_met: BTreeSet<Node>,
}

impl Iterator for GraphIterator {
  type Item = char;

  fn next(&mut self) -> Option<char> {
    let next = self.find_suitable_candidate()?;
    self.candidates.remove(&next);
    self.dependencies_met.insert(next.clone());

    return Some(next.name());
  }
}

impl GraphIterator {
  fn find_suitable_candidate(&self) -> Option<Node> {
    for candidate in self.candidates.iter() {
      if candidate.are_dependencies_satisfied(&self.dependencies_met) {
        return Some(candidate.clone());
      }
    }

    return None;
  }
}

impl From<&Graph> for GraphIterator {
  fn from(graph: &Graph) -> GraphIterator {
    let candidates: BTreeSet<Node> = graph.nodes.iter().map(|(_, node)| node.clone()).collect();

    GraphIterator {
      dependencies_met: BTreeSet::new(),
      candidates: candidates,
    }
  }
}

#[cfg(test)]
mod test {
  use super::step::Step;
  use super::Graph;

  use super::GraphIterator;

  use std::iter::FromIterator;

  #[test]
  fn test_walk() {
    let steps = vec![
      Step {
        target: 'A',
        dependency: 'C',
      },
      Step {
        target: 'F',
        dependency: 'C',
      },
      Step {
        target: 'B',
        dependency: 'A',
      },
      Step {
        target: 'D',
        dependency: 'A',
      },
      Step {
        target: 'E',
        dependency: 'B',
      },
      Step {
        target: 'E',
        dependency: 'D',
      },
      Step {
        target: 'E',
        dependency: 'F',
      },
    ];

    let mut graph = Graph::new();

    for step in steps {
      graph.add_step(step);
    }

    let graph_iter = GraphIterator::from(&graph);

    assert_eq!(String::from("CABDFE"), String::from_iter(graph_iter));
  }
}
