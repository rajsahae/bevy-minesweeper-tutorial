use crate::{events::TileTriggerEvent, Board, Bomb, Coordinates, Neighbor, Uncover};
use bevy::prelude::*;

pub fn trigger_event_handler(
    mut commands: Commands,
    board: Res<Board>,
    mut tile_trigger: EventReader<TileTriggerEvent>,
) {
    for trigger_event in tile_trigger.iter() {
        if let Some(entity) = board.tile_to_uncover(&trigger_event.0) {
            commands.entity(*entity).insert(Uncover);
        }
    }
}

pub fn uncover_tiles(
    mut command: Commands,
    mut board: ResMut<Board>,
    children: Query<(Entity, &Parent), With<Uncover>>,
    parents: Query<(&Coordinates, Option<&Bomb>, Option<&Neighbor>)>,
) {
    for (entity, parent) in children.iter() {
        // destroy the tile cover
        command.entity(entity).despawn_recursive();

        let (coords, bomb, bomb_counter) = match parents.get(parent.get()) {
            Ok(v) => v,
            Err(e) => {
                error!("{e}");
                continue;
            }
        };

        match board.try_uncover_tile(coords) {
            Some(e) => debug!("Uncovered tile {coords} (entity: {e:?}"),
            None => debug!("attempted to uncover a tile already uncovered: {coords}"),
        }

        if bomb.is_some() {
            info!("Boom!");
            // explosion event
        } else if bomb_counter.is_none() {
            // propogate Uncover to neighbor tiles
            for entity in board.adjacent_covered_tiles(coords) {
                command.entity(entity).insert(Uncover);
            }
        }
    }
}
