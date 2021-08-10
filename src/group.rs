use super::pixel::PixelColor;
use super::tile::{Tile, TileCell};
use std::cmp::Ordering;
use std::collections::HashSet;
use std::fmt::{self, Debug, Formatter};

pub type CandidateTable<'a> = Vec<Vec<Candidate<'a>>>;

pub enum Candidacy {
  Available,
  Taken,
  End,
}

pub struct Candidate<'a> {
  tile: &'a TileCell<'a>,
  compat: usize,
}

impl<'a> Candidate<'a> {
  pub fn new(tile: &'a TileCell<'a>, compat: usize) -> Candidate<'a> {
    Candidate { tile, compat }
  }

  pub fn cmp_by_compat(&self, other: &Candidate<'_>) -> Ordering {
    // flipped comparison for descending order
    other.compat.cmp(&self.compat)
  }

  pub fn candidacy(&self) -> Candidacy {
    if self.compat == 0 {
      // Because candidates are sorted, if we hit 0 we know we can stop.
      return Candidacy::End;
    }

    if self.tile.borrow().is_none() {
      // If the tile has already been taken, we can't use it here.
      return Candidacy::Taken;
    }

    // Otherwise we're good to go!
    Candidacy::Available
  }

  pub fn borrow(&self) -> std::cell::Ref<'_, Option<&'a Tile<'a>>> {
    self.tile.borrow()
  }

  pub fn claim(&self) {
    self.tile.replace(None);
  }
}

pub struct Group<'a> {
  tiles: Vec<&'a Tile<'a>>,
}

impl<'a> Group<'a> {
  pub fn new(initial_tile: &'a Tile<'a>) -> Group<'a> {
    Group {
      tiles: vec![initial_tile],
    }
  }

  fn colors(curr: &Vec<&'a Tile<'a>>, new: Option<&'a Tile<'a>>) -> HashSet<&'a PixelColor> {
    let mut colors = HashSet::new();

    for tile in curr.iter() {
      for color in tile.colors().iter() {
        colors.insert(color);
      }
    }

    if let Some(tile) = new {
      for color in tile.colors().iter() {
        colors.insert(color);
      }
    }

    colors
  }

  pub fn try_add(&mut self, tile: &'a Tile<'a>) -> Result<(), ()> {
    let colors = Self::colors(&self.tiles, Some(tile));
    if colors.len() > Tile::MAX_COLORS {
      return Err(());
    }

    self.tiles.push(tile);
    Ok(())
  }

  pub fn iter(&self) -> std::slice::Iter<'_, &Tile> {
    self.tiles.iter()
  }
}

impl Debug for Group<'_> {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    let color_count = Self::colors(&self.tiles, None).len();

    f.debug_struct("Group")
      .field("Tiles", &self.tiles.len())
      .field("Colors", &color_count)
      .finish()
  }
}
