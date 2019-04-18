
use std::cell::RefCell;

use std::collections::HashMap;
use std::rc::Rc;

use super::step::Step;

type NodeRef = Rc<RefCell<Node>>;

pub struct Node {
  name: String,
  dependencies: RefCell<Vec<NodeRef>>,
}

impl Node {
  pub fn new(name: String) -> Node {
    Node {
      name: name,
      dependencies: RefCell::new(vec![]),
    }
  }

  pub fn add_dependency(&self, dependency: NodeRef) {
    let mut deps = self.dependencies.borrow_mut();
    deps.push(dependency.clone());
  }
}

pub struct GraphBuilder {
  nodes: HashMap<String, NodeRef>,
}

impl<'a> GraphBuilder {
  pub fn new() -> GraphBuilder {
    GraphBuilder {
      nodes: HashMap::new(),
    }
  }

  pub fn add_step(&mut self, step: &Step<'a>) {
    let target = self.get_or_insert(String::from(step.target));
    let dependency = self.get_or_insert(String::from(step.dependency));

    let target_borrow = target.borrow();

    target_borrow.add_dependency(dependency)
  }

  fn get_or_insert(&mut self, node_name: String) -> NodeRef {
    self
      .nodes
      .entry(node_name.clone())
      .or_insert(Rc::new(RefCell::new(Node::new(node_name))).clone())
      .clone()
  }
}