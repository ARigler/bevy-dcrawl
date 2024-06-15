use bevy::math::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct CTile_map {
    dimensions: UVec2,
}

impl CTile_map {
    pub fn new(x: u32, y: u32) -> Self {
        Self {
            dimensions: UVec2::new(x, y),
        }
    }
}
