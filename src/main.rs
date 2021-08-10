mod group;
mod pixel;
mod tile;

use group::{Candidacy, Candidate, Group};
use image::{DynamicImage, GenericImageView};
use std::cell::RefCell;
use std::env;
use std::fs;
use tile::Tile;

fn main() {
  let path_opt = env::args().nth(1);

  if let Some(path) = path_opt {
    let imgr = image::open(path);

    match imgr {
      Ok(img) => run(img),
      Err(e) => panic!("Failed to open image: {}", e),
    }
  } else {
    panic!("No path was provided. Provide a path to the image.")
  }
}

fn run(img: DynamicImage) {
  let (width, height) = get_tile_dimensions(&img);
  let tiles_count = (width * height) as usize;
  let mut tiles = Vec::with_capacity(tiles_count);

  println!("Step 1: Instantiating all tiles...");

  for x in 0..width {
    for y in 0..height {
      let view = img.view(x * Tile::SIZE, y * Tile::SIZE, Tile::SIZE, Tile::SIZE);
      let tile = Tile::new(view, (x, y));

      if tile.is_empty() {
        continue;
      }

      tiles.push(tile);
    }
  }

  println!("Step 2: Converting tiles to cells...");

  let tile_cells = tiles
    .iter()
    .map(|t| RefCell::new(Some(t)))
    .collect::<Vec<_>>();

  println!("Step 3: Calculating tile compatibility...");

  // subtract tiles_count because we skip comparing a tile with itself
  let compats_count = usize::pow(tiles_count, 2) - tiles_count;
  let mut compats = Vec::with_capacity(compats_count);

  for tile_cell in &tile_cells {
    let mut tile_compats = Vec::with_capacity(tiles_count);

    for other_cell in &tile_cells {
      let compat = tile_cell
        .borrow()
        .unwrap()
        .compat(other_cell.borrow().unwrap());

      tile_compats.push(Candidate::new(other_cell, compat));
    }

    tile_compats.sort_by(|a, b| a.cmp_by_compat(&b));

    compats.push(tile_compats);
  }

  println!("Step 4: Sorting into groups...");

  let mut groups = Vec::new();

  for (n, tile_cell) in tile_cells.iter().enumerate() {
    let candidates = &compats[n];

    let tile_opt = tile_cell.borrow();
    if tile_opt.is_none() {
      // A tile already drafted into another group can't form its own.
      continue;
    }

    let tile = tile_opt.unwrap();
    let mut group = Group::new(tile);

    for candidate in candidates {
      match candidate.candidacy() {
        Candidacy::End => break,
        Candidacy::Taken => continue,
        Candidacy::Available => {
          let candidate_tile = candidate.borrow().unwrap();
          let result = group.try_add(candidate_tile);

          if let Ok(_) = result {
            candidate.claim();
          }
        }
      }
    }

    groups.push(group);
  }

  println!("Step 5: Outputting files...");

  fs::remove_dir_all("output").ok();
  fs::create_dir("output").unwrap();

  for (group_id, group) in groups.iter().enumerate() {
    let group_dir = format!("output/{}", group_id);
    fs::create_dir(&group_dir).unwrap();
    fs::write(format!("{}/_group.txt", group_dir), format!("{:?}", group)).unwrap();

    for (tile_id, tile) in group.iter().enumerate() {
      tile
        .save(format!("{}/{}.png", &group_dir, &tile_id))
        .unwrap();
    }
  }
}

#[inline]
fn get_tile_dimensions(img: &DynamicImage) -> (u32, u32) {
  let dims = img.dimensions();
  let width_ratio = dims.0 % Tile::SIZE;
  let height_ratio = dims.1 % Tile::SIZE;
  if width_ratio != 0 || height_ratio != 0 {
    panic!("Image dimensions must be multiples of 16, got {:?}", dims);
  }

  (dims.0 / Tile::SIZE, dims.1 / Tile::SIZE)
}
