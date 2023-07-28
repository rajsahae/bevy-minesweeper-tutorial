use crate::components::Coordinates;
use bevy::prelude::Event;

#[derive(Debug, Copy, Clone, Event)]
pub struct TileTriggerEvent(pub Coordinates);

#[derive(Debug, Copy, Clone, Event)]
pub struct TileMarkEvent(pub Coordinates);

#[derive(Debug, Copy, Clone, Event)]
pub struct BoardCompletedEvent;

#[derive(Debug, Copy, Clone, Event)]
pub struct BombExplosionEvent;
