extern crate num;
extern crate rand;
extern crate rand_distr;
use std::ops::{Add, Sub, Mul, Neg, Div, Range};
use num::{Zero, One};
use rand_distr::{StandardNormal, Standard};
use rand::prelude::*;
use std::sync::Arc;

pub struct Vec3<T>(T, T, T);

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

def_op!(Add, add, +);
def_op!(Mul, mul, *);
def_op!(Sub, sub, -);
def_op!(Div, div, /);
def_scalar_op!(Add, add, +);
def_scalar_op!(Mul, mul, *);
def_scalar_op!(Sub, sub, -);
def_scalar_op!(Div, div, /);

impl<T>Neg for Vec3<T> where T: Neg {
  type Output = Vec3<<T as Neg>::Output>;
  fn neg(self) -> Self::Output{ Vec3(-self.0, -self.1, -self.2) }
}

impl<T> Zero for Vec3<T> where T: Zero + PartialEq {
  fn zero() -> Self { Vec3(T::zero(), T::zero(), T::zero()) }
  fn is_zero(&self) -> bool {
    let zero = T::zero();
    self.0 == zero && self.1 == zero && self.2 == zero
  }
}

impl<T> One for Vec3<T> where T: One + PartialEq {
  fn one() -> Self { Vec3(T::one(), T::one(), T::one()) }
  fn is_one(&self) -> bool {
    let one = T::one();
    self.0 == one && self.1 == one && self.2 == one
  }
}

impl<T> Default for Vec3<T> where T: Default {
  fn default() -> Self { Vec3(T::default(), T::default(), T::default()) }
}

impl<T> Copy for Vec3<T> where T: Copy { }
impl<T> Clone for Vec3<T> where T: Clone {
  fn clone(&self) -> Self {
    Vec3(self.0.clone(), self.1.clone(), self.2.clone())
  }
}

impl<T> Vec3<T> where T: num::Float {
  fn sqr_magn(&self) -> T { self.0.powi(2) + self.1.powi(2) + self.2.powi(2) }
  fn magn(&self) -> T { self.sqr_magn().sqrt() }
  fn norm(&self) -> Self { (*self)/self.magn() }
  fn dot(&self, o: Self) -> T { self.0 * o.0 + self.1 * o.1 + self.2 * o.2 }
  fn sqrt(&self) -> Self { Vec3(self.0.sqrt(), self.1.sqrt(), self.2.sqrt()) }
  fn cross(&self, o: Self) -> Self {
    Vec3(
      self.1 * o.2 - self.2 * o.1,
      self.2 * o.0 - self.0 * o.2,
      self.0 * o.1 - self.1 * o.0,
    )
  }
  fn reflect(self, across: Vec3<T>) -> Self {
    self - across * self.dot(across) * T::from(2.0).unwrap()
  }
  fn refract(self, norm: Vec3<T>, refract_ratio: T) -> Option<Vec3<T>> {
    let u = self.norm();
    let dt = u.dot(norm);
    Some(T::one() - refract_ratio.powi(2) * (T::one() - dt.powi(2)))
      .filter(|discrim| discrim.is_sign_positive())
      .map(|d| (u - norm*dt)*refract_ratio - norm*d.sqrt())
  }
}

pub struct Ray<T> {
  pos: Vec3<T>,
  dir: Vec3<T>,
}

impl<T> Ray<T> {
  fn new(pos: Vec3<T>, dir: Vec3<T>) -> Self { Ray{pos, dir} }
}
impl<T> Ray<T> where T: num::Float {
  fn at(&self, t: T) -> Vec3<T> { self.pos + self.dir * t }
}

pub struct Screen {
  w: usize,
  h: usize,
  data: Vec<Vec3<f32>>,
}

impl Screen {
  fn new(w: usize, h: usize) -> Self {
    let mut data = Vec::with_capacity(w * h);
    data.resize_with(w * h, Default::default);
    Screen{w: w, h: h, data: data}
  }
  fn set(&mut self, x: usize, y: usize, val: Vec3<f32>) {
    self.data[x + self.w * y] = val;
  }
}

