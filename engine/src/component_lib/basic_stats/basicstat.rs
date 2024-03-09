use std::{
  fmt::Debug,
  marker::PhantomData,
  ops::{AddAssign, Sub, SubAssign},
};

use crate::math::max;

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
  N: SubAssign + AddAssign + Copy + Default + Sub<Output = N> + PartialOrd,
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
    //Use max to ensure this never dips below 0
    //Assuming N is an integer or a float, N::default() = 0
    self.base = max(N::default(), self.base - amount);
    self.total = max(N::default(), self.total - amount);
  }

  ///Add a number to a [`BasicStat`]'s `bonus` field.
  pub fn add_bonus(&mut self, amount: N) {
    self.bonus += amount;
    self.total += amount;
  }

  ///Subtract a number from a [`BasicStat`]'s `bonus` field.
  pub fn sub_bonus(&mut self, amount: N) {
    //Use max to ensure this never dips below 0
    //Assuming N is an integer or a float, N::default() = 0
    self.bonus = max(N::default(), self.bonus - amount);
    self.total = max(N::default(), self.total - amount);
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
  remaining: N,
}

impl<T, N> BasicGrowableStat<T, N>
where
  N: SubAssign + AddAssign + Copy + Default + Sub<Output = N> + PartialOrd + Debug,
{
  pub fn new(base: N) -> Self {
    let max = BasicStat::new(base);
    BasicGrowableStat { max, remaining: base }
  }

  ///Add a number to a [`BasicStat`]'s `base` field.
  pub fn add_base(&mut self, amount: N) {
    self.max.add_base(amount);
  }

  ///Subtract a number from a [`BasicStat`]'s `base` field.
  pub fn sub_base(&mut self, amount: N) {
    self.max.sub_base(amount);
  }

  ///Add a number to a [`BasicStat`]'s `bonus` field.
  pub fn add_bonus(&mut self, amount: N) {
    self.max.add_bonus(amount);
  }

  ///Subtract a number from a [`BasicStat`]'s `bonus` field.
  pub fn sub_bonus(&mut self, amount: N) {
    self.max.sub_bonus(amount);
  }

  ///Add the amount from the `remaining` field of the [`BasicGrowableStat`].
  pub fn add_remaining(&mut self, amount: N) {
    self.remaining += amount;
  }

  ///Remove the amount from the `remaining` field of the [`BasicGrowableStat`].
  pub fn sub_remaining(&mut self, amount: N) {
    self.remaining = max(N::default(), self.remaining - amount)
  }

  ///Set a [`BasicGrowableStat`]'s `remaining` field equal to its `max` value.
  pub fn reset(&mut self) {
    self.remaining = self.max.total;
  }

  ///Return the sum of a [`BasicStat`]'s `base` and `bonus` fields.
  pub fn max(&self) -> N {
    self.max.total
  }

  ///Return the remaning amount of a [`BasicGrowableStat`].
  pub fn remaining(&self) -> N {
    self.remaining
  }

  ///Return a `bool` indicating whether the `remaining` field of a [`BasicGrowableStat`] is zero.
  pub fn is_zero(&self) -> bool {
    //Assuming N is an integer or a float, N::default() = 0
    self.remaining == N::default()
  }
}
#[cfg(test)]
mod test {
  use crate::math::max;

  #[test]
  fn does_not_subtract_below_zero() {
    let mut initial = 100;
    initial = max(0, initial - 1000);
    assert_eq!(initial, 0);
  }
}
