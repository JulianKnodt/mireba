use num::{Float, One, Zero};
use serde::{Deserialize, Serialize};
use std::{
  marker::PhantomData,
  ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Rem, Sub, SubAssign,
  },
};

/// Abstract vector over some finite field
pub trait Vector: Div<<Self as Vector>::Field, Output = Self> + Copy {
  type Field: Float;
  /// Inner product of vector with o
  fn dot(&self, o: &Self) -> Self::Field;
  /// Construct self from an array of items, if it is less than the dimension of the vector
  /// The rest will be zero filled.
  fn cons<const N: usize>(vs: [Self::Field; N]) -> Self;
  /// Returns the square magnitude of this vector
  fn sqr_magn(&self) -> Self::Field { self.dot(self) }
  /// Returns the magnitude of this vector
  fn magn(&self) -> Self::Field { self.sqr_magn().sqrt() }
  /// Returns a unit vector in the same direction as this one
  fn norm(&self) -> Self { *self / self.magn() }
  /// The angle between two vectors as defined by the shortest arc between them.
  /// Note this is unsigned in the range [0, PI].
  fn angle(&self, o: &Self) -> Self::Field { self.cos_angle(&o).acos() }
  /// Returns the cosine of the angle between two vectors.
  /// Convenient if the cosine is needed
  fn cos_angle(&self, o: &Self) -> Self::Field { self.dot(o) / (self.magn() * o.magn()) }
  /// The ratio that self must be scaled by in order to by the same magnitude as the dest.
  fn scale_ratio(&self, dst: &Self) -> Self::Field { (dst.sqr_magn() / self.sqr_magn()).sqrt() }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Vec3<T>(pub T, pub T, pub T);

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
  fn set(&mut self, n: u8, v: T) {
    assert!(n < 3);
    let dst = match n {
      0 => &mut self.0,
      1 => &mut self.1,
      2 => &mut self.2,
      _ => unreachable!(),
    };
    *dst = v;
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
  pub fn extend(self, w: T) -> Vec4<T> {
    let Vec3(i, j, k) = self;
    Vec4([i, j, k, w])
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

impl<T: Float> Vector for Vec3<T> {
  type Field = T;
  fn dot(&self, o: &Self) -> T {
    let &Vec3(i, j, k) = self;
    let &Vec3(x, y, z) = o;
    i * x + j * y + k * z
  }
  fn cons<const N: usize>(vs: [T; N]) -> Self {
    let o = T::zero();
    match &vs[..] {
      [] => Vec3::zero(),
      &[a] => Vec3(a, o, o),
      &[a, b] => Vec3(a, b, o),
      &[a, b, c, ..] => Vec3(a, b, c),
    }
  }
}

impl<T: Float> Vec3<T> {
  pub fn sqrt(&self) -> Self { Vec3(self.0.sqrt(), self.1.sqrt(), self.2.sqrt()) }
  pub fn sqr_dist(&self, o: &Self) -> T { (*self - *o).sqr_magn() }
  pub fn dist(&self, o: &Self) -> T { (*self - *o).magn() }
  pub fn floor(&self) -> Self { Vec3(self.0.floor(), self.1.floor(), self.2.floor()) }
  pub fn cross(&self, o: &Self) -> Self {
    let &Vec3(a0, a1, a2) = self;
    let &Vec3(o0, o1, o2) = o;
    Vec3(a1 * o2 - a2 * o1, a2 * o0 - a0 * o2, a0 * o1 - a1 * o0)
  }
  pub fn reflect(self, across: &Self) -> Self {
    self - *across * self.dot(&across) * T::from(2.0).unwrap()
  }
  pub fn refract(self, norm: Vec3<T>, eta: T) -> Option<Vec3<T>> {
    let cos_l = self.dot(&norm);
    let discrim = T::one() - eta * eta * (T::one() - cos_l * cos_l);
    if discrim.is_sign_negative() {
      return None;
    }
    let cos_r = discrim.sqrt();
    Some(self * eta - norm * (eta * cos_l + cos_r))
  }
  pub fn lerp(u: T, min: Vec3<T>, max: Vec3<T>) -> Vec3<T> { min * u + max * (T::one() - u) }
  pub fn max_parts(&self, o: &Self) -> Vec3<T> {
    Vec3(self.0.max(o.0), self.1.max(o.1), self.2.max(o.2))
  }
  pub fn min_parts(&self, o: &Self) -> Vec3<T> {
    Vec3(self.0.min(o.0), self.1.min(o.1), self.2.min(o.2))
  }
  pub fn to_f32(&self) -> Vec3<f32> { self.apply_fn(|v| num::NumCast::from(v).unwrap()) }
  pub fn clamp(&mut self, min: T, max: T) {
    self.0 = self.0.max(min).min(max);
    self.1 = self.1.max(min).min(max);
    self.2 = self.2.max(min).min(max);
  }
  pub fn apply_fn<F, S>(&self, f: F) -> Vec3<S>
  where
    F: Fn(T) -> S, {
    Vec3(f(self.0), f(self.1), f(self.2))
  }
  // TODO possibly optimization listed here
  // https://en.wikipedia.org/wiki/Conversion_between_quaternions_and_Euler_angles
  pub fn apply_quat(&self, q: &Quat<T>) -> Self {
    // manually compute the multiplication here because it's more efficient
    let p: Quat<T> = (*self).into();
    ((*q * p) * q.conj()).0
  }
  pub fn from_str_radix(strs: (&str, &str, &str), radix: u32) -> Result<Self, T::FromStrRadixErr> {
    let (x, y, z) = strs;
    Ok(Vec3(
      T::from_str_radix(x, radix)?,
      T::from_str_radix(y, radix)?,
      T::from_str_radix(z, radix)?,
    ))
  }
  /// All vectors 'o' along one side of self return true, otherwise false,
  /// assuming that self and o fall in the same plane defined by normal. Otherwise behaviour is
  /// undefined. Which side it returns should not be relied on.
  pub fn sided(&self, o: &Self, normal: &Self) -> bool {
    self.cross(o).dot(normal).is_sign_positive()
  }
}

impl<T: One + Zero> Vec3<T> {
  pub fn basis(n: u8) -> Self {
    let mut out = Vec3::zero();
    out.set(n, T::one());
    out
  }
}

impl<T: Clone> From<T> for Vec3<T> {
  #[inline]
  fn from(t: T) -> Self { Vec3(t.clone(), t.clone(), t) }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Ray<T, V = Vec3<T>> {
  pub pos: V,
  pub dir: V,
  phantom: PhantomData<T>,
}

impl<T, V> Ray<T, V> {
  /// Returns a new ray with the given position and direction
  pub fn new(pos: V, dir: V) -> Self {
    Ray {
      pos,
      dir,
      phantom: PhantomData,
    }
  }
}
impl<T: Float, V> Ray<T, V>
where
  V: Add<Output = V> + Mul<T, Output = V> + Copy,
{
  /// Returns the position along a ray that corresponds to some parameter T
  pub fn at(&self, t: T) -> V { self.pos + self.dir * t }
  pub fn step(&mut self, t: T) { self.pos = self.pos + (self.dir * t) }
}

impl<T: Float> Ray<T, Vec2<T>> {
  /// Sets the length of this ray to the given amount
  pub fn set_length(&mut self, t: T) { self.dir = self.dir.norm() * t; }
}

impl<T: Float, V> Ray<T, V>
where
  V: Neg<Output = V> + Copy,
{
  /// Flips the direction of this ray
  pub fn flip(&self) -> Self { Ray::new(self.pos, -self.dir) }
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

#[derive(PartialEq, Eq, Ord, PartialOrd, Clone, Copy, Debug, Hash)]
pub struct Vec2<T = f32>(pub T, pub T);

impl<T: Zero> Zero for Vec2<T> {
  fn zero() -> Self { Vec2(T::zero(), T::zero()) }
  fn is_zero(&self) -> bool { self.0.is_zero() && self.1.is_zero() }
}

impl<T: One + Eq> One for Vec2<T> {
  fn one() -> Self { Vec2(T::one(), T::one()) }
  fn is_one(&self) -> bool { self.0.is_one() && self.1.is_one() }
}

impl<T> Vec2<T> {
  pub fn flip(self) -> Self { Vec2(self.1, self.0) }
}

impl<T: Float> Vector for Vec2<T> {
  type Field = T;
  fn dot(&self, o: &Self) -> T {
    let &Vec2(i, j) = self;
    let &Vec2(x, y) = o;
    i * x + j * y
  }
  fn cons<const N: usize>(vs: [T; N]) -> Self {
    let o = T::zero();
    match &vs[..] {
      [] => Vec2::zero(),
      &[a] => Vec2(a, o),
      &[a, b, ..] => Vec2(a, b),
    }
  }
}

impl<T: Float> Vec2<T> {
  /// Returns an angle in the range [0, 2*PI] between self and dst
  pub fn signed_angle(&self, dst: &Self) -> T {
    let &Vec2(i, j) = self;
    let &Vec2(x, y) = dst;
    (i * y - j * x).atan2(self.dot(&dst))
  }
}

/// Homogenous vector
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Vec4<T>(pub [T; 4]);

impl<T: Zero> Zero for Vec4<T> {
  fn zero() -> Self { Vec4([T::zero(), T::zero(), T::zero(), T::zero()]) }
  fn is_zero(&self) -> bool { self.0.iter().all(|v| v.is_zero()) }
}

impl<T: One + Eq> One for Vec4<T> {
  fn one() -> Self { Vec4([T::one(), T::one(), T::one(), T::one()]) }
  fn is_one(&self) -> bool { self.0.iter().all(|v| v.is_one()) }
}

impl<T: One> From<Vec3<T>> for Vec4<T> {
  fn from(v: Vec3<T>) -> Self {
    let Vec3(a, b, c) = v;
    Vec4([a, b, c, T::one()])
  }
}

impl<T: Float> From<Vec4<T>> for Vec3<T> {
  fn from(v: Vec4<T>) -> Self {
    let Vec4([x, y, z, w]) = v;
    Vec3(x, y, z) / w
  }
}

impl<T: Float> Vector for Vec4<T> {
  type Field = T;
  fn dot(&self, o: &Self) -> T {
    let &Vec4([x, y, z, w]) = self;
    let &Vec4([i, j, k, l]) = o;
    x * i + y * j + z * k + w * l
  }
  fn cons<const N: usize>(vs: [T; N]) -> Self {
    let o = T::zero();
    match &vs[..] {
      [] => Vec4::zero(),
      &[a] => Vec4([a, o, o, o]),
      &[a, b] => Vec4([a, b, o, o]),
      &[a, b, c] => Vec4([a, b, c, o]),
      &[a, b, c, d, ..] => Vec4([a, b, c, d]),
    }
  }
}

impl<T: One + Zero> Vec4<T> {
  /// returns a basis vector for this
  pub fn basis(v: u8) -> Self {
    assert!(v < 4);
    let mut out = Vec4::zero();
    out.0[v as usize] = T::one();
    out
  }
}

impl<T> Index<usize> for Vec4<T> {
  type Output = T;
  fn index(&self, i: usize) -> &Self::Output { &self.0[i] }
}

impl<T> IndexMut<usize> for Vec4<T> {
  fn index_mut(&mut self, i: usize) -> &mut Self::Output { &mut self.0[i] }
}

impl<T: Copy> Vec4<T> {
  pub fn apply_fn<F, S>(&self, f: F) -> Vec4<S>
  where
    F: Fn(T) -> S, {
    let &Vec4([a, b, c, d]) = self;
    Vec4([f(a), f(b), f(c), f(d)])
  }
}

macro_rules! def_op {
  ($name: ident, $fn_name: ident, $op: tt) => {
    impl<T>$name for Vec3<T> where T: $name {
      type Output = Vec3<<T as $name>::Output>;
      fn $fn_name(self, o: Self) -> Self::Output {
        Vec3(self.0 $op o.0, self.1 $op o.1, self.2 $op o.2)
      }
    }
    impl<T>$name for Vec2<T> where T: $name {
      type Output = Vec2<<T as $name>::Output>;
      fn $fn_name(self, o: Self) -> Self::Output {
        Vec2(self.0 $op o.0, self.1 $op o.1)
      }
    }
    impl<T>$name for Vec4<T> where T: $name {
      type Output = Vec4<<T as $name>::Output>;
      fn $fn_name(self, o: Self) -> Self::Output {
        let Vec4([i,j,k,l]) = self;
        let Vec4([x,y,z,w]) = o;
        Vec4([i $op x, j $op y, k $op z, l $op w])
      }
    }
    impl<T: $name + Copy>$name for &Vec3<T> {
      type Output = Vec3<<T as $name>::Output>;
      fn $fn_name(self, o: Self) -> Self::Output {
        Vec3(self.0 $op o.0, self.1 $op o.1, self.2 $op o.2)
      }
    }
  };
}

macro_rules! def_scalar_op {
  ($name: ident, $fn_name: ident, $op: tt) => {
    impl<T>$name<T> for Vec2<T> where T: $name + Copy {
      type Output = Vec2<<T as $name>::Output>;
      fn $fn_name(self, o: T) -> Self::Output {
        Vec2(self.0 $op o, self.1 $op o)
      }
    }
    impl<T>$name<T> for Vec3<T> where T: $name + Copy {
      type Output = Vec3<<T as $name>::Output>;
      fn $fn_name(self, o: T) -> Self::Output {
        Vec3(self.0 $op o, self.1 $op o, self.2 $op o)
      }
    }
    impl<T>$name<T> for Vec4<T> where T: $name + Copy {
      type Output = Vec4<<T as $name>::Output>;
      fn $fn_name(self, o: T) -> Self::Output {
        let Vec4([x,y,z,w]) = self;
        Vec4([x $op o, y $op o, z $op o, w $op o])
      }
    }
  };
}

macro_rules! def_assign_op {
  ($name: ident, $fn_name: ident, $op: tt) => {
    impl<T>$name for Vec3<T> where T: $name {
      fn $fn_name(&mut self, o: Self) {
        self.0 $op o.0;
        self.1 $op o.1;
        self.2 $op o.2;
      }
    }
    impl<T>$name for Vec2<T> where T: $name {
      fn $fn_name(&mut self, o: Self) {
        self.0 $op o.0;
        self.1 $op o.1;
      }
    }
  };
}

// Define all vector-vector operations
def_op!(Add, add, +);
def_op!(Mul, mul, *);
def_op!(Sub, sub, -);
def_op!(Div, div, /);
def_op!(Rem, rem, %);
// Define all scalar-vector operations
def_scalar_op!(Add, add, +);
def_scalar_op!(Mul, mul, *);
def_scalar_op!(Sub, sub, -);
def_scalar_op!(Div, div, /);
def_scalar_op!(Rem, rem, %);
// Define all assign vector-vector operations
def_assign_op!(AddAssign, add_assign, +=);
def_assign_op!(SubAssign, sub_assign, -=);
def_assign_op!(MulAssign, mul_assign, *=);
def_assign_op!(DivAssign, div_assign, /=);

#[cfg(test)]
mod test {
  use super::Vec3;
  #[test]
  fn test_iter() {
    let v = Vec3(0i32, 1, 2);
    assert_eq!(v.iter().copied().collect::<Vec<_>>(), vec![0, 1, 2]);
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Quat<T>(pub Vec3<T>, pub T);

impl<T: Float> Quat<T> {
  pub fn new(x: T, y: T, z: T, w: T) -> Self { Quat(Vec3(x, y, z), w) }
  pub fn conj(&self) -> Self { Self(-self.0, self.1) }
  pub fn sqr_magn(&self) -> T { self.0.sqr_magn() + self.1 * self.1 }
  pub fn magn(&self) -> T { self.sqr_magn().sqrt() }
  pub fn is_unit(&self) -> bool { self.sqr_magn().is_one() }
  pub fn norm(&self) -> Self { *self / self.magn() }
  /// Encodes a scaling factor into the quaternion
  pub fn scale(&self, factor: T) -> Self { *self * factor.sqrt() }
  /// Returns a quaternion which is a rotation in the 3 dimensions given
  pub fn rot(along: &Vec3<T>) -> Self {
    let two = T::from(2.0).unwrap();
    let Vec3(cx, cy, cz) = along.apply_fn(|v| T::cos(v / two));
    let Vec3(sx, sy, sz) = along.apply_fn(|v| T::sin(v / two));
    Quat::new(
      sx * cy * cz - cx * sy * sz,
      cx * sy * cz + sx * cy * sz,
      cx * cy * sz - sx * sy * cz,
      cx * cy * cz + sx * sy * sz,
    )
  }
  // TODO investigate slerping
}

impl<T: Neg> Neg for Quat<T> {
  type Output = Quat<<T as Neg>::Output>;
  fn neg(self) -> Self::Output { Quat(-self.0, -self.1) }
}

impl<T: Zero> Zero for Quat<T> {
  fn zero() -> Self { Quat(Vec3::zero(), T::zero()) }
  fn is_zero(&self) -> bool { self.0.is_zero() && self.1.is_zero() }
}

impl<T: Zero + One + PartialEq + Float> One for Quat<T> {
  fn one() -> Self { Quat(Vec3::zero(), T::one()) }
  fn is_one(&self) -> bool { self.0.is_zero() && self.1.is_one() }
}

impl<T: Float> Mul for Quat<T> {
  type Output = Quat<T>;
  fn mul(self, o: Self) -> Self {
    let Quat(Vec3(x, y, z), w) = self;
    let Quat(Vec3(i, j, k), l) = o;
    Quat::new(
      w * i + l * x + y * k - z * j,
      w * j + l * y + z * i - k * x,
      w * k + l * z + j * x - i * y,
      w * l - x * i - y * j - z * k,
    )
  }
}

impl<T: Zero> From<Vec3<T>> for Quat<T> {
  fn from(v: Vec3<T>) -> Self { Quat(v, T::zero()) }
}

macro_rules! def_quat_op {
  ($name: ident, $fn_name: ident, $op: tt) => {
    impl<T>$name for Quat<T> where T: $name {
      type Output = Quat<<T as $name>::Output>;
      fn $fn_name(self, o: Self) -> Self::Output {
        Quat(self.0 $op o.0, self.1 $op o.1)
      }
    }
    impl<T: $name + Copy>$name for &Quat<T> {
      type Output = Quat<<T as $name>::Output>;
      fn $fn_name(self, o: Self) -> Self::Output {
        Quat(self.0 $op o.0, self.1 $op o.1)
      }
    }
  };
}

macro_rules! def_scalar_quat_op {
  ($name: ident, $fn_name: ident, $op: tt) => {
    impl<T>$name<T> for Quat<T> where T: $name + Copy {
      type Output = Quat<<T as $name>::Output>;
      fn $fn_name(self, o: T) -> Self::Output {
        Quat(self.0 $op o, self.1 $op o)
      }
    }
  };
}

macro_rules! def_assign_quat_op {
  ($name: ident, $fn_name: ident, $op: tt) => {
    impl<T>$name for Quat<T> where T: $name {
      fn $fn_name(&mut self, o: Self) {
        self.0 $op o.0;
        self.1 $op o.1;
      }
    }
  };
}

// Define all quat-quat operations
def_quat_op!(Add, add, +);
def_quat_op!(Sub, sub, -);
// Define all scalar-quat operations
def_scalar_quat_op!(Add, add, +);
def_scalar_quat_op!(Mul, mul, *);
def_scalar_quat_op!(Sub, sub, -);
def_scalar_quat_op!(Div, div, /);
def_scalar_quat_op!(Rem, rem, %);
// Define all assign quat-quat operations
def_assign_quat_op!(AddAssign, add_assign, +=);
def_assign_quat_op!(SubAssign, sub_assign, -=);
def_assign_quat_op!(MulAssign, mul_assign, *=);

use rand::{
  distributions::{Distribution, Standard},
  Rng,
};
impl<T> Distribution<Vec3<T>> for Standard
where
  Standard: Distribution<T>,
{
  fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec3<T> {
    Vec3(rng.gen(), rng.gen(), rng.gen())
  }
}

impl<T> Distribution<Vec2<T>> for Standard
where
  Standard: Distribution<T>,
{
  fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec2<T> { Vec2(rng.gen(), rng.gen()) }
}
