use std::ops::Index;

// Refactor:
// -Should Arena be untyped? I don't really think it needs to be since I won't be holding heterogenous data.
// -Should the get_elements method return a Vec<&T> or a Vec<T>
// -Default capacity might need to be resized. May also make sense for it to be a static or something defined by a load in parameter.

///Structure for holding preallocated data. [See more](https://en.wikipedia.org/wiki/Region-based_memory_management).
pub struct Arena<T> {
  ///Containter for allocated data.
  data: Vec<T>,
  ///Pointer to the next free arena slot.
  ptr_next: usize,
  ///Number of elements stored in the [`Arena`].
  len: usize,
}

pub type ArenaId = usize;

impl<T> Arena<T> {
  ///Default capacity of the [`Arena`].
  const DEFAULT_CAP: usize = 10;

  ///Create a new [`Arena`] with size equal to `DEFAULT_ALLOC`.
  pub fn new() -> Self {
    Arena {
      data: Vec::with_capacity(Self::DEFAULT_CAP),
      ptr_next: 0,
      len: 0,
    }
  }

  ///Loads data into [`Arena`] and returns a handle to the data.
  pub fn alloc(&mut self, data: T) -> ArenaId {
    self.data.push(data);
    let id = self.ptr_next;
    self.ptr_next += 1;
    self.len += 1;

    //Return the id of the newly inserted data
    id
  }

  ///Creates an [`Arena`] with a custom capcity.
  pub fn with_capacity(cap: usize) -> Self {
    Arena {
      data: Vec::with_capacity(cap),
      ptr_next: 0,
      len: 0,
    }
  }

  ///Return elements matching the submitted [`ArenaId`]s.
  pub fn get_elements(&self, ids: Vec<ArenaId>) -> Vec<&T> {
    ids.into_iter().map(|id| &self.data[id]).collect::<Vec<_>>()
  }

  ///Returns the number of elements stored in the [`Arena`].
  pub fn len(&self) -> usize {
    self.len
  }

  ///Return the element matching the submitted [`ArenaId`].
  pub fn get(&self, id: ArenaId) -> &T {
    &self.data[id]
  }
}

impl<T> Index<usize> for Arena<T> {
  type Output = T;

  fn index(&self, index: usize) -> &Self::Output {
    &self.data[index]
  }
}
