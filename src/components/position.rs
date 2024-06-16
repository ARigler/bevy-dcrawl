use bevy::prelude::*;

#[derive(Component)]
pub struct CPosition {
    pub coords: IVec2,
}

impl CPosition {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            coords: IVec2::new(x, y),
        }
    }
}
