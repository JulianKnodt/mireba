extern crate clap;
use clap::{App, Arg, SubCommand};
use gfx::two_d::{
  scene::{RawScene, Scene},
};
use std::{
  fs::File,
  io::{BufReader, BufWriter},
};

pub fn main() {
  let matches = App::new("Mitsume(見つめ)")
    .version("0.1")
    .author("julianknodt")
    .about("2D graphics processing")
    .arg(
      Arg::with_name("input")
        .short("i")
        .long("input")
        .value_name("FILE")
        .help("Input file (if example is specified this is where it will be stored")
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
    .subcommand(SubCommand::with_name("example").about("Creates an empty scene file"))
    .get_matches();

  let input_file = matches.value_of("input").unwrap();

  if let Some(_sub) = matches.subcommand_matches("example") {
    // Creates an example scene
    let empty_scene = RawScene::example();
    let f = File::create(input_file).expect("Could not create input file");
    let f = BufWriter::new(f);
    serde_json::to_writer_pretty(f, &empty_scene).expect("Failed to write example file");
    return;
  }
  let f = File::open(input_file).expect("Could not find input file");
  let f = BufReader::new(f);

  let output_file = matches.value_of("output").unwrap_or("out2d.jpg");
  let raw_scene: RawScene = serde_json::from_reader(f).expect("Error while reading json");
  let mut scene: Scene = raw_scene.into();
  scene.render();
  scene
    .film
    .to_image()
    .save(output_file)
    .expect("Failed to save file");
  // TODO extract film here
}
