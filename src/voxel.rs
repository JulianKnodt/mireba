
// Non-uniform partition of space based on curvature of shape
#[derive(PartialEq, Debug)]
pub struct VoxelGrid<T> {
  octree: Octree<T>,
  resolution: f32,
}
