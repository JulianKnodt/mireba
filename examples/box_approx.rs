use clap::{App, Arg};
use quick_maths::Vec3;
use gfx::bounds::Bounds;
use image::load;

fn main() {
  let matches = App::new("BoxApprox")
    .version("0.1")
    .author("jk")
    .arg(
      Arg::with_name("input")
        .short("i")
        .long("input")
        .value_name("FILE")
        .help("Input image file name")
        .required(true),
    )
    .arg(
      Arg::with_name("num-iters")
        .short("n")
        .long("num-iters")
        .value_name("#")
        .help("Number of iterations to optimize for")
        .default_value("10000")
    )
    .get_matches();
  let img = open(matches.value_of("input").unwrap())
    .expect("Failed to open file")
    .into_rgba();
  let num_iters = matches.value_of("num-iters").unwrap();

  let mut curr: ImageBuffer<Rgba<u8>, _> = ImageBuffer::new(img.width(), img.height());
  let width = img.width() as f32;
  let height = img.height() as f32;

  let compute_box_color = |bounds|: Vec3<u8> {
    let mut color_sum = Vec3::zero();
    let mut count = 0u64;
    for [x, y] in bounds.mesh_grid() {
      let &RGBA([r, g, b, _]) = img.get_pixel(x, y);
      // TODO can use alpha as a multiplicative factor by normalizing between 0 and 1
      color_sum += Vec3::new(r as u64, g as u64, b as u64);
      count += 1;
    }
    (color/count).cast()
  }

  for i in 0..num_iters {
    let compute_proposed = |v: [f32; 2]| -> f32 {
      todo!()
    }
  }
}
