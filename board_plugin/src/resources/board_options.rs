use bevy::prelude::*;
use serde::{Deserialize, Serialize};

// #[cfg(feature = "debug")]
// use bevy_inspector_egui::prelude::*;

// /// Tile size options
// #[cfg_attr(
//     feature = "debug",
//     derive(Reflect, InspectorOptions),
//     reflect(InspectorOptions)
// )]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum TileSize {
    Fixed(f32),
    Adaptive { min: f32, max: f32 },
}

impl Default for TileSize {
    fn default() -> Self {
        Self::Adaptive {
            min: 10.0,
            max: 50.0,
        }
    }
}

// #[cfg_attr(
//     feature = "debug",
//     derive(Reflect, InspectorOptions),
//     reflect(InspectorOptions)
// )]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum BoardPosition {
    Centered { offset: Vec3 },
    Custom(Vec3),
}

impl Default for BoardPosition {
    fn default() -> Self {
        Self::Centered {
            offset: Default::default(),
        }
    }
}

// Use serde to allow saving/loading option presets
// #[cfg_attr(
//     feature = "debug",
//     derive(Reflect, InspectorOptions),
//     reflect(InspectorOptions)
// )]
#[derive(Debug, Clone, Deserialize, Serialize, Resource)]
pub struct BoardOptions {
    pub map_size: (u16, u16),
    pub bomb_count: u16,
    pub position: BoardPosition,
    pub tile_size: TileSize,
    pub tile_padding: f32,
    pub safe_start: bool, // generate a safe place to start
}

impl Default for BoardOptions {
    fn default() -> Self {
        Self {
            map_size: (15, 15),
            bomb_count: 30,
            position: Default::default(),
            tile_size: Default::default(),
            tile_padding: 0.,
            safe_start: false,
        }
    }
}
