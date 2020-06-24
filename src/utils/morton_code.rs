// https://fgiesen.wordpress.com/2009/12/13/decoding-morton-codes/
pub const fn compact_1_by_1(mut v: u32) -> u32 {
  v &= 0x55555555; // v = -f-e -d-c -b-a -9-8 -7-6 -5-4 -3-2 -1-0
  v = (v ^ (v >> 1)) & 0x33333333; // v = --fe --dc --ba --98 --76 --54 --32 --10
  v = (v ^ (v >> 2)) & 0x0f0f0f0f; // v = ---- fedc ---- ba98 ---- 7654 ---- 3210
  v = (v ^ (v >> 4)) & 0x00ff00ff; // v = ---- ---- fedc ba98 ---- ---- 7654 3210
  (v ^ (v >> 8)) & 0x0000ffff // v = ---- ---- ---- ---- fedc ba98 7654 3210
}

pub const fn morton_decode(v: u32) -> (u32, u32) { (compact_1_by_1(v), compact_1_by_1(v >> 1)) }

pub const fn part_1_by_1(mut x: u32) -> u32 {
  x &= 0x0000ffff; // x = ---- ---- ---- ---- fedc ba98 7654 3210
  x = (x ^ (x << 8)) & 0x00ff00ff; // x = ---- ---- fedc ba98 ---- ---- 7654 3210
  x = (x ^ (x << 4)) & 0x0f0f0f0f; // x = ---- fedc ---- ba98 ---- 7654 ---- 3210
  x = (x ^ (x << 2)) & 0x33333333; // x = --fe --dc --ba --98 --76 --54 --32 --10
  (x ^ (x << 1)) & 0x55555555 // x = -f-e -d-c -b-a -9-8 -7-6 -5-4 -3-2 -1-0
}

pub const fn morton_encode(x: u32, y: u32) -> u32 { (part_1_by_1(y) << 1) + part_1_by_1(x) }

#[test]
fn test_morton_coding() {
  let n = 100;
  let (x, y) = morton_decode(n);
  assert_eq!(n, morton_encode(x, y));
}
