use std::cell::RefCell;
use std::collections::BTreeSet;
use std::rc::Rc;

use std::cmp::Ordering;

pub type NodeRef = Rc<RefCell<Node>>;

#[derive(Debug)]
pub struct Node {
  pub name: String,
  pub dependencies: RefCell<BTreeSet<NodeRef>>,
}

impl Node {
  pub fn new(name: String) -> Node {
    Node {
      name: name,
      dependencies: RefCell::new(BTreeSet::new()),
    }
  }

  pub fn add_dependency(&self, dependency: NodeRef) {
    let mut deps = self.dependencies.borrow_mut();
    deps.insert(dependency.clone());
  }
}

impl Ord for Node {
  fn cmp(&self, other: &Node) -> Ordering {
    self.name.cmp(&other.name)
  }
}

impl PartialOrd for Node {
  fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Eq for Node {}

impl PartialEq for Node {
  fn eq(&self, other: &Node) -> bool {
    self.name == other.name
  }
}

impl From<&str> for Node {
  fn from(name: &str) -> Node {
    Node::new(String::from(name))
  }
}

#[cfg(test)]
mod test {

  use std::collections::BTreeSet;
  use std::cell::RefCell;
  use std::rc::Rc;

  use super::Node;
  use super::NodeRef;

  #[test]
  fn add_depdendency() {
    let node = Node::from("A");
    let dependency: NodeRef = Rc::new(RefCell::new(Node::from("A")));

    node.add_dependency(dependency.clone());
    let mut expected_dependencies = BTreeSet::new();
    expected_dependencies.insert(dependency);

    assert_eq!(String::from("A"), node.name);
    assert_eq!(expected_dependencies, *node.dependencies.borrow());
  }
}