impl Screen {
  pub fn write_ppn(&self) {
    print!("P3\n{} {}\n255\n", self.w, self.h);
    (0..self.h).for_each(|y|
      (0..self.w).for_each(|x| {
        let color = self.data.get(x + y * self.w).cloned().unwrap_or(Vec3::zero());
        println!("{} {} {}", color.0 as i32, color.1 as i32, color.2 as i32);
      }));
  }
}

fn lerp<T>(u: T, min: Vec3<T>, max: Vec3<T>) -> Vec3<T> where T: num::Float {
  min*u + max*(T::one()-u)
}

fn quad_solve<T>(a: T, b: T, c: T) -> Option<(T, T)> where T: num::Float {
    Some(b*b - a*c*T::from(4.0).unwrap())
      .filter(|discrim| discrim.is_sign_positive())
      .map(|discrim| discrim.sqrt())
      .map(|d|((-b + d)/(T::from(2.0).unwrap() * a),(-b - d)/(T::from(2.0).unwrap() * a)))
}

pub struct Visibility<T> {
  param: T,
  pos: Vec3<T>,
  norm: Vec3<T>,
  mat: Arc<dyn Material<T>>,
}

pub trait Visible<T> {
  // returns parameter T, position, and normal
  fn hit(&self, r: &Ray<T>, bounds: Option<Range<T>>) -> Option<Visibility<T>>;
}

pub struct Sphere<T> {
  center: Vec3<T>,
  radius: T,
  mat: Arc<dyn Material<T>>,
}

impl<T> Sphere<T> where T: num::Float {
  fn new(center: Vec3<T>, radius: T, mat: Arc<dyn Material<T>>) -> Self {
    Sphere{center, radius, mat}
  }
  fn normal(&self, v: Vec3<T>) -> Vec3<T> { v - self.center }
}

impl<T> Visible<T> for Sphere<T> where T: num::Float {
  fn hit(&self, r: &Ray<T>, bounds: Option<Range<T>>) -> Option<Visibility<T>> {
    let from_sphere = r.pos - self.center;
    let a = r.dir.sqr_magn();
    let b = T::from(2.0).unwrap() * r.dir.dot(from_sphere);
    let c = from_sphere.sqr_magn() - self.radius.powi(2);
    quad_solve(a, b, c)
      .filter(|(t0, t1)| t0.is_sign_positive() || t1.is_sign_positive())
      .map(|(t0, t1)|
        if t0.is_sign_negative() { t1 }
        else if t1.is_sign_negative() { t0 }
        else { t0.min(t1) }
      )
      .filter(|t| bounds.map_or(true, |b| b.contains(t)))
      .map(|t| {
        let vec = r.at(t);
        Visibility{
          param: t,
          pos: vec,
          norm: self.normal(vec),
          mat: Arc::clone(&self.mat),
        }
      })
  }
}

impl<T> Visible<T> for Vec<Box<dyn Visible<T>>> where T: num::Float {
  fn hit(&self, r: &Ray<T>, bounds: Option<Range<T>>) -> Option<Visibility<T>> {
    let mut curr_bound = bounds.unwrap_or(T::zero()..T::infinity());
    self.iter().fold(None, |nearest, item|
      if let Some(vis) = item.hit(r, Some(curr_bound.clone())) {
        match &nearest {
          Some(prev) if vis.param > prev.param => nearest,
          Some(_) | None => {
            curr_bound.end = vis.param;
            Some(vis)
          },
      }} else { nearest })
  }
}

pub struct Camera<T> {
  pos: Vec3<T>,

  // Screen positions
  ll_corner: Vec3<T>,
  hori: Vec3<T>,
  vert: Vec3<T>,
}

pub fn rand_in_unit_disk<T>() -> (T, T)
  where T: num::Float, Standard: Distribution<T> {
  let mut rng = thread_rng();
  let r = rng.gen().sqrt();
  let theta = rng.gen() * T::from(2.0 * std::f64::consts::PI).unwrap();
  (r * theta.cos(), r * theta.sin())
}

