use bevy::prelude::*;
use board_plugin::BoardPlugin;

#[cfg(feature = "debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

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

    // Bevy default plugins with window setup
    app.add_plugins(DefaultPlugins.set(window));
    // startup system (cameras)
    app.add_startup_system(camera_setup);

    #[cfg(feature = "debug")]
    // Debug hiearchy inspector
    app.add_plugin(WorldInspectorPlugin::new());

    app.add_plugin(BoardPlugin);

    app.run();
}

fn camera_setup(mut commands: Commands) {
    // 2D orthographic camera
    commands.spawn(Camera2dBundle::default());
}
