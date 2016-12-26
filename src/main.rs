// Copyright (c) 2016 Brandon Thomas <bt@brand.io>, <echelon@gmail.com>
// Radiant Text: display text on a laser projector using the EtherDream DAC.

extern crate etherdream;
extern crate ilda;

mod error;
mod font;

use etherdream::dac::Dac;
use etherdream::protocol::COLOR_MAX;
use etherdream::protocol::Point;
use etherdream::protocol::X_MAX;
use etherdream::protocol::Y_MAX;
use font::IldaFont;
use font::get_narrow_font;
use font::get_simple_font;
use ilda::animation::Animation;
use ilda::animation::Frame;
use ilda::data::IldaEntry;
use ilda::parser::read_file;
use std::boxed::Box;
use std::sync::Arc;

fn main() {
  println!("Reading ILDA file...");
  println!("{}, {}", etherdream::protocol::X_MAX, etherdream::protocol::X_MIN);

  let filename = "./ild/datboi.ild"; // Works
  let filename = "./ild/cogz99.ild"; // Works (animated)
  let filename = "./ild/font_impact.ild"; // Works
  let filename = "./ild/font_lucida.ild"; // Works
  let filename = "./ild/thunda2.ild"; // Works (animated)
  let filename = "./ild/Charmander.ild";
  let filename = "./ild/nyancat.ild"; // Works!! :D
  let filename = "./ild/formatt.ild"; // Fails on header read
  let filename = "./ild/in.ild"; // WTF is this
  let filename = "./ild/Skittles.ILD"; // Fails to render
  let filename = "./ild/font_narrow_vector.ild";
  let filename = "./ild/font_simple_vector.ild";
  //let filename = "./ild/koolaidman.ild"; // TODO: Doesn't render correctly?

  let font = get_simple_font().ok().unwrap();

  let animation = Animation::read_file(filename).unwrap();

  /*let animation = match Animation::read_file(filename) {
    Ok(animation) => animation,
    Err(e) => {
      println!("Error: {:?}", e);
      panic!();
    },
  };*/

  println!("Animation Len: {}", &animation.frame_count());

  println!("Searching for EtherDream DAC...");
  let search_result = etherdream::network::find_first_dac().unwrap();

  let ip_address = search_result.ip_address;
  println!("Found dac: {}", ip_address);

  let mut dac = Dac::new(ip_address);

  let mut frame_index = 0;
  let mut frame_repeat_count = 0;
  let mut point_index = 0;

  dac.play_function(move |num_points: u16| {
    let limit = num_points as usize;
    let mut buf = Vec::new();
    let frame = font.get_char_frame('6').unwrap();

    while buf.len() < limit {
      match frame.get_point(point_index) {
        None => {
          point_index = 0;
          continue;
        },
        Some(ref point) => {
          println!("Point : {}", point_index);
          let r = color(point.r);
          let g = color(point.g);
          let b = color(point.b);
          buf.push(Point::xy_rgb(point.x, point.y, r, g, b));
          point_index += 1;
        },
      }
    }

    buf
  });

  // TODO: Draw something interesting.
  /*dac.play_function(move |num_points: u16| {
    let limit = num_points as usize;
    let mut buf = Vec::new();

    let animation = font.get_animation();
    while buf.len() < limit {
      match animation.get_frame(frame_index) {
        None => {
          frame_index = 0;
          point_index = 0;
          continue;
        },
        Some(ref frame) => {
          match frame.get_point(point_index) {
            None => {
              // NB: Repeat slows the animation speed.
              frame_repeat_count += 1;
              if frame_repeat_count > 20_000 {
                frame_index += 1;
                frame_repeat_count = 0;
              }
              point_index = 0;
              continue;
            },
            Some(ref point) => {
              println!("Frame: {}", frame_index);
              let r = color(point.r);
              let g = color(point.g);
              let b = color(point.b);
              buf.push(Point::xy_rgb(point.x, point.y, r, g, b));
              point_index += 1;
            }
          }
        },
      }
    }

    buf
  });*/
}

/// Map the color ranges.
fn color(color: u8) -> u16 {
  // 0 -> 0
  // 127 -> 32767
  // 255 -> 65535
  (color as u16) << 8
}
