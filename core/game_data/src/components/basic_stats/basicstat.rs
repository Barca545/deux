use math::{max, number::NumberAssign};
use std::fmt::Debug;

#[derive(Debug, Clone, Copy,)]
/// # Purpose
/// A wrapper for basic stats.
/// - `N` denotes the type of the number the stat holds.
///
/// # Notes
/// Used to emulate a stat with a "base" value and "bonus" value. Tracks changes
/// to the base and max valuese.
pub struct BasicStat<N,> {
  /// Value of the stat gained from leveling up.
  base: N,
  /// Value of the stat gained from items and buffs.
  bonus: N,
  /// Total value of the stat (`max = base + bonus`).
  max: N,
  /// Remaining value of the stat (`remaining = max - n`).
  remaining: N,
  // /// Marker indicating what type of Data the
  // ty:PhantomData<T,>,
}

impl<N,> BasicStat<N,>
where
  N: NumberAssign,
{
  /// Creates a new instance of a [`BasicStat`].
  pub fn new(base: N,) -> Self {
    BasicStat {
      base,
      bonus: N::ZERO,
      max: base,
      remaining: base,
      // ty: PhantomData,
    }
  }

  /// Add a number to a [`BasicStat`]'s [`base`](BasicStat::base) field.
  #[inline(always)]
  pub fn add_base(&mut self, amount: N,) {
    self.base += amount;
    self.max += amount;
  }

  /// Subtract a number from a [`BasicStat`]'s [`base`](BasicStat::base) field.
  #[inline(always)]
  pub fn sub_base(&mut self, amount: N,) {
    // Use max to ensure this never dips below 0
    // Assuming N is an integer or a float, N::default() = 0
    self.base = max(N::ZERO, self.base - amount,);
    self.max = max(N::ZERO, self.max - amount,);
  }

  /// Add a number to a [`BasicStat`]'s [`bonus`](BasicStat::bonus) field.
  #[inline(always)]
  pub fn add_bonus(&mut self, amount: N,) {
    self.bonus += amount;
    self.max += amount;
  }

  /// Subtract a number from a [`BasicStat`]'s [`bonus`](BasicStat::bonus)
  /// field.
  #[inline(always)]
  pub fn sub_bonus(&mut self, amount: N,) {
    // Use max to ensure this never dips below 0
    // Assuming N is an integer or a float, N::default() = 0
    self.bonus = max(N::ZERO, self.bonus - amount,);
    self.max = max(N::ZERO, self.max - amount,);
  }

  /// Add the amount from the [`remaining`](BasicStat::remaining) field of the
  /// [`BasicStat`].
  #[inline(always)]
  pub fn add_remaining(&mut self, amount: N,) {
    self.remaining += amount;
  }

  /// Remove the amount from the [`remaining`](BasicStat::remaining) field of
  /// the [`BasicStat`].
  #[inline(always)]
  pub fn sub_remaining(&mut self, amount: N,) {
    self.remaining = max(N::ZERO, self.remaining - amount,)
  }

  /// Return a [`BasicStat`]'s [`base`](BasicStat::base) field.
  #[inline(always)]
  pub fn base(&self,) -> N {
    self.max
  }

  /// Return a [`BasicStat`]'s [`bonus`](BasicStat::bonus) field.
  #[inline(always)]
  pub fn bonus(&self,) -> N {
    self.bonus
  }

  /// Return the sum of a [`BasicStat`]'s [`max`](BasicStat::base) and
  /// [`bonus`](BasicStat::bonus) fields.
  #[inline(always)]
  pub fn max(&self,) -> N {
    self.max
  }

  /// Return the remaning amount of a [`BasicStat`].
  #[inline(always)]
  pub fn remaining(&self,) -> N {
    self.remaining
  }

  /// Set a [`BasicStat`]'s [`remaining`](BasicStat::remaining) field equal to
  /// its [`max`](BasicStat::max) value.
  #[inline(always)]
  pub fn reset(&mut self,) {
    self.remaining = self.max;
  }

  /// Return a `bool` indicating whether the [`remaining`](BasicStat::remaining)
  /// field is zero.
  #[inline(always)]
  pub fn is_zero(&self,) -> bool {
    self.remaining.is_zero()
  }
}

#[macro_export]
/// Create a [NewType](https://doc.rust-lang.org/rust-by-example/generics/new_types.html) wrapping a [`BasicStat`]. Implements all of `BasicStat`'s methods.
macro_rules! new_basic_stat {
  ($ident:ident,$ty:ty,) => {
    #[derive(Debug, Clone, Copy,)]
    pub struct $ident(BasicStat<$ty,>,);

    impl $ident {
      #[allow(unused)]
      pub fn new(base: $ty,) -> Self {
        $ident(BasicStat::new(base,),)
      }
      #[allow(unused)]
      /// Add `amount` to the `base` field.
      pub fn add_base(&mut self, amount: $ty,) {
        self.0.add_base(amount,);
      }

      #[allow(unused)]
      /// Remove `amount` from the `base` field.
      pub fn sub_base(&mut self, amount: $ty,) {
        self.0.sub_base(amount,);
      }

      #[allow(unused)]
      /// Add `amount` to the `bonus` field.
      pub fn add_bonus(&mut self, amount: $ty,) {
        self.0.add_bonus(amount,);
      }

      #[allow(unused)]
      /// Remove `amount` from the `bonus` field.
      pub fn sub_bonus(&mut self, amount: $ty,) {
        self.0.sub_bonus(amount,);
      }

      #[allow(unused)]
      /// Add `amount` to the `remaining` field.
      pub fn add_remaining(&mut self, amount: $ty,) {
        self.0.add_remaining(amount,);
      }

      #[allow(unused)]
      /// Remove `amount` from the `remaining` field.
      pub fn sub_remaining(&mut self, amount: $ty,) {
        self.0.sub_remaining(amount,);
      }

      #[allow(unused)]
      /// Return the value of the `base ` field.
      pub fn base(&self,) -> $ty {
        self.0.base()
      }

      #[allow(unused)]
      /// Return the value of the `bonus` field.
      pub fn bonus(&self,) -> $ty {
        self.0.bonus()
      }

      #[allow(unused)]
      /// Return the value of the `max` field.
      pub fn max(&self,) -> $ty {
        self.0.max()
      }

      #[allow(unused)]
      /// Value of the `remaining` field.
      pub fn remaining(&self,) -> $ty {
        self.0.remaining()
      }

      #[allow(unused)]
      /// Set the `remaining` field equal to
      /// the `max` value.
      pub fn reset(&mut self,) {
        self.0.reset()
      }

      #[allow(unused)]
      /// Return a `bool` indicating whether the `remaining`
      /// field is zero.
      pub fn is_zero(&self,) -> bool {
        self.0.is_zero()
      }
    }
  };
}
