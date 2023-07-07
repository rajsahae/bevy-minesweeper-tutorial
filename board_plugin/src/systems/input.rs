use crate::events::TileTriggerEvent;
use crate::Board;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn input_handling(
    windows: Query<&Window, With<PrimaryWindow>>,
    board: Res<Board>,
    input: Res<Input<MouseButton>>,
    mut tile_trigger: EventWriter<TileTriggerEvent>,
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
                // generate event
            }
            if input.just_pressed(MouseButton::Middle) {
                info!("hint {coordinates}");
                // generate event
            }
        }
    }
}
