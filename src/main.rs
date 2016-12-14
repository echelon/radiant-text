// Copyright (c) 2016 Brandon Thomas <bt@brand.io>, <echelon@gmail.com>
// Radiant Text: display text on a laser projector using the EtherDream DAC.

extern crate etherdream;
extern crate ilda;

use etherdream::dac::Dac;
use etherdream::protocol::Point;

fn main() {
  println!("Searching for EtherDream DAC...");
  let search_result = etherdream::network::find_first_dac().unwrap();

  let ip_address = search_result.ip_address;
  println!("Found dac: {}", ip_address);

  let mut dac = Dac::new(ip_address);

  // TODO: Draw something interesting.
  dac.play_function(|num_points: u16| {
    let mut list = Vec::new();
    for i in 0..num_points {
      let x = (i % 10_000) as i16;
      list.push(Point::xy_binary(x, 0, true));
    }
    list
  });
}
