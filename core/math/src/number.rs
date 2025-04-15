use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

/// Trait indicating the implementor has a "zero" value.
pub trait Zero: Sized {
  /// The zero value of the implementor.
  const ZERO: Self;

  /// Returns a `bool` indicating whether `Self` is zero.
  fn is_zero(&self,) -> bool;

  /// Set `self` equal to its "zero" value.
  #[inline(always)]
  fn set_zero(&mut self,) {
    *self = Self::ZERO;
  }
}

macro_rules! impl_zero {
  ($num_ty:ty, $val:expr) => {
    impl Zero for $num_ty {
      const ZERO: $num_ty = $val;

      #[inline(always)]
      fn is_zero(&self,) -> bool {
        self == &Self::ZERO
      }
    }
  };
}

impl_zero!(u8, 0);
impl_zero!(u16, 0);
impl_zero!(u32, 0);
impl_zero!(u64, 0);
impl_zero!(usize, 0);
impl_zero!(i8, 0);
impl_zero!(i16, 0);
impl_zero!(i32, 0);
impl_zero!(i64, 0);
impl_zero!(isize, 0);
impl_zero!(f32, 0.0);
impl_zero!(f64, 0.0);

pub trait NumberOperations<Rhs = Self, Output = Self,>:
  Mul<Rhs, Output = Output,>
  + Add<Rhs, Output = Output,>
  + Div<Rhs, Output = Output,>
  + Sub<Rhs, Output = Output,>
{
}

impl<N,> NumberOperations<N, N,> for N where
  N: Mul<N, Output = N,> + Add<N, Output = N,> + Div<N, Output = N,> + Sub<N, Output = N,>
{
}

/// Marker trait gauranteeing `N` will behave like a number
pub trait Number: Sized + PartialOrd + Zero + NumberOperations + Clone + Copy {}

pub trait NumberAssign: Number + AddAssign + SubAssign + MulAssign + DivAssign {}

// Blanket implementions of the number trait for all numerical types
impl<N,> Number for N where N: Sized + PartialOrd + Zero + NumberOperations + Clone + Copy {}

// Blanket implementions of the numberassign trait for all numerical types
impl<N,> NumberAssign for N where N: Number + AddAssign + SubAssign + MulAssign + DivAssign {}
