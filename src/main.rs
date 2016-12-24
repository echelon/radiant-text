// Copyright (c) 2016 Brandon Thomas <bt@brand.io>, <echelon@gmail.com>
// Radiant Text: display text on a laser projector using the EtherDream DAC.

extern crate etherdream;
extern crate ilda;

use std::sync::Arc;
use std::boxed::Box;
use etherdream::dac::Dac;
use etherdream::protocol::COLOR_MAX;
use etherdream::protocol::Point;
use etherdream::protocol::X_MAX;
use etherdream::protocol::Y_MAX;
use ilda::animation::Animation;
use ilda::animation::Frame;
use ilda::data::IldaEntry;
use ilda::parser::read_file;

fn main() {
  println!("Reading ILDA file...");
  println!("{}, {}", etherdream::protocol::X_MAX, etherdream::protocol::X_MIN);

  let filename = "./ild/datboi.ild";
  //let filename = "./ild/koolaidman.ild";

  let animation = Animation::read_file(filename).unwrap();

  println!("Animation Len: {}", &animation.frame_count());

  let points = read_points(filename).ok().unwrap();
  //let mut iterators = PointIterators { frame: None, point: None };

  println!("Searching for EtherDream DAC...");
  let search_result = etherdream::network::find_first_dac().unwrap();

  let ip_address = search_result.ip_address;
  println!("Found dac: {}", ip_address);

  let mut dac = Dac::new(ip_address);

  let mut frame_index = 0;
  let mut point_index = 0;

  // TODO: Draw something interesting.
  dac.play_function(move |num_points: u16| {
    let limit = num_points as usize;
    let mut buf = Vec::new();

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
              frame_index += 1;
              point_index = 0;
              continue;
            },
            Some(ref point) => {
              buf.push(Point::xy_binary(point.x, point.y, true));
              point_index += 1;
            }
          }
        },
      }
    }

    buf
  });
}

// TODO: Error class
fn read_points(filename: &str) -> Result<Vec<Point>, ilda::IldaError> {
  let headers = read_file(filename)?;
  let mut points = Vec::new();

  let mut count = 0;
  for header in headers {
    match header {
      /*IldaEntry::HeaderEntry(h) => {
        println!("HEADER: {:?}", h);
        count += 1;
        if count > 1 {
          break;
        }
      },*/
      IldaEntry::TcPoint2dEntry(pt) => {
        let point = Point::xy_rgb(pt.x, pt.y, color(pt.g), color(pt.b), color(pt.r));
        //let point = Point::xy_binary(pt.x, pt.y, true);
        points.push(point);
      },
      IldaEntry::TcPoint3dEntry(_) => {
        println!("3D point unsupported")
      }
      _ => {
        println!("UNSUPPORTED");
      },
    }
  }
  Ok(points)
}

/// Map the color ranges.
fn color(color: u8) -> u16 {
  // 0 -> 0
  // 127 -> 32767
  // 255 -> 65535
  (color as u16) << 8
}
