// Copyright 2016 Brandon Thomas <bt@brand.io>, <echelon@gmail.com>

//! Font descriptors.

use error::RadiantTextError;
use ilda::animation::Animation;
use ilda::animation::Frame;
use std::collections::HashMap;

/// Load the simple font.
// TODO: lazy_static!
pub fn get_simple_font() -> Result<IldaFont, RadiantTextError> {
  let mut map = HashMap::new();
  map.insert('1', 16);
  map.insert('2', 17);
  map.insert('3', 18);
  map.insert('4', 19);
  map.insert('5', 20);
  map.insert('6', 21);
  map.insert('7', 22);
  map.insert('8', 23);
  map.insert('9', 24);

  IldaFont::build(
    "simple vector".to_string(),
    "./ild/font_simple_vector.ild".to_string(),
    map
  )
}

/// Load the narrow font.
// TODO: lazy_static!
pub fn get_narrow_font() -> Result<IldaFont, RadiantTextError> {
  let mut map = HashMap::new();
  map.insert('a', 22);
  map.insert('b', 9);

  IldaFont::build(
    "narrow vector".to_string(),
    "./ild/font_narrow_vector.ild".to_string(),
    map
  )
}

/// Describes an ILDA font file.
pub struct IldaFont {
  name: String,
  filename: String,
  x_top: i16,
  x_bottom: i16,
  y_top: i16,
  y_bottom: i16,
  character_map: HashMap<char, usize>,
  animation: Animation,
}

impl IldaFont {
  /// CTOR.
  pub fn build(name: String,
             filename: String,
             character_map: HashMap<char, usize>)
      -> Result<Self, RadiantTextError> {

    let animation = Animation::read_file(&filename)?;

    Ok(IldaFont {
      name: name,
      filename: filename,
      x_top: 0,
      x_bottom: 0,
      y_top: 0,
      y_bottom: 0,
      character_map: character_map,
      animation: animation,
    })
  }

  /// Get the entire ILDA animation.
  pub fn get_animation(&self) -> &Animation {
    &self.animation
  }

  /// Get the frame corresponding to the character, if it exists.
  pub fn get_char_frame(&self, character: char) -> Option<&Frame> {
    self.character_map.get(&character)
        .and_then(|index| self.animation.get_frame(*index))
  }
}
