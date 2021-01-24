use linalg::{
  num::{Float},
  vec::{Ray3, Vec2, Vec3, Vector},
};
use crate::{
  bounds::Bounds,
  transform::Transform,
};
use num::{traits::float::FloatConst, Zero};
use rand::{thread_rng, Rng};
use rand_distr::{Distribution, Standard};

pub trait Cam<T: Float> {
  /// Takes a point on the raster([0, resolution_max])
  /// And returns a ray from the camera to the point on the raster in world space.
  fn ray_to(&self, uv: Vec2<T>) -> Ray3<T>;
}

#[derive(Clone, Debug, PartialEq)]
pub struct Camera<T> {
  // Position of camera
  pos: Vec3<T>,

  // Screen positions
  ll_corner: Vec3<T>,
  /// How long is the screen horizontally?
  hori: Vec3<T>,
  /// How tall is the screen vertically?
  vert: Vec3<T>,
}

pub fn rand_in_unit_disk<T>() -> (T, T)
where
  T: Float + FloatConst,
  Standard: Distribution<T>, {
  let mut rng = thread_rng();
  let r = rng.gen().sqrt();
  let theta = rng.gen() * T::from(2.0).unwrap() * T::PI();
  (r * theta.cos(), r * theta.sin())
}

impl<T> Camera<T>
where
  T: Float,
  Standard: Distribution<T>,
{
  pub fn new(vert_fov_deg: T, aspect_ratio: T) -> Self {
    let two = T::from(2.0).unwrap();

    let theta = vert_fov_deg.to_radians();

    let half_height = (theta / two).tan();
    let half_width = half_height * aspect_ratio;
    Camera {
      ll_corner: Vec3(-half_width, -half_height, -T::one()),
      vert: Vec3(half_width * two, T::zero(), T::zero()),
      hori: Vec3(T::zero(), half_height * two, T::zero()),
      pos: Vec3::zero(),
    }
  }
  pub fn aimed(from: Vec3<T>, at: Vec3<T>, up: Vec3<T>, vert_fov_deg: T, aspect: T) -> Self {
    let theta = vert_fov_deg.to_radians();
    let half_height = (theta / T::from(2.0).unwrap()).tan();
    let half_width = half_height * aspect;
    let w = (from - at).norm();
    let u = up.cross(&w).norm();
    let v = w.cross(&u).norm();
    Camera {
      ll_corner: from - v * half_height - u * half_width - w,
      vert: u * half_width * T::from(2.0).unwrap(),
      hori: v * half_height * T::from(2.0).unwrap(),
      pos: from,
    }
  }
  pub fn center(&self) -> Vec3<T> {
    let two = T::from(2.0).unwrap();
    self.ll_corner + self.hori / two + self.vert / two
  }
  /// Translates this camera
  pub fn translate(&mut self, by: &Vec3<T>) {
    self.pos = self.pos + *by;
    self.ll_corner = self.ll_corner + *by;
  }
  /// Gets a unit vector in the forward direction
  pub fn fwd(&self) -> Vec3<T> { (self.center() - self.pos).norm() }
  pub fn rays(&self, n: u32, xy: Vec2<T>) -> impl Iterator<Item = Ray3<T>> + '_
  where
    Standard: Distribution<T>, {
    let Vec2(x, y) = xy;
    let mut rng = thread_rng();
    (0..n).map(move |_| self.ray_to(Vec2(x + rng.gen(), y + rng.gen())))
  }
}

impl<T: Float> Cam<T> for Camera<T> {
  fn ray_to(&self, uv: Vec2<T>) -> Ray3<T> {
    let Vec2(u, v) = uv;
    Ray3::new(
      self.pos,
      self.ll_corner + self.hori * v + self.vert * u - self.pos,
    )
  }
}

#[derive(Debug)]
pub struct ProjectiveCamera<T> {
  pub cam_to_world: Transform<T>,
  pub cam_to_screen: Transform<T>,
  pub screen_to_raster: Transform<T>,
  pub focal_dist: T,
  pub lens_rad: T,
}

