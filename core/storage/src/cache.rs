use std::{collections::HashMap, hash::Hash, marker::PhantomData};

pub struct Cache<T,>
where
  T: Cacheable,
{
  /// Tracks the [`CacheKey`] of data in the cache. Prevents
  /// duplication if the same data is added multiple times.
  keys: HashMap<T::Output, CacheKey<T,>,>,
  /// Stored data.
  storage: Vec<T,>,
}

impl<T: Cacheable,> Cache<T,> {
  /// Constructs a new, empty `Cache`.
  pub fn new() -> Self {
    Cache {
      keys: HashMap::new(),
      storage: Vec::new(),
    }
  }

  pub fn insert(&mut self, data: T,) -> CacheKey<T,> {
    match self.keys.get(&data.hash(),) {
      // If the data is already cached just return its ID
      Some(key,) => *key,
      // If the data is not cached we need to cache
      None => {
        let key = CacheKey {
          key: self.storage.len(),
          _data: PhantomData,
        };
        self.keys.insert(data.hash(), key,);
        self.storage.push(data,);
        key
      }
    }
  }

  pub fn get(&self, id: CacheKey<T,>,) -> &T {
    self.storage.get(id.key,).unwrap()
  }
}

#[derive(Debug,)]
pub struct CacheKey<T,> {
  key: usize,
  // This just exists so we can make it specific to the type
  // It's a pointer so the compiler doesn't care about the inner value's behavior
  _data: PhantomData<*const T,>,
}

impl<T,> Clone for CacheKey<T,> {
  fn clone(&self,) -> Self {
    *self
  }
}

impl<T,> Copy for CacheKey<T,> {}

// This trait exists essentially so you don't have to store T twice in a Cache.
// Ideally the output of Cacheable would be a simple usize or something else
// small
pub trait Cacheable {
  type Output: Eq + Hash;
  /// Converts the implementor into a hash which can be used as a [`HashMap`]
  /// key.
  fn hash(&self,) -> Self::Output;
}

impl<T,> Cacheable for T
where
  T: Eq + Hash + Clone,
{
  type Output = Self;

  fn hash(&self,) -> Self::Output {
    self.clone()
  }
}
