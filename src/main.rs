use bevy::{app::AppExit, prelude::*};
use board_plugin::{
    resources::{BoardAssets, BoardOptions, SpriteMaterial},
    BoardPlugin,
};

// #[cfg(feature = "debug")]
// use bevy_inspector_egui::quick::WorldInspectorPlugin;

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, States)]
pub enum AppState {
    #[default]
    Load,
    InGame,
    Paused,
    Out,
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
        .add_startup_system(setup_camera)
        .add_startup_system(setup_board)
        .add_state::<AppState>()
        .add_plugin(BoardPlugin {
            start_state: AppState::Load,
            running_state: AppState::InGame,
            end_state: AppState::Out,
        })
        .add_system(state_handler)
        .run();
}

fn setup_camera(mut commands: Commands) {
    // 2D orthographic camera
    commands.spawn(Camera2dBundle::default());
}

fn setup_board(
    mut commands: Commands,
    mut next: ResMut<NextState<AppState>>,
    asset_server: Res<AssetServer>,
) {
    // options
    commands.insert_resource(BoardOptions {
        map_size: (20, 20),
        bomb_count: 40,
        tile_padding: 3.0,
        safe_start: true,
        ..default()
    });

    // assets
    commands.insert_resource(BoardAssets {
        label: "Default".to_string(),
        board_material: SpriteMaterial {
            color: Color::WHITE,
            ..default()
        },
        tile_material: SpriteMaterial {
            color: Color::DARK_GRAY,
            ..default()
        },
        covered_tile_material: SpriteMaterial {
            color: Color::GRAY,
            ..default()
        },
        flag_material: SpriteMaterial {
            color: Color::WHITE,
            texture: asset_server.load("sprites/flag.png"),
        },
        bomb_material: SpriteMaterial {
            color: Color::WHITE,
            texture: asset_server.load("sprites/bomb.png"),
        },
        bomb_counter_font: asset_server.load("fonts/pixeled.ttf"),
        bomb_counter_colors: BoardAssets::default_colors(),
    });

    // activate plugin
    next.set(AppState::InGame);
}

fn state_handler(
    current: Res<State<AppState>>,
    mut next: ResMut<NextState<AppState>>,
    keys: Res<Input<KeyCode>>,
    mut exit: EventWriter<AppExit>,
) {
    if keys.just_pressed(KeyCode::C) {
        if current.0 != AppState::Out {
            info!("clearing game");
            next.set(AppState::Out);
        }
    }
    if keys.just_pressed(KeyCode::L) {
        if current.0 == AppState::Out {
            info!("loading game");
            next.set(AppState::Load);
        }
    }

    if keys.just_pressed(KeyCode::Escape)
        || keys.just_pressed(KeyCode::Pause)
        || keys.just_pressed(KeyCode::P)
    {
        debug!("toggle pause");
        if current.0 == AppState::InGame {
            info!("pausing");
            next.set(AppState::Paused);
        } else if current.0 == AppState::Paused {
            info!("unpausing");
            next.set(AppState::InGame);
        }
    }

    if keys.just_pressed(KeyCode::Q) {
        info!("exit");
        exit.send_default();
    }

    if current.0 == AppState::Load {
        info!("starting game");
        next.set(AppState::InGame);
    }
}