impl<T: Float> ProjectiveCamera<T> {
  pub fn new(
    cam_to_world: Transform<T>,
    cam_to_screen: Transform<T>,
    screen: Bounds<Vec2<T>>,
    resolution: (usize, usize),
    focal_dist: T,
    lens_rad: T,
  ) -> Self {
    let (res_x, res_y) = (
      T::from(resolution.0).unwrap(),
      T::from(resolution.1).unwrap(),
    );
    let &Vec2(hx, hy) = screen.max();
    let &Vec2(lx, ly) = screen.min();
    let l = T::one();

    let screen_to_raster = Transform::scale(Vec3(res_x, res_y, l))
      .compose(&Transform::scale(Vec3(l / (hx - lx), l / (ly - hy), l)))
      .compose(&Transform::translate(Vec3(-lx, -hy, T::zero())));
    Self {
      cam_to_world,
      cam_to_screen,
      screen_to_raster,
      focal_dist,
      lens_rad,
    }
  }
}

#[derive(Debug)]
pub struct OrthographicCamera<T>(ProjectiveCamera<T>);

impl<T: Float> OrthographicCamera<T> {
  pub fn new(
    cam_to_world: Transform<T>,
    screen: Bounds<Vec2<T>>,
    res: (usize, usize),
    focal_dist: T,
    lens_rad: T,
  ) -> Self {
    let orthographic = Transform::orthographic(T::zero(), T::one());
    Self(ProjectiveCamera::new(
      cam_to_world,
      orthographic,
      screen,
      res,
      focal_dist,
      lens_rad,
    ))
  }
  pub fn rays(&self, n: u32, xy: Vec2<T>) -> impl Iterator<Item = Ray3<T>> + '_
  where
    Standard: Distribution<T>, {
    let Vec2(x, y) = xy;
    let mut rng = thread_rng();
    (0..n).map(move |_| self.ray_to(Vec2(x + rng.gen(), y + rng.gen())))
  }
}

impl<T: Float> Cam<T> for OrthographicCamera<T> {
  fn ray_to(&self, uv: Vec2<T>) -> Ray3<T> {
    let pos = uv.extend(T::zero());
    // point in camera space
    let pos = self.0.screen_to_raster.apply_inv_pt(&pos);
    let pos = self.0.cam_to_screen.apply_inv_pt(&pos);
    let pos = self.0.cam_to_world.apply_pt(&pos);
    let dir = self
      .0
      .cam_to_world
      .apply_vec(&Vec3(T::zero(), T::zero(), T::one()))
      .norm();
    Ray3::new(pos, dir)
  }
}

#[derive(Debug)]
pub struct PerspectiveCamera<T>(pub ProjectiveCamera<T>);

impl<T: Float> PerspectiveCamera<T> {
  pub fn new(
    cam_to_world: Transform<T>,
    screen: Bounds<Vec2<T>>,
    res: (usize, usize),
    focal_dist: T,
    lens_rad: T,
    fov: T,
  ) -> Self {
    let e = T::from(10.0).unwrap();
    let pers = Transform::perspective(fov, e.powi(-2), e.powi(3));
    Self(ProjectiveCamera::new(
      cam_to_world,
      pers,
      screen,
      res,
      focal_dist,
      lens_rad,
    ))
  }
  pub fn rays(&self, n: u32, xy: Vec2<T>) -> impl Iterator<Item = Ray3<T>> + '_
  where
    Standard: Distribution<T>, {
    let Vec2(x, y) = xy;
    let mut rng = thread_rng();
    (0..n).map(move |_| self.ray_to(Vec2(x + rng.gen(), y + rng.gen())))
  }
}

impl<T: Float> Cam<T> for PerspectiveCamera<T> {
  fn ray_to(&self, uv: Vec2<T>) -> Ray3<T> {
    let raster_pos = uv.extend(T::zero());
    // point in camera space
    let dir = self.0.screen_to_raster.apply_inv_pt(&raster_pos);
    let dir = self.0.cam_to_screen.apply_inv_pt(&dir).norm();
    let dir = self.0.cam_to_world.apply_vec(&dir).norm();
    let pos = self.0.cam_to_world.apply_pt(&Vec3::zero());
    Ray3::new(pos, dir)
  }
}
