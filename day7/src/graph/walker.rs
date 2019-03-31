use std::cell::RefCell;
use std::collections::BTreeMap;
use std::collections::HashSet;

pub trait Walker<'a> {
  fn walk_to_next(
    &self,
    nodes: &BTreeMap<&'a str, HashSet<&'a str>>,
    completed_steps: &HashSet<&'a str>,
  ) -> &'a str;
}

pub struct SimpleWalker {}

impl SimpleWalker {
  pub fn new() -> SimpleWalker {
    SimpleWalker {}
  }
}

impl<'a> Walker<'a> for SimpleWalker {
  fn walk_to_next(
    &self,
    nodes: &BTreeMap<&'a str, HashSet<&'a str>>,
    completed_steps: &HashSet<&'a str>,
  ) -> &'a str {
    nodes
      .iter()
      .filter(|&(key, val)| val.is_subset(&completed_steps) && !completed_steps.contains(key))
      .map(|(key, _)| key)
      .next()
      .unwrap()
  }
}

pub struct DelayedWalker<'a> {
  walker: Box<Walker<'a>>,
  prev: RefCell<Option<&'a str>>,
  wait_time: RefCell<u32>,
  total_time: RefCell<i32>,
}

impl<'a> DelayedWalker<'a> {
  pub fn new(walker: Box<Walker<'a>>) -> DelayedWalker<'a> {
    DelayedWalker {
      walker: walker,
      prev: RefCell::new(None),
      wait_time: RefCell::new(0),
      total_time: RefCell::new(0),
    }
  }
}

impl<'a> Walker<'a> for DelayedWalker<'a> {
  fn walk_to_next(
    &self,
    nodes: &BTreeMap<&'a str, HashSet<&'a str>>,
    completed_steps: &HashSet<&'a str>,
  ) -> &'a str {
    *self.total_time.borrow_mut() += 1;
    let mut wait_time = self.wait_time.borrow_mut();
    let mut prev = self.prev.borrow_mut();

    if *wait_time == 0 {
      let next = self.walker.walk_to_next(nodes, completed_steps);
      *wait_time = (next.chars().next().unwrap() as u32) - ('A' as u32);

      *prev = Some(next);

      return next;
    } else {
      *wait_time -= 1;
      return prev.unwrap_or(".");
    }
  }
}

#[cfg(test)]
mod test {
  use super::DelayedWalker;
  use super::SimpleWalker;
  use super::Walker;

  use std::collections::BTreeMap;
  use std::collections::HashSet;

  #[test]
  fn simple_walker_walks() {
    let walker = SimpleWalker::new();

    let mut nodes = BTreeMap::new();
    nodes.insert("A", HashSet::new());

    let mut deps = HashSet::new();
    deps.insert("A");

    nodes.insert("B", deps);

    let mut completed_steps = &mut HashSet::new();

    assert_eq!("A", walker.walk_to_next(&nodes, completed_steps));

    completed_steps.insert("A");

    assert_eq!("B", walker.walk_to_next(&nodes, completed_steps));
  }

  #[test]
  fn delayed_walker_waits() {
    let wrapped = Box::new(SimpleWalker::new());
    let walker = DelayedWalker::new(wrapped);

    let mut nodes = BTreeMap::new();
    nodes.insert("B", HashSet::new());

    let mut deps = HashSet::new();
    deps.insert("B");

    nodes.insert("A", deps);

    let mut completed_steps = HashSet::new();

    assert_eq!("B", walker.walk_to_next(&nodes, &completed_steps));

    completed_steps.insert("B");

    assert_eq!("B", walker.walk_to_next(&nodes, &completed_steps));
    assert_eq!("A", walker.walk_to_next(&nodes, &completed_steps));
  }
}