impl<T> Camera<T> where T: num::Float, Standard: Distribution<T> {
  pub fn new(vert_fov_deg: T, aspect_ratio: T) -> Self {
    let theta = vert_fov_deg.to_radians();
    let half_height = (theta/T::from(2.0).unwrap()).tan();
    let half_width = half_height * aspect_ratio;
    Camera{
      ll_corner: Vec3(-half_width, -half_height, -T::one()),
      vert: Vec3(half_width * T::from(2.0).unwrap(), T::zero(), T::zero()),
      hori: Vec3(T::zero(), half_height * T::from(2.0).unwrap(), T::zero()),
      pos: Vec3::zero(),
    }
  }
  pub fn aimed(from: Vec3<T>, at: Vec3<T>, up: Vec3<T>, vert_fov_deg: T, aspect: T) -> Self {
    let theta = vert_fov_deg.to_radians();
    let half_height = (theta/T::from(2.0).unwrap()).tan();
    let half_width = half_height * aspect;
    let w = (from-at).norm();
    let u = up.cross(w).norm();
    let v = w.cross(u).norm();
    Camera{
      ll_corner: from - v*half_height - u*half_width - w,
      vert: u*half_width * T::from(2.0).unwrap(),
      hori: v*half_height * T::from(2.0).unwrap(),
      pos: from,
    }
  }
  pub fn to(&self, u: T, v: T) -> Ray<T> {
    Ray::new(self.pos, self.ll_corner+self.hori*v+self.vert*u - self.pos)
  }
  pub fn rays(&self, n: usize, x: T, y: T, w: T, h: T) -> Vec<Ray<T>> {
    let mut rng = thread_rng();
    (0..n).map(|_| self.to((x+rng.gen())/w, (y+rng.gen())/h)).collect()
  }
}

fn rand_in_unit_sphere<T>() -> Vec3<T>
  where T: num::Float, StandardNormal: Distribution<T>, Standard: Distribution<T> {
  let std = StandardNormal;
  let mut rng = thread_rng();
  let v = Vec3(rng.sample(std), rng.sample(std), rng.sample(std)).norm();
  v/rng.gen().powf(T::from(1.0/3.0).unwrap())
}

fn color<V, T>(r: &Ray<T>, item: &V) -> Vec3<T>
  where T: num::Float, StandardNormal: Distribution<T>, Standard: Distribution<T>,
  V: Visible<T> {
  if let Some(vis) = item.hit(&r, Some(T::from(0.001).unwrap()..T::infinity())) {
    vis.mat.scatter(&r, &vis)
      .map(|(atten, bounce)| color(&bounce, item) * atten)
      .unwrap_or_else(Vec3::zero)
  } else {
    lerp(
      (r.dir.norm().1+T::one()) * T::from(0.5).unwrap(), Vec3::one(),
      Vec3(T::from(0.5).unwrap(), T::from(0.7).unwrap(), T::one())
    )
  }
}

pub trait Material<T> {
  // return attenuation, out ray
  fn scatter(&self, ray_in: &Ray<T>, vis: &Visibility<T>) -> Option<(Vec3<T>, Ray<T>)>;
}

pub struct Lambertian<T> { albedo: Vec3<T>, }
impl<T> Copy for Lambertian<T> where T: Copy { }
impl<T> Clone for Lambertian<T> where T: Clone {
  fn clone(&self) -> Self { Lambertian { albedo: self.albedo.clone() } }
}

impl<T> Material<T> for Lambertian<T>
  where T: num::Float, StandardNormal: Distribution<T>, Standard: Distribution<T> {
  fn scatter(&self, _: &Ray<T>, vis: &Visibility<T>) -> Option<(Vec3<T>, Ray<T>)> {
    let target = vis.pos + vis.norm.norm() + rand_in_unit_sphere();
    Some((self.albedo, Ray::new(vis.pos, target-vis.pos)))
  }
}

