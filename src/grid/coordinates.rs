use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Reflect)]
pub struct Coordinates(i32, i32);

impl Coordinates {
    pub fn from_position(x: i32, z: i32) -> Self {
        let x = x - z / 2;
        Self(x, z)
    }

    pub const fn x(&self) -> i32 {
        self.0
    }

    pub const fn y(&self) -> i32 {
        -self.x() - self.z()
    }

    pub const fn z(&self) -> i32 {
        self.1
    }
}

impl std::fmt::Display for Coordinates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n{}\n{}", self.x(), self.y(), self.z())
    }
}
