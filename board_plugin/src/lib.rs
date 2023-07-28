mod bounds;
pub mod components;
mod events;
pub mod resources;
mod systems;

pub use bounds::Bounds2;

use bevy::log;
use bevy::prelude::*;
use bevy::text::{Text, TextAlignment};
use bevy::utils::HashMap;
use bevy::window::PrimaryWindow;
use components::*;
use events::*;
use resources::{
    board::Board, tile::Tile, tile_map::TileMap, BoardAssets, BoardOptions, BoardPosition, TileSize,
};
use systems::{input_handling, mark_tiles, trigger_event_handler, uncover_tiles};

pub struct BoardPlugin<T> {
    pub start_state: T,
    pub running_state: T,
    pub end_state: T,
}

impl<T: States> Plugin for BoardPlugin<T> {
    fn build(&self, app: &mut App) {
        log::info!("Loading BoardPlugin");
        app.add_event::<TileTriggerEvent>()
            .add_event::<TileMarkEvent>()
            .add_event::<BombExplosionEvent>()
            .add_event::<BoardCompletedEvent>()
            .add_systems(OnExit(self.start_state.clone()), Self::create_board)
            .add_systems(OnEnter(self.end_state.clone()), Self::cleanup)
            .add_systems(
                Update,
                (
                    input_handling,
                    trigger_event_handler,
                    uncover_tiles,
                    mark_tiles,
                )
                    .run_if(in_state(self.running_state.clone())),
            );
    }
}

impl<T> BoardPlugin<T> {
    fn create_board(
        mut commands: Commands,
        board_options: Option<Res<BoardOptions>>,
        board_assets: Res<BoardAssets>,
        query: Query<&Window, With<PrimaryWindow>>,
    ) {
        let options = match board_options {
            None => BoardOptions::default(),
            Some(o) => o.clone(),
        };

        let mut tile_map = TileMap::empty(options.map_size.0, options.map_size.1);
        let window = query.single();

        let tile_size = match options.tile_size {
            TileSize::Fixed(s) => s,
            TileSize::Adaptive { min, max } => {
                Self::adaptive_tile_size(window, (min, max), (tile_map.width(), tile_map.height()))
            }
        };

        tile_map.add_bombs(options.bomb_count);

        let board_size = Vec2::new(
            tile_map.width() as f32 * tile_size,
            tile_map.height() as f32 * tile_size,
        );

        let position = match options.position {
            BoardPosition::Centered { offset } => {
                Vec3::new(-(board_size.x / 2.), -(board_size.y / 2.), 0.) + offset
            }
            BoardPosition::Custom(v) => v,
        };

        let mut covered_tiles =
            HashMap::with_capacity((tile_map.width() * tile_map.height()).into());

        let mut safe_start = None;

        let board_entity = commands
            .spawn(SpatialBundle {
                visibility: Visibility::Visible,
                transform: Transform::from_translation(position),
                ..default()
            })
            .insert(Name::new("Board"))
            .with_children(|parent| {
                parent
                    .spawn(SpriteBundle {
                        sprite: Sprite {
                            color: board_assets.board_material.color,
                            custom_size: Some(board_size),
                            ..default()
                        },
                        texture: board_assets.board_material.texture.clone(),
                        transform: Transform::from_xyz(board_size.x / 2., board_size.y / 2., 0.),
                        ..default()
                    })
                    .insert(Name::new("Background"));

                Self::spawn_tiles(
                    parent,
                    &tile_map,
                    tile_size,
                    options.tile_padding,
                    &board_assets,
                    &mut covered_tiles,
                    &mut safe_start,
                );
            })
            .id();

        if options.safe_start {
            if let Some(entity) = safe_start {
                commands.entity(entity).insert(Uncover);
            }
        }

        #[cfg(feature = "debug")]
        log::info!("{}", tile_map.console_output());

        commands.insert_resource(Board {
            tile_map,
            tile_size,
            bounds: Bounds2 {
                position: position.truncate(),
                size: board_size,
            },
            covered_tiles,
            marked_tiles: vec![],
            entity: board_entity,
        });
    }

