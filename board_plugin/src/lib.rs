pub mod components;
pub mod resources;

use bevy::log;
use bevy::prelude::*;
use resources::tile_map::TileMap;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        log::info!("Loading BoardPlugin");
        app.add_startup_system(Self::create_board);
    }
}

impl BoardPlugin {
    pub fn create_board() {
        let mut tile_map = TileMap::empty(20, 20);
        tile_map.add_bombs(40);
        #[cfg(feature = "debug")]
        log::info!("{}", tile_map.console_output());
    }
}
