extern crate clap;
use clap::{App, Arg};
use gfx::{
  accelerator::naive::Naive,
  integrator::direct::Direct,
  scene::{RawScene, Scene},
};
use std::fs::File;

// TODO need to setup this so it can be switched out at compile time
/// This is the accelerator used by the render
pub type Accelerator = Naive;

pub fn main() {
  let matches = App::new("Mireba(見れば)")
    .version("0.1")
    .author("julianknodt")
    .about("Photorealistic Rendering")
    .arg(
      Arg::with_name("input")
        .short("i")
        .long("input")
        .value_name("FILE")
        .help("Input scene file")
        .required(true)
        .takes_value(true),
    )
    .arg(
      Arg::with_name("output")
        .short("o")
        .long("output")
        .value_name("FILE")
        .help("Output file")
        .required(false)
        .takes_value(true),
    )
    .get_matches();
  let input_file = matches.value_of("input").unwrap();
  let output_file = matches.value_of("output").unwrap_or("out.jpg");
  let f = File::open(input_file).expect("Could not find input file");
  let raw_scene: RawScene = serde_json::from_reader(f).expect("Error while reading json");
  let scene: Scene<(), Accelerator> = raw_scene.build();
  scene.render(Direct {});
  scene
    .camera
    .film()
    .to_image()
    .save(output_file)
    .expect("Failed to save file");
  // TODO extract film here
}