pub struct Metallic<T> {
  albedo: Vec3<T>,
  fuzz: T,
}
impl<T> Copy for Metallic<T> where T: Copy { }
impl<T> Clone for Metallic<T> where T: Clone {
  fn clone(&self) -> Self { Metallic { albedo: self.albedo.clone(), fuzz: self.fuzz.clone(), } }
}
impl<T> Metallic<T> where T: num::Float {
  fn new(r: T, g: T, b: T, f: T) -> Self { Metallic { albedo: Vec3(r, g, b), fuzz: f, } }
}

impl<T> Material<T> for Metallic<T>
  where T: num::Float, StandardNormal: Distribution<T>, Standard: Distribution<T> {
  fn scatter(&self, r: &Ray<T>, vis: &Visibility<T>) -> Option<(Vec3<T>, Ray<T>)> {
    let bounce = Ray::new(
      vis.pos,
      r.dir.norm().reflect(vis.norm.norm()) + rand_in_unit_sphere() * self.fuzz
    );
    Some((self.albedo, bounce))
      .filter(|(_,b)| b.dir.dot(vis.norm).is_sign_positive())
  }
}

pub struct Dielectric<T> {
  refract_idx: T
}
impl<T> Copy for Dielectric<T> where T: Copy { }
impl<T> Clone for Dielectric<T> where T: Clone {
  fn clone(&self) -> Self { Dielectric{ refract_idx: self.refract_idx.clone() } }
}
fn schlick<T>(cosi: T, refr_idx: T) -> T where T: num::Float {
  let r0 = ((T::one()-refr_idx)/(T::one()+refr_idx)).powi(2);
  r0 + (T::one()-r0) * (T::one() - cosi).powi(5)
}
impl<T> Dielectric<T> {
  fn new(t: T) -> Self { Dielectric{ refract_idx: t } }
}
impl<T> Material<T> for Dielectric<T>
  where T: num::Float, Standard: Distribution<T> {
  fn scatter(&self, r: &Ray<T>, vis: &Visibility<T>) -> Option<(Vec3<T>, Ray<T>)> {
    let unit_norm = vis.norm.norm();
    let v = r.dir.norm().dot(unit_norm);
    let (out_norm, refr_ratio, cosi) = if v.is_sign_positive() {
      (-vis.norm.norm(), self.refract_idx, self.refract_idx * v / r.dir.magn())
    } else {
      (vis.norm.norm(), self.refract_idx.recip(), - v / r.dir.magn())
    };
    let out = r.dir.refract(out_norm, refr_ratio)
      .filter(|_| rand::random() < schlick(cosi, self.refract_idx))
      .unwrap_or(r.dir.norm().reflect(unit_norm));
    Some((Vec3::one(), Ray::new(vis.pos, out)))
  }
}

fn main() {
  let (w, h) = (400, 200);
  let n = 150;

  let mut screen = Screen::new(w, h);
  let camera = Camera::aimed(Vec3(-2.,2.,1.), Vec3(0.,0.,-1.), Vec3(0.,1.,0.,), 90.,(w/h) as f32);
  let lamb = Lambertian{
    albedo: Vec3(0.5, 0.5, 0.5),
  };
  let red_metal = Metallic::new(1.0, 0.8, 0.5, 0.25);
  let di = Dielectric::new(1.5);
  let items : Vec<Box<dyn Visible<f32>>> = vec!(
    Box::new(Sphere::new(Vec3(0.0, 0.0, -1.0), 0.5, Arc::new(lamb))),
    Box::new(Sphere::new(Vec3(0.0, -100.5, -1.0), 100.0, Arc::new(lamb))),
    Box::new(Sphere::new(Vec3(1.0, 0.0, -1.0), 0.5, Arc::new(di))),
    Box::new(Sphere::new(Vec3(-2.0, 0.0, -1.0), 0.5, Arc::new(red_metal))),
  );
  (0..w).for_each(|x| {
    (0..h).for_each(|y| {
      let color = camera.rays(n, x as f32, y as f32, w as f32, h as f32)
        .iter()
        .fold(Vec3::zero(),|acc,r| acc + (color(r, &items).sqrt() * 255.9))/(n as f32);
      screen.set(x, h-y-1, color);
    });
  });
  screen.write_ppn();
}
