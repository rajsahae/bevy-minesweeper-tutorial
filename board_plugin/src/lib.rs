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
    board::Board, tile::Tile, tile_map::TileMap, BoardOptions, BoardPosition, TileSize,
};
use systems::{input_handling, trigger_event_handler, uncover_tiles};

pub struct BoardPlugin<T> {
    pub running_state: T,
}

impl<T: States> Plugin for BoardPlugin<T> {
    fn build(&self, app: &mut App) {
        log::info!("Loading BoardPlugin");
        app.add_event::<TileTriggerEvent>()
            .add_system(Self::create_board.in_schedule(OnEnter(self.running_state.clone())))
            .add_systems(
                (input_handling, trigger_event_handler, uncover_tiles)
                    .in_set(OnUpdate(self.running_state.clone())),
            )
            .add_system(Self::cleanup.in_schedule(OnExit(self.running_state.clone())));
    }
}

impl<T> BoardPlugin<T> {
    fn create_board(
        mut commands: Commands,
        board_options: Option<Res<BoardOptions>>,
        query: Query<&Window, With<PrimaryWindow>>,
        asset_server: Res<AssetServer>,
    ) {
        let font: Handle<Font> = asset_server.load("fonts/pixeled.ttf");
        let bomb_image: Handle<Image> = asset_server.load("sprites/bomb.png");
        let _flag_image: Handle<Image> = asset_server.load("sprites/flag.png");

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
                            color: Color::WHITE,
                            custom_size: Some(board_size),
                            ..default()
                        },
                        transform: Transform::from_xyz(board_size.x / 2., board_size.y / 2., 0.),
                        ..default()
                    })
                    .insert(Name::new("Background"));

                Self::spawn_tiles(
                    parent,
                    &tile_map,
                    tile_size,
                    options.tile_padding,
                    Color::GRAY,
                    bomb_image,
                    font,
                    Color::DARK_GRAY,
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
            entity: board_entity,
        });
    }

    #[allow(clippy::too_many_arguments)]
    fn spawn_tiles(
        parent: &mut ChildBuilder,
        tile_map: &TileMap,
        tile_size: f32,
        tile_padding: f32,
        color: Color,
        image: Handle<Image>,
        font: Handle<Font>,
        covered_tile_color: Color,
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
                        color,
                        custom_size: Some(Vec2::splat(tile_size - tile_padding)),
                        ..default()
                    },
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
                                color: covered_tile_color,
                                custom_size: Some(Vec2::splat(tile_size - tile_padding)),
                                ..default()
                            },
                            transform: Transform::from_xyz(0., 0., 2.),
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
                                texture: image.clone(),
                                ..default()
                            });
                        });
                    }
                    Tile::Neighbor(count) => {
                        cmd.insert(Neighbor { count: *count });
                        cmd.with_children(|parent| {
                            parent.spawn(Self::bomb_count_text_bundle(
                                *count,
                                font.clone(),
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

    fn bomb_count_text_bundle(count: u8, font: Handle<Font>, size: f32) -> Text2dBundle {
        // retrieve the text and the correct color
        let (text, color) = (
            count.to_string(),
            match count {
                1 => Color::WHITE,
                2 => Color::GREEN,
                3 => Color::YELLOW,
                4 => Color::ORANGE,
                _ => Color::PURPLE,
            },
        );

        Text2dBundle {
            text: Text {
                sections: vec![TextSection {
                    value: text,
                    style: TextStyle {
                        color,
                        font,
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
