use super::pixel::PixelColor;
use image::{DynamicImage, GenericImageView, ImageError, SubImage as View};
use std::cell::RefCell;
use std::collections::HashSet;

pub type TileCell<'a> = RefCell<Option<&'a Tile<'a>>>;

pub struct Tile<'a> {
  view: View<&'a DynamicImage>,
  colors: HashSet<PixelColor>,
  tile_coords: (u32, u32),
}

impl<'a> Tile<'a> {
  pub const SIZE: u32 = 16;
  pub const MAX_COLORS: usize = 16;

  pub fn new(view: View<&'a DynamicImage>, tile_coords: (u32, u32)) -> Tile<'a> {
    let (width, height) = view.dimensions();
    let mut colors = HashSet::with_capacity(Self::MAX_COLORS);

    for x in 0..width {
      for y in 0..height {
        let color = PixelColor::new(view.get_pixel(x, y));
        colors.insert(color);
      }
    }

    if colors.len() > Self::MAX_COLORS {
      panic!("Tile at {:?} has more than 16 colours.", tile_coords);
    }

    Tile {
      view,
      colors,
      tile_coords,
    }
  }

  pub fn is_empty(&self) -> bool {
    self.colors.len() == 1 && matches!(self.colors.iter().nth(0), Some(PixelColor::Empty))
  }

  pub fn colors(&self) -> &HashSet<PixelColor> {
    &self.colors
  }

  pub fn compat(&self, other: &Tile<'_>) -> usize {
    if self == other {
      return 0;
    }

    macro_rules! by {
      ($method:ident) => {
        self
          .colors
          .$method(&other.colors)
          .collect::<HashSet<_>>()
          .len();
      };
    }

    let total_colors = by!(union);
    if total_colors > Self::MAX_COLORS {
      return 0;
    }

    by!(intersection)
  }

  pub fn save(&self, location: String) -> Result<(), ImageError> {
    let image = self.view.to_image();
    image.save(location)
  }
}

impl PartialEq for Tile<'_> {
  fn eq(&self, other: &Tile<'_>) -> bool {
    self.tile_coords == other.tile_coords
  }
}
