
mod node;
pub mod step;

use std::cell::RefCell;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::rc::Rc;

use self::node::Node;
use self::node::NodeRef;
use self::step::Step;
pub struct Graph {
  nodes: HashMap<String, NodeRef>,
}

impl Graph {
  pub fn new() -> Graph {
    Graph {
      nodes: HashMap::new(),
    }
  }

  pub fn add_step(&mut self, step: Step) {
    let target = self.get_or_insert(String::from(step.target));
    let dependency = self.get_or_insert(String::from(step.dependency));

    target.borrow().add_dependency(dependency);
  }

  fn get_or_insert(&mut self, node_name: String) -> NodeRef {
    self
      .nodes
      .entry(node_name.clone())
      .or_insert(Rc::new(RefCell::new(Node::new(node_name))).clone())
      .clone()
  }
}

pub struct GraphIterator {
  candidates: BTreeSet<NodeRef>,
  dependencies_met: BTreeSet<NodeRef>,
}

impl Iterator for GraphIterator {
  type Item = String;

  fn next(&mut self) -> Option<String> {
    let next = self.find_suitable_candidate()?;
    self.candidates.remove(&next);
    self.dependencies_met.insert(next.clone());

    return Some(next.clone().borrow().name.clone());
  }
}

impl GraphIterator {
  fn find_suitable_candidate(&mut self) -> Option<NodeRef> {
    for candidate in self.candidates.iter() {
      let dependencies = candidate.borrow().dependencies.clone();
      if dependencies.borrow().is_subset(&self.dependencies_met) {
        return Some(candidate.clone());
      }
    }

    return None;
  }
}

impl From<&Graph> for GraphIterator {
  fn from(graph: &Graph) -> GraphIterator {
    let candidates: BTreeSet<NodeRef> = graph.nodes.iter().map(|(_, node)| node.clone()).collect();

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
        target: String::from("A"),
        dependency: String::from("C"),
      },
      Step {
        target: String::from("F"),
        dependency: String::from("C"),
      },
      Step {
        target: String::from("B"),
        dependency: String::from("A"),
      },
      Step {
        target: String::from("D"),
        dependency: String::from("A"),
      },
      Step {
        target: String::from("E"),
        dependency: String::from("B"),
      },
      Step {
        target: String::from("E"),
        dependency: String::from("D"),
      },
      Step {
        target: String::from("E"),
        dependency: String::from("F"),
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
