use image::{Rgb, Rgba};
use radix_fmt::radix;
use std::cmp::Ordering;
use std::fmt::{self, Debug, Formatter};

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

impl Debug for PixelColor {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      PixelColor::Empty => write!(f, "(empty)"),
      PixelColor::Full(_) => {
        let hex = self.as_u32();
        let hex = format!("{}", radix(hex, 16));
        let hex = &hex[1..=hex.len() - 1];

        write!(f, "{}", hex)
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
