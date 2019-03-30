pub mod step;

use std::collections::BTreeMap;
use std::collections::HashSet;

use self::step::Step;

pub struct GraphBuilder<'a> {
  nodes: BTreeMap<&'a str, HashSet<&'a str>>,
}

impl<'a> GraphBuilder<'a> {
  pub fn new() -> GraphBuilder<'a> {
    GraphBuilder {
      nodes: BTreeMap::new(),
    }
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
    Graph::new(self.nodes)
  }
}

pub struct Graph<'a> {
  nodes: BTreeMap<&'a str, HashSet<&'a str>>,
}

impl<'a> Graph<'a> {
  fn new(nodes: BTreeMap<&'a str, HashSet<&'a str>>) -> Graph<'a> {
    Graph { nodes: nodes }
  }

  pub fn walk(&self) -> String {
    let mut visited: HashSet<&'a str> = HashSet::new();
    let mut ordered_visited: Vec<&'a str> = Vec::new();
    let all_nodes = self.nodes.keys().map(|x| *x).collect::<HashSet<&'a str>>();

    while visited != all_nodes {
      let next_target = self.get_next_target(&visited);
      visited.insert(next_target);
      ordered_visited.push(next_target);
    }

    return ordered_visited.join("");
  }

  fn get_next_target(&self, visited: &HashSet<&'a str>) -> &'a str {
    self
      .nodes
      .iter()
      .filter(|&(key, val)| val.is_subset(&visited) && !visited.contains(key))
      .map(|(key, _)| key)
      .next()
      .unwrap()
  }
}
