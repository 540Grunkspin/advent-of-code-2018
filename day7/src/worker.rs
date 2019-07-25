use std::collections::HashSet;

pub trait Worker<Task> {
  fn is_available(&self) -> bool;
  fn has_work(&self) -> bool;
  fn add_work(&mut self, input: char) -> Result<(), &'static str>;
  fn work(&mut self) -> Option<Task>;
  fn time_worked(&self) -> u32;
  fn working_on(&self) -> Option<Task>;
}

pub struct BasicWorker {
  work_time: u32,
  work: Vec<char>,
}

impl BasicWorker {
  pub fn new() -> BasicWorker {
    BasicWorker {
      work_time: 0,
      work: Vec::new(),
    }
  }
}

impl Worker<char> for BasicWorker {
  fn is_available(&self) -> bool {
    self.work.is_empty()
  }

  fn has_work(&self) -> bool {
    !self.is_available()
  }

  fn working_on(&self) -> Option<char> {
    self.work.first().map(|task| task.clone())
  }

  fn add_work(&mut self, input: char) -> Result<(), &'static str> {
    if !self.is_available() {
      return Err("Can not add work to an already working worker.");
    }

    let work_amount = (input as usize) - ('A' as usize) + 61;
    for _ in 0..work_amount {
      self.work.push(input);
    }
    return Ok(());
  }

  fn time_worked(&self) -> u32 {
    self.work_time
  }

  fn work(&mut self) -> Option<char> {
    let done_with = self.work.pop();
    self.work_time += 1;

    if done_with.is_some() && self.work.is_empty() {
      return done_with;
    }

    return None;
  }
}

pub struct CompositeWorker {
  work_time: u32,
  workers: Vec<Box<dyn Worker<char>>>,
}

impl CompositeWorker {
  pub fn new() -> CompositeWorker {
    CompositeWorker {
      work_time: 0,
      workers: Vec::new(),
    }
  }

  pub fn add_worker(&mut self, worker: Box<dyn Worker<char>>) {
    self.workers.push(worker);
  }
}

impl Worker<HashSet<char>> for CompositeWorker {
  fn is_available(&self) -> bool {
    self.workers.iter().any(|worker| worker.is_available())
  }

  fn has_work(&self) -> bool {
    self.workers.iter().any(|worker| worker.has_work())
  }

  fn add_work(&mut self, work: char) -> Result<(), &'static str> {
    match self.workers.iter_mut().find(|worker| worker.is_available()) {
      Some(worker) => worker.add_work(work),
      None => Err("Can not add work to an already working worker"),
    }
  }

  fn work(&mut self) -> Option<HashSet<char>> {
    let mut result: HashSet<char> = HashSet::new();
    self.work_time += 1;

    for worker in self.workers.iter_mut() {
      if let Some(task) = worker.work() {
        result.insert(task);
      }
    }

    if result.is_empty() {
      return None;
    }

    return Some(result);
  }

  fn working_on(&self) -> Option<HashSet<char>> {
    let mut result: HashSet<char> = HashSet::new();
    for worker in self.workers.iter() {
      if let Some(task) = worker.working_on() {
        result.insert(task);
      }
    }

    if result.is_empty() {
      None
    } else {
      Some(result)
    }
  }

  fn time_worked(&self) -> u32 {
    self.work_time
  }
}
