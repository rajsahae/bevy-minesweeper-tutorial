use bevy::prelude::Component;

#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Component)]
pub struct Neighbor {
    pub count: u8,
}
