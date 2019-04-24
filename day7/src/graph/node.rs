use std::cell::RefCell;
use std::collections::BTreeSet;
use std::rc::Rc;

use std::cmp::Ordering;

type NodeRef = Rc<RefCell<_Node>>;

#[derive(Debug)]
struct _Node {
  name: char,
  dependencies: BTreeSet<NodeRef>,
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone)]
pub struct Node(NodeRef);

impl Node {
  fn new(name: char) -> Node {
    Node(
      Rc::new(
        RefCell::new(
          _Node {
            name: name,
            dependencies: BTreeSet::new(),
          }
          )
        )
      )
  }

  pub fn name(&self) -> char {
    self.0.borrow().name
  }

  pub fn are_dependencies_satisfied(&self, met_dependencies: &BTreeSet<Node>) -> bool {
    let met_node_refs = met_dependencies.iter().map(|node| node.clone().0);
    self.0.borrow().dependencies.is_subset(&met_node_refs.collect())
  }

  pub fn add_dependency(&self, dependency: Node) {
    let mut node = self.0.borrow_mut();
    node.dependencies.insert(dependency.0.clone());
  }
}

impl Ord for _Node {
  fn cmp(&self, other: &_Node) -> Ordering {
    self.name.cmp(&other.name)
  }
}

impl PartialOrd for _Node {
  fn partial_cmp(&self, other: &_Node) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Eq for _Node {}

impl PartialEq for _Node {
  fn eq(&self, other: &_Node) -> bool {
    self.name == other.name
  }
}

impl From<char> for Node {
  fn from(name: char) -> Node {
    Node::new(name)
  }
}

#[cfg(test)]
mod test {

  use std::collections::BTreeSet;

  use super::Node;

  #[test]
  fn add_depdendency() {
    let node = Node::from('A');
    let dependency: Node = Node::from('A');

    node.add_dependency(dependency.clone());
    let mut expected_dependencies = BTreeSet::new();
    expected_dependencies.insert(dependency);

    assert_eq!('A', node.name());
    assert!(node.are_dependencies_satisfied(&expected_dependencies))
  }
}
