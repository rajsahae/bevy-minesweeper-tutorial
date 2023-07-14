use bevy::prelude::*;
use board_plugin::{resources::BoardOptions, BoardPlugin};

// #[cfg(feature = "debug")]
// use bevy_inspector_egui::quick::WorldInspectorPlugin;

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, States)]
pub enum AppState {
    #[default]
    InGame,
    Out,
    Paused,
    Reload,
}

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
            safe_start: true,
            ..default()
        })
        .add_state::<AppState>()
        .add_plugin(BoardPlugin {
            running_state: AppState::InGame,
        })
        .add_system(state_handler)
        .run();
}

fn camera_setup(mut commands: Commands) {
    // 2D orthographic camera
    commands.spawn(Camera2dBundle::default());
}

fn state_handler(
    current: Res<State<AppState>>,
    mut next: ResMut<NextState<AppState>>,
    keys: Res<Input<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::C) {
        debug!("clearing detected");
        if current.0 == AppState::InGame {
            info!("clearing game");
            next.set(AppState::Out);
        }
    }
    if keys.just_pressed(KeyCode::G) {
        debug!("loading detected");
        if current.0 == AppState::InGame {
            info!("clear then reload");
            next.set(AppState::Reload);
        }
        if current.0 == AppState::Out {
            info!("loading game");
            next.set(AppState::InGame);
        }
    }

    // TODO: this resets the board because it exits/enters states
    // Can we find a way to simply disable input_handling system?

    // if keys.just_pressed(KeyCode::Escape) {
    //     debug!("toggle pause");
    //     if current.0 == AppState::InGame {
    //         info!("pausing");
    //         next.set(AppState::Paused);
    //     } else {
    //         info!("unpausing");
    //         next.set(AppState::InGame);
    //     }
    // }

    if current.0 == AppState::Reload {
        info!("reloading");
        next.set(AppState::InGame);
    }
}
