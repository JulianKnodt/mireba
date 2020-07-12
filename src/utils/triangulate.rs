use quick_maths::Vec3;

/// Triangulates a face by taking the first vertex as the pivot and making triangles between
/// adjacent pairs of vertices
pub fn triangulate<T: Copy, I: IntoIterator<Item = T>>(v: I) -> impl Iterator<Item = Vec3<T>> {
  let mut iter = v.into_iter();
  let first = iter.next().unwrap();
  let second = iter.next().unwrap();
  iter.scan(second, move |prev, n| {
    let face = Vec3::new(first, *prev, n);
    *prev = n;
    Some(face)
  })
}
