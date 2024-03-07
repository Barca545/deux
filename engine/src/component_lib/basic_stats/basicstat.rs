use std::{
  marker::PhantomData,
  ops::{AddAssign, SubAssign},
};

#[derive(Debug, Clone, Copy)]
///A wrapper for basic stats.
/// `T` denotes the type of the stat.
/// `N` denotes the type of the integer the stat holds.
pub struct BasicStat<T, N> {
  base: N,
  bonus: N,
  total: N,
  ty: PhantomData<T>,
}

impl<T, N> BasicStat<T, N>
where
  N: SubAssign + AddAssign + Copy + Default,
{
  ///Creates a new instance of a [`BasicStat`].
  pub fn new(base: N) -> Self {
    BasicStat {
      //Value of the stat gained from leveling up.
      base,
      //Value of the stat derived from items and buffs
      bonus: N::default(),
      //Total value of the stat
      total: base,
      //Container of the basic type the stat holds
      ty: PhantomData,
    }
  }

  ///Add a number to a [`BasicStat`]'s `base` field.
  pub fn add_base(&mut self, amount: N) {
    self.base += amount;
    self.total += amount;
  }

  ///Subtract a number from a [`BasicStat`]'s `base` field.
  pub fn sub_base(&mut self, amount: N) {
    self.base -= amount;
    self.total -= amount;
  }

  ///Add a number to a [`BasicStat`]'s `bonus` field.
  pub fn add_bonus(&mut self, amount: N) {
    self.bonus += amount;
    self.total += amount;
  }

  ///Subtract a number from a [`BasicStat`]'s `bonus` field.
  pub fn sub_bonus(&mut self, amount: N) {
    self.bonus -= amount;
    self.total -= amount;
  }

  ///Return a [`BasicStat`]'s `base` field.
  pub fn base(&self) -> N {
    self.total
  }

  ///Return a [`BasicStat`]'s `bonus` field.
  pub fn bonus(&self) -> N {
    self.total
  }

  ///Return the sum of a [`BasicStat`]'s `base` and `bonus` fields.
  pub fn total(&self) -> N {
    self.total
  }
}

#[derive(Debug, Clone, Copy)]
pub struct BasicGrowableStat<T, N> {
  max: BasicStat<T, N>,
  pub remaining: N,
}

impl<T, N> BasicGrowableStat<T, N>
where
  N: SubAssign + AddAssign + Copy + Default,
{
  pub fn new(base: N) -> Self {
    let max = BasicStat::new(base);
    BasicGrowableStat { max, remaining: base }
  }

  ///Add a number to a [`BasicStat`]'s `base` field.
  pub fn add_base(&mut self, amount: N) {
    self.max.base += amount;
    self.max.total += amount;
  }

  ///Subtract a number from a [`BasicStat`]'s `base` field.
  pub fn sub_base(&mut self, amount: N) {
    self.max.base -= amount;
    self.max.total -= amount;
  }

  ///Add a number to a [`BasicStat`]'s `bonus` field.
  pub fn add_bonus(&mut self, amount: N) {
    self.max.bonus += amount;
    self.max.total += amount;
  }

  ///Subtract a number from a [`BasicStat`]'s `bonus` field.
  pub fn sub_bonus(&mut self, amount: N) {
    self.max.bonus -= amount;
    self.max.total -= amount;
  }

  ///Return the sum of a [`BasicStat`]'s `base` and `bonus` fields.
  pub fn max(&self) -> N {
    self.max.total
  }
}
