extern crate rand_distr;
use num::{Float, One, Zero};
use std::ops::{Add, Div, Index, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Vec3<T = f32>(pub T, pub T, pub T);

macro_rules! def_op {
  ($name: ident, $fn_name: ident, $op: tt) => {
    impl<T>$name for Vec3<T> where T: $name {
      type Output = Vec3<<T as $name>::Output>;
      fn $fn_name(self, o: Self) -> Self::Output {
        Vec3(self.0 $op o.0, self.1 $op o.1, self.2 $op o.2)
      }
    }
  };
}

macro_rules! def_scalar_op {
  ($name: ident, $fn_name: ident, $op: tt) => {
    impl<T>$name<T> for Vec3<T> where T: $name + Copy {
      type Output = Vec3<<T as $name>::Output>;
      fn $fn_name(self, o: T) -> Self::Output {
        Vec3(self.0 $op o, self.1 $op o, self.2 $op o)
      }
    }
  };
}

// Define all vector-vector operations
def_op!(Add, add, +);
def_op!(Mul, mul, *);
def_op!(Sub, sub, -);
def_op!(Div, div, /);
// Define all scalar-vector operations
def_scalar_op!(Add, add, +);
def_scalar_op!(Mul, mul, *);
def_scalar_op!(Sub, sub, -);
def_scalar_op!(Div, div, /);
impl<T> Index<usize> for Vec3<T> {
  type Output = T;
  fn index(&self, idx: usize) -> &Self::Output { self.nth(idx as u8) }
}

impl<T> Vec3<T> {
  pub fn iter(&self) -> Iter<'_, T> { Iter { vec: &self, nth: 0 } }
  pub fn nth(&self, n: u8) -> &T {
    assert!(n < 3);
    match n {
      0 => &self.0,
      1 => &self.1,
      2 => &self.2,
      _ => unreachable!(),
    }
  }
  /// Selects from a if sel is true else returns from b
  #[inline]
  pub fn choose(a: Vec3<T>, b: Vec3<T>, sel: Vec3<bool>) -> Self {
    Vec3(
      if sel.0 { a.0 } else { b.0 },
      if sel.1 { a.1 } else { b.1 },
      if sel.2 { a.2 } else { b.2 },
    )
  }
  /// Swaps elements in a and b according to select and returns the swapped set.
  #[inline]
  pub fn exchange(mut a: Vec3<T>, mut b: Vec3<T>, sel: Vec3<bool>) -> [Self; 2] {
    use std::mem::swap;
    if sel.0 {
      swap(&mut a.0, &mut b.0)
    }
    if sel.1 {
      swap(&mut a.1, &mut b.1)
    }
    if sel.2 {
      swap(&mut a.2, &mut b.2)
    }
    [a, b]
  }
}

impl<T: Neg> Neg for Vec3<T> {
  type Output = Vec3<<T as Neg>::Output>;
  fn neg(self) -> Self::Output { Vec3(-self.0, -self.1, -self.2) }
}

impl<T: Zero> Zero for Vec3<T> {
  fn zero() -> Self { Vec3(T::zero(), T::zero(), T::zero()) }
  fn is_zero(&self) -> bool { self.0.is_zero() && self.1.is_zero() && self.2.is_zero() }
}

impl<T: One + PartialEq> One for Vec3<T> {
  fn one() -> Self { Vec3(T::one(), T::one(), T::one()) }
  fn is_one(&self) -> bool { self.0.is_one() && self.1.is_one() && self.2.is_one() }
}

impl<T: Float> Vec3<T> {
  pub fn sqr_magn(&self) -> T { self.0.powi(2) + self.1.powi(2) + self.2.powi(2) }
  pub fn magn(&self) -> T { self.sqr_magn().sqrt() }
  pub fn norm(&self) -> Self { (*self) / self.magn() }
  pub fn dot(&self, o: Self) -> T { self.0 * o.0 + self.1 * o.1 + self.2 * o.2 }
  pub fn sqrt(&self) -> Self { Vec3(self.0.sqrt(), self.1.sqrt(), self.2.sqrt()) }
  pub fn cross(&self, o: Self) -> Self {
    Vec3(
      self.1 * o.2 - self.2 * o.1,
      self.2 * o.0 - self.0 * o.2,
      self.0 * o.1 - self.1 * o.0,
    )
  }
  pub fn reflect(self, across: Vec3<T>) -> Self {
    self - across * self.dot(across) * T::from(2.0).unwrap()
  }
  pub fn refract(self, norm: Vec3<T>, refract_ratio: T) -> Option<Vec3<T>> {
    let u = self.norm();
    let dt = u.dot(norm);
    Some(T::one() - refract_ratio.powi(2) * (T::one() - dt.powi(2)))
      .filter(|discrim| discrim.is_sign_positive())
      .map(|d| (u - norm * dt) * refract_ratio - norm * d.sqrt())
  }
  pub fn lerp(u: T, min: Vec3<T>, max: Vec3<T>) -> Vec3<T> { min * u + max * (T::one() - u) }
  pub fn max_parts(&self, o: &Self) -> Vec3<T> {
    Vec3(self.0.max(o.0), self.1.max(o.1), self.2.max(o.2))
  }
  pub fn min_parts(&self, o: &Self) -> Vec3<T> {
    Vec3(self.0.min(o.0), self.1.min(o.1), self.2.min(o.2))
  }
}

impl<T: Clone> From<T> for Vec3<T> {
  #[inline]
  fn from(t: T) -> Self { Vec3(t.clone(), t.clone(), t) }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct Ray<T = f32> {
  pub pos: Vec3<T>,
  pub dir: Vec3<T>,
}

impl<T> Ray<T> {
  pub fn new(pos: Vec3<T>, dir: Vec3<T>) -> Self { Ray { pos, dir } }
}
impl<T: Float> Ray<T> {
  pub fn at(&self, t: T) -> Vec3<T> { self.pos + self.dir * t }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Iter<'a, T> {
  vec: &'a Vec3<T>,
  nth: u8,
}

impl<'a, T> Iterator for Iter<'a, T> {
  type Item = &'a T;
  fn next(&mut self) -> Option<Self::Item> {
    let out = match self.nth {
      0 => &self.vec.0,
      1 => &self.vec.1,
      2 => &self.vec.2,
      _ => return None,
    };
    self.nth += 1;
    Some(out)
  }
}

#[cfg(test)]
mod test {
  use super::Vec3;
  #[test]
  fn test_iter() {
    let v = Vec3(0i32, 1, 2);
    assert_eq!(v.iter().copied().collect::<Vec<_>>(), vec![0, 1, 2]);
  }
}
