use bevy::math::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct CTile_map {
    dimensions: IVec2,
}

impl CTile_map {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            dimensions: IVec2::new(x, y),
        }
    }
}
