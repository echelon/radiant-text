// Copyright (c) 2016 Brandon Thomas <bt@brand.io>, <echelon@gmail.com>
// Radiant Text: display text on a laser projector using the EtherDream DAC.

extern crate etherdream;
extern crate ilda;

use etherdream::dac::Dac;
use etherdream::protocol::X_MAX;
use etherdream::protocol::Y_MAX;
use etherdream::protocol::Point;

use ilda::parser::read_file;
use ilda::data::IldaEntry;

fn main() {
  println!("Reading ILDA file...");
  println!("{}, {}", etherdream::protocol::X_MAX, etherdream::protocol::X_MIN);

  let filename = "./ild/datboi.ild";
  //let filename = "./ild/koolaidman.ild";
  let points = read_points(filename).ok().unwrap();

  println!("Len: {}", &points.len());

  for point in &points {
    println!("Point: {:?}", point);
  }

  println!("Searching for EtherDream DAC...");
  let search_result = etherdream::network::find_first_dac().unwrap();

  let ip_address = search_result.ip_address;
  println!("Found dac: {}", ip_address);

  let mut dac = Dac::new(ip_address);

  let mut j = 0;

  // TODO: Draw something interesting.
  dac.play_function(move |num_points: u16| {
    println!("Play mut function: {}", num_points);

    let mut list = Vec::new();
    let size = num_points as usize;

    while list.len() < size {
      j = (j + 1) % &points.len();
      let point = points.get(j).unwrap();
      list.push(point.clone());
    }

    list
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
        //let point = Point::xy_rgb(pt.x, pt.y, pt.r as u16, pt.g as u16, pt.b as u16);
        let point = Point::xy_binary(pt.x, pt.y, true);
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
