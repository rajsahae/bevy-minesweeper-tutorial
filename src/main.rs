use bevy::prelude::*;
use board_plugin::{resources::BoardOptions, BoardPlugin};

// #[cfg(feature = "debug")]
// use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    let window = WindowPlugin {
        primary_window: Some(Window {
            title: "Mine Sweeper!".to_string(),
            resolution: (700., 800.).into(),
            ..Default::default()
        }),
        ..Default::default()
    };

    let mut app = App::new();

    // Debug hiearchy inspector
    // #[cfg(feature = "debug")]
    // app.add_plugin(WorldInspectorPlugin::new());

    // Bevy default plugins with window setup
    app.add_plugins(DefaultPlugins.set(window))
        // startup system (cameras)
        .add_startup_system(camera_setup)
        .insert_resource(BoardOptions {
            map_size: (20, 20),
            bomb_count: 40,
            tile_padding: 3.0,
            ..default()
        })
        .add_plugin(BoardPlugin)
        .run();
}

fn camera_setup(mut commands: Commands) {
    // 2D orthographic camera
    commands.spawn(Camera2dBundle::default());
}
