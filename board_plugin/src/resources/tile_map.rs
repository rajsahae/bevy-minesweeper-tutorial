use crate::components::Coordinates;
use crate::resources::tile::Tile;

use rand::{thread_rng, Rng};
use std::ops::{Deref, DerefMut};

/// Delta coordinates for all 8 square neighbors
///
/// *--------*-------*-------*
/// | -1, 1  | 0, 1  | 1, 1  |
/// |--------|-------|-------|
/// | -1, 0  | tile  | 1, 0  |
/// |--------|-------|-------|
/// | -1, -1 | 0, -1 | 1, -1 |
/// *--------*-------*-------*
///
const NEIGHBOR_OFFSETS: [(i8, i8); 8] = [
    // bottom left
    (-1, -1),
    // bottom
    (0, -1),
    // bottom right
    (1, -1),
    // left
    (-1, 0),
    // right
    (1, 0),
    // top left
    (-1, 1),
    // top
    (0, 1),
    // top right
    (1, 1),
];

/// Base tile map
#[derive(Debug, Clone)]
pub struct TileMap {
    bomb_count: u16,
    height: u16,
    width: u16,
    map: Vec<Vec<Tile>>,
}

impl TileMap {
    /// Generate empty map
    pub fn empty(width: u16, height: u16) -> Self {
        Self {
            bomb_count: 0,
            height,
            width,
            map: (0..height)
                .map(|_| (0..width).map(|_| Tile::Empty).collect())
                .collect(),
        }
    }

    #[cfg(feature = "debug")]
    pub fn console_output(&self) -> String {
        let mut buffer = format!(
            "Map ({}, {}) with {} bombs:\n",
            self.width, self.height, self.bomb_count,
        );

        let line: String = (0..=(self.width + 1)).map(|_| '-').collect();
        buffer = format!("{}{}\n", buffer, line);
        for line in self.iter().rev() {
            buffer = format!("{}|", buffer);
            for tile in line.iter() {
                buffer = format!("{}{}", buffer, tile.console_output());
            }
            buffer = format!("{}|\n", buffer);
        }
        format!("{}{}", buffer, line)
    }

    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn height(&self) -> u16 {
        self.height
    }

    // pub fn bomb_count(&self) -> u16 {
    //     self.bomb_count
    // }

    pub fn surrounding_tiles(&self, coordinates: Coordinates) -> impl Iterator<Item = Coordinates> {
        NEIGHBOR_OFFSETS
            .iter()
            .copied()
            .map(move |offset| coordinates + offset)
    }

    pub fn is_bomb_at(&self, coordinates: Coordinates) -> bool {
        if coordinates.x >= self.width || coordinates.y >= self.height {
            false
        } else {
            self.map[coordinates.y as usize][coordinates.x as usize].is_bomb()
        }
    }

    pub fn bomb_count_at(&self, coordinates: Coordinates) -> u8 {
        if self.is_bomb_at(coordinates) {
            0
        } else {
            self.surrounding_tiles(coordinates)
                .filter(|neighbor| self.is_bomb_at(*neighbor))
                .count() as u8
        }
    }

    pub fn add_bombs(&mut self, count: u16) {
        self.bomb_count = count;
        let mut remaining = count;
        let mut rng = thread_rng();

        while remaining > 0 {
            let (x, y) = (
                rng.gen_range(0..self.width) as usize,
                rng.gen_range(0..self.height) as usize,
            );

            if let Tile::Empty = self[y][x] {
                self[y][x] = Tile::Bomb;
                remaining -= 1;
            }
        }

        for y in 0..self.height {
            for x in 0..self.width {
                let coord = Coordinates { x, y };
                if self.is_bomb_at(coord) {
                    continue;
                }

                let num = self.bomb_count_at(coord);

                if num > 0 {
                    self.map[y as usize][x as usize] = Tile::Neighbor(num);
                }
            }
        }
    }
}

impl Deref for TileMap {
    type Target = Vec<Vec<Tile>>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl DerefMut for TileMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}