    #[allow(clippy::too_many_arguments)]
    fn spawn_tiles(
        parent: &mut ChildBuilder,
        tile_map: &TileMap,
        tile_size: f32,
        tile_padding: f32,
        board_assets: &BoardAssets,
        covered_tiles: &mut HashMap<Coordinates, Entity>,
        safe_start_entity: &mut Option<Entity>,
    ) {
        for (y, line) in tile_map.iter().enumerate() {
            for (x, tile) in line.iter().enumerate() {
                let mut cmd = parent.spawn_empty();
                let coordinates = Coordinates {
                    x: x as u16,
                    y: y as u16,
                };
                cmd.insert(SpriteBundle {
                    sprite: Sprite {
                        color: board_assets.tile_material.color,
                        custom_size: Some(Vec2::splat(tile_size - tile_padding)),
                        ..default()
                    },
                    texture: board_assets.tile_material.texture.clone(),
                    transform: Transform::from_xyz(
                        (x as f32 * tile_size) + (tile_size / 2.),
                        (y as f32 * tile_size) + (tile_size / 2.),
                        1.,
                    ),
                    ..default()
                })
                .insert(Name::new(format!("Tile ({x}, {y})")))
                .insert(coordinates);

                cmd.with_children(|parent| {
                    let entity = parent
                        .spawn(SpriteBundle {
                            sprite: Sprite {
                                color: board_assets.covered_tile_material.color,
                                custom_size: Some(Vec2::splat(tile_size - tile_padding)),
                                ..default()
                            },
                            transform: Transform::from_xyz(0., 0., 2.),
                            texture: board_assets.covered_tile_material.texture.clone(),
                            ..default()
                        })
                        .insert(Name::new("Tile Cover"))
                        .id();
                    covered_tiles.insert(coordinates, entity);
                    if safe_start_entity.is_none() && *tile == Tile::Empty {
                        *safe_start_entity = Some(entity);
                    }
                });

                match tile {
                    Tile::Bomb => {
                        cmd.insert(Bomb);
                        cmd.with_children(|parent| {
                            parent.spawn(SpriteBundle {
                                sprite: Sprite {
                                    custom_size: Some(Vec2::splat(tile_size - tile_padding)),
                                    ..default()
                                },
                                transform: Transform::from_xyz(0., 0., 1.),
                                texture: board_assets.bomb_material.texture.clone(),
                                ..default()
                            });
                        });
                    }
                    Tile::Neighbor(count) => {
                        cmd.insert(Neighbor { count: *count });
                        cmd.with_children(|parent| {
                            parent.spawn(Self::bomb_count_text_bundle(
                                *count,
                                board_assets,
                                tile_size - tile_padding,
                            ));
                        });
                    }
                    Tile::Empty => (),
                }
            }
        }
    }

    fn adaptive_tile_size(
        window: &Window,
        (min, max): (f32, f32),      // Tile size constraints
        (width, height): (u16, u16), // Tile map dimensions
    ) -> f32 {
        let max_w = window.resolution.width() / width as f32;
        let max_h = window.resolution.height() / height as f32;
        max_w.min(max_h).clamp(min, max)
    }

    fn bomb_count_text_bundle(count: u8, board_assets: &BoardAssets, size: f32) -> Text2dBundle {
        // retrieve the text and the correct color
        let color = board_assets.bomb_counter_color(count);

        Text2dBundle {
            text: Text {
                sections: vec![TextSection {
                    value: count.to_string(),
                    style: TextStyle {
                        color,
                        font: board_assets.bomb_counter_font.clone(),
                        font_size: size,
                    },
                }],
                alignment: TextAlignment::Center,
                ..default()
            },
            transform: Transform::from_xyz(0., 0., 1.),
            ..default()
        }
    }

    fn cleanup(board: Res<Board>, mut commands: Commands) {
        commands.entity(board.entity).despawn_recursive();
        commands.remove_resource::<Board>();
    }
}
