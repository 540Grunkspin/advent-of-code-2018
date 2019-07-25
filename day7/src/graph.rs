use std::collections::BTreeMap;
use std::collections::HashSet;

#[derive(Clone)]
pub struct Graph(BTreeMap<char, HashSet<char>>);

pub struct Entry {
  name: char,
  dependency: char,
}

impl From<String> for Entry {
  fn from(input: String) -> Entry {
    let name = input.chars().nth(36).unwrap();
    let dependency = input.chars().nth(5).unwrap();

    Entry {
      name: name,
      dependency: dependency,
    }
  }
}

impl Graph {
  pub fn new() -> Graph {
    Graph(BTreeMap::new())
  }

  pub fn add_entry(&mut self, entry: Entry) {
    self.0.entry(entry.dependency).or_insert(HashSet::new());
    let dependencies = self.0.entry(entry.name).or_insert(HashSet::new());
    dependencies.insert(entry.dependency);
  }

  pub fn iter<'a>(&'a self) -> GraphIterator<'a> {
    GraphIterator::new(&self.0)
  }
}

pub struct GraphIterator<'a> {
  met_dependencies: HashSet<char>,
  items: &'a BTreeMap<char, HashSet<char>>,
}

impl<'a> GraphIterator<'a> {
  pub fn new(items: &'a BTreeMap<char, HashSet<char>>) -> GraphIterator<'a> {
    GraphIterator {
      met_dependencies: HashSet::new(),
      items: items,
    }
  }

  pub fn next_task(&self) -> Option<char> {
    for (key, val) in self.items.iter() {
      if self.met_dependencies.is_superset(&val) && !self.met_dependencies.contains(key) {
        return Some(*key);
      }
    }

    return None;
  }

  pub fn available_tasks(&self) -> HashSet<char> {
    let mut result = HashSet::new();

    for (key, val) in self.items.iter() {
      if self.met_dependencies.is_superset(&val) && !self.met_dependencies.contains(key) {
        result.insert(*key);
      }
    }

    return result;
  }

  pub fn complete(&mut self, task: char) {
    self.met_dependencies.insert(task);
  }
}

impl<'a> Iterator for GraphIterator<'a> {
  type Item = char;
  fn next(&mut self) -> Option<Self::Item> {
    if let Some(task) = self.next_task() {
      self.complete(task);
      return Some(task);
    }
    return None;
  }
}
