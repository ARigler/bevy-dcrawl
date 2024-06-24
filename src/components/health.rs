pub use crate::*;
pub use bevy::prelude::*;

#[derive(Component)]
pub struct CHealth {
    pub current: i32,
    pub max: i32,
}
