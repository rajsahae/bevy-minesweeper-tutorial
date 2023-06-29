use bevy::prelude::*;
use std::fmt::{self, Display, Formatter};
use std::ops::{Add, Sub};

#[cfg(feature = "debug")]
use bevy_inspector_egui::prelude::*;

#[cfg_attr(
    feature = "debug",
    derive(Reflect, InspectorOptions),
    reflect(InspectorOptions)
)]
#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component)]
pub struct Coordinates {
    pub x: u16,
    pub y: u16,
}

impl Add for Coordinates {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Coordinates {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x.saturating_sub(rhs.x),
            y: self.y.saturating_sub(rhs.y),
        }
    }
}

impl Display for Coordinates {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Add<(i8, i8)> for Coordinates {
    type Output = Self;

    fn add(self, rhs: (i8, i8)) -> Self::Output {
        Self {
            x: (self.x as i16 + rhs.0 as i16) as u16,
            y: (self.y as i16 + rhs.1 as i16) as u16,
        }
    }
}
