use bevy::prelude::*;

pub enum TileType {
    Floor,
    Wall,
}

#[derive(Component)]
pub struct CTile {
    tile_type: TileType,
}

impl CTile {
    pub fn new(initial_type: TileType) -> Self {
        Self {
            tile_type: initial_type,
        }
    }
}
