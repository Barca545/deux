use std::{hash::Hash, marker::PhantomData, ops::Index};

// Refactor:
// - Make the Arena generational I *think* this is as easy as making the ID
//   track it's generation but confirm
// - Make get_mut/get_elements_mut methods
// - Default capacity might need to be resized. May also make sense for it to be
//   a static or something defined by a load in parameter.

/// Structure for holding preallocated data. [See more](https://en.wikipedia.org/wiki/Region-based_memory_management).
pub struct Arena<T,> {
  /// Containter for allocated data.
  data: Vec<T,>,
  /// Pointer to the next free slot.
  ptr_next: usize,
  /// Number of elements stored in the [`Arena`].
  len: usize,
}

impl<T,> Arena<T,> {
  /// Default capacity of the [`Arena`].
  const DEFAULT_CAP: usize = 100;

  /// Create a new [`Arena`] with size equal to `DEFAULT_CAP`.
  pub fn new() -> Self {
    Arena {
      data: Vec::with_capacity(Self::DEFAULT_CAP,),
      ptr_next: 0,
      len: 0,
    }
  }

  /// Loads data into [`Arena`] and returns a `usize` handle to the data.
  pub fn alloc(&mut self, data: T,) -> ArenaId<T,> {
    self.data.push(data,);
    let id = self.ptr_next;
    self.ptr_next += 1;
    self.len += 1;

    // Return the id of the newly inserted data
    ArenaId {
      id,
      _data: PhantomData,
    }
  }

  /// Creates an [`Arena`] with a custom capcity.
  pub fn with_capacity(cap: usize,) -> Self {
    Arena {
      data: Vec::with_capacity(cap,),
      ptr_next: 0,
      len: 0,
    }
  }
  /// Returns the number of elements stored in the [`Arena`].
  pub fn len(&self,) -> usize {
    self.len
  }

  /// Return elements matching the submitted [`ArenaId`]s.
  pub fn get_elements(&self, ids: Vec<ArenaId<T,>,>,) -> Vec<&T,> {
    ids
      .into_iter()
      .map(|id| &self.data[id.id],)
      .collect::<Vec<_,>>()
  }

  /// Return the element matching the submitted [`ArenaId`].
  pub fn get(&self, id: &ArenaId<T,>,) -> &T {
    &self.data[id.id]
  }
}

#[derive(Debug,)]
/// [Newtype](https://doc.rust-lang.org/rust-by-example/generics/new_types.html) for the ID of an element inside an [`Arena`]. Used
pub struct ArenaId<T,> {
  id: usize,
  _data: PhantomData<*const T,>,
}

impl<T,> Clone for ArenaId<T,> {
  fn clone(&self,) -> Self {
    Self {
      id: self.id.clone(),
      _data: self._data.clone(),
    }
  }
}

impl<T,> Copy for ArenaId<T,> {}

impl<T,> PartialEq for ArenaId<T,> {
  fn eq(&self, other: &Self,) -> bool {
    self.id == other.id
  }
}

impl<T,> Eq for ArenaId<T,> {}

impl<T,> Hash for ArenaId<T,> {
  fn hash<H: std::hash::Hasher,>(&self, state: &mut H,) {
    self.id.hash(state,);
  }
}

// TODO: Could adapt this entry idea for the slab pool thing in SPDR
// just get rid of the gen field and make next only a u32
// Or not the pool but the free list the allocator uses. Explore that.
#[derive(Debug, Clone, Copy,)]
enum ArenaEntry<T,> {
  Entry { gen: u64, data: T, },
  Empty { next: usize, },
}

#[derive(Debug,)]
pub struct GenArenaIndex<T,> {
  idx: usize,
  generation: u64,
  _data: PhantomData<*const T,>,
}

impl<T,> Clone for GenArenaIndex<T,> {
  fn clone(&self,) -> Self {
    Self {
      idx: self.idx.clone(),
      generation: self.generation.clone(),
      _data: self._data.clone(),
    }
  }
}

impl<T,> Copy for GenArenaIndex<T,> {}

/// Design inspired by https://docs.rs/generational-arena/latest/generational_arena/
/// The default capacity is 10.
pub struct GenerationalArena<T,> {
  storage: Vec<ArenaEntry<T,>,>,
  free: usize,
}

impl<T,> GenerationalArena<T,> {
  const DEFAULT_CAP: usize = 10;

  /// Create a new `GenerationalArena` with the default capacity.
  pub fn new() -> Self {
    // for i in 0..Self::DEFAULT_CAP {
    //   storage[i] = ArenaEntry::Empty { next: i + 1, }
    // }

    Self {
      storage: Vec::with_capacity(Self::DEFAULT_CAP,),
      free: 0,
    }
  }

  /// Create a new `GenerationalArena` with the provided capacityy
  pub fn with_capacity(cap: usize,) -> Self {
    // for i in 0..cap {
    //   storage[i] = ArenaEntry::Empty { next: i + 1, }
    // }

    Self {
      storage: Vec::with_capacity(cap,),
      free: 0,
    }
  }

  pub fn insert(&mut self, data: T,) {
    // Place the data in the next free cell.
    self.storage[self.free] = ArenaEntry::Entry {
      gen: 0, data: data,
    };
    // Update the next free cell and freelist
    todo!()
  }

  pub fn remove(&mut self,) -> T {
    // Update the next free cell and freelist
    todo!()
  }

  pub fn get(&self, idx: GenArenaIndex<T,>,) -> Option<&T,> {
    // TODO: Confirm the index is the correct generation and return...something if
    // not
    match &self.storage[idx.idx] {
      ArenaEntry::Entry { data, .. } => Some(data,),
      ArenaEntry::Empty { .. } => None,
    }
  }

  pub fn get_mut(&mut self, idx: GenArenaIndex<T,>,) -> Option<&mut T,> {
    // TODO: Confirm the index is the correct generation and return...something if
    // not
    match &mut self.storage[idx.idx] {
      ArenaEntry::Entry { data, gen, } => {
        // Update the generation when accessed mutably
        *gen += 1;
        Some(data,)
      }
      ArenaEntry::Empty { .. } => None,
    }
  }
}
