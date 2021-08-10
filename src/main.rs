mod group;
mod pixel;
mod tile;

use group::{Candidacy, Candidate, CandidateTable, Group};
use image::{DynamicImage, GenericImageView};
use std::cell::RefCell;
use std::env;
use std::fs;
use tile::{Tile, TileCell};

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
  // pipe operator when
  let tiles = create_tiles(&img);
  let cells = create_cells(&tiles);
  let table = create_candidate_table(&cells);
  let groups = create_groups(&cells, &table);
  create_output(&groups);
}

fn create_tiles(img: &DynamicImage) -> Vec<Tile> {
  let (width, height) = get_tile_dimensions(&img);
  let tiles_count = (width * height) as usize;
  let mut tiles = Vec::with_capacity(tiles_count);

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

  tiles
}

fn create_cells<'a>(tiles: &'a Vec<Tile<'a>>) -> Vec<TileCell<'a>> {
  tiles.iter().map(|t| RefCell::new(Some(t))).collect()
}

fn create_candidate_table<'a>(tiles: &'a Vec<TileCell<'a>>) -> CandidateTable<'a> {
  // subtract tiles_count because we skip comparing a tile with itself
  let tiles_count = tiles.len();
  let capacity = usize::pow(tiles_count, 2) - tiles_count;
  let mut table = Vec::with_capacity(capacity);

  for tile in tiles {
    let mut candidates = Vec::with_capacity(tiles_count);

    for other in tiles {
      let compat = tile.borrow().unwrap().compat(other.borrow().unwrap());
      candidates.push(Candidate::new(other, compat));
    }

    candidates.sort_by(|a, b| a.cmp_by_compat(&b));

    table.push(candidates);
  }

  table
}

fn create_groups<'a>(
  tiles: &'a Vec<TileCell<'a>>,
  table: &'a CandidateTable<'a>,
) -> Vec<Group<'a>> {
  let mut groups = Vec::new();

  for (n, tile_cell) in tiles.iter().enumerate() {
    let candidates = &table[n];

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

          if matches!(result, Ok(_)) {
            candidate.claim();
          }
        }
      }
    }

    groups.push(group);
  }

  groups
}

fn create_output(groups: &Vec<Group>) {
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
