use image::{Rgb, Rgba};
use std::cmp::Ordering;

#[derive(PartialEq, Eq, Hash)]
pub enum PixelColor {
  Empty,
  Full(Rgb<u8>),
}

impl PixelColor {
  pub fn new(color: Rgba<u8>) -> PixelColor {
    let [r, g, b, alpha] = color.0;

    match alpha {
      0 => PixelColor::Empty,
      255 => PixelColor::Full(Rgb([r, g, b])),
      _ => panic!("Semitransparent pixel found."),
    }
  }

  pub fn as_u32(&self) -> u32 {
    match self {
      PixelColor::Empty => 0,
      PixelColor::Full(color) => {
        let [r, g, b] = color.0;
        (1_u32 << 24) + ((r as u32) << 16) + ((g as u32) << 8) + b as u32
      }
    }
  }
}

impl PartialOrd for PixelColor {
  fn partial_cmp(&self, other: &PixelColor) -> Option<Ordering> {
    Some(self.cmp(&other))
  }
}

impl Ord for PixelColor {
  fn cmp(&self, other: &PixelColor) -> Ordering {
    self.as_u32().cmp(&other.as_u32())
  }
}
