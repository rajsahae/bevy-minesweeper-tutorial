use crate::events::{TileMarkEvent, TileTriggerEvent};
use crate::Board;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn input_handling(
    windows: Query<&Window, With<PrimaryWindow>>,
    board: Res<Board>,
    input: Res<Input<MouseButton>>,
    mut tile_trigger: EventWriter<TileTriggerEvent>,
    mut mark_trigger: EventWriter<TileMarkEvent>,
) {
    let window = windows.single();

    if let Some(pos) = window.cursor_position() {
        if let Some(coordinates) = board.mouse_position(window, pos) {
            if input.just_pressed(MouseButton::Left) {
                info!("uncover {coordinates}");
                tile_trigger.send(TileTriggerEvent(coordinates));
            }
            if input.just_pressed(MouseButton::Right) {
                info!("mark {coordinates}");
                mark_trigger.send(TileMarkEvent(coordinates));
            }
            // if input.just_pressed(MouseButton::Middle) {
            //     info!("hint {coordinates}");
            //     // generate event
            // }
        }
    }
}
