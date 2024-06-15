use bevy::prelude::*;

#[derive(Component)]
pub struct CPosition {
    coords: UVec2,
}

impl CPosition {
    pub fn new(x: u32, y: u32) -> Self {
        Self {
            coords: UVec2::new(x, y),
        }
    }
}
