use std::collections::HashMap;

// Refactor:
// - Use the final implementation from here: https://matklad.github.io/2020/03/22/fast-simple-rust-interner.html

pub struct Interner {
  map:HashMap<String, u32,>,
  vec:Vec<String,>,
}

impl Interner {
  pub fn new() -> Self {
    Interner {
      map:HashMap::new(),
      vec:Vec::new(),
    }
  }
  pub fn intern(&mut self, str:&str,) -> u32 {
    match self.map.get(str,) {
      Some(idx,) => *idx,
      None => {
        let idx = self.map.len() as u32;
        self.map.insert(str.to_owned(), idx,);
        self.vec.push(str.to_owned(),);
        idx
      }
    }
  }

  pub fn lookup(&self, idx:u32,) -> &str {
    self.vec[idx as usize].as_str()
  }
}
