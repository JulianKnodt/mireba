// TODO might need to convert this into a different enum in the case of d == 0?
#[inline]
pub fn quad_solve<T>(a: T, b: T, c: T) -> Option<(T, T)>
where
  T: num::Float, {
  Some(b * b - a * c * T::from(4.0).unwrap())
    .filter(|discrim| discrim.is_sign_positive())
    .map(|discrim| discrim.sqrt())
    .map(|d| {
      let denom = T::from(2.0).unwrap() * a;
      ((-b + d) / denom, (-b - d) / denom)
    })
}
