use crate::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct CEnemy {
    pub delta: IVec2,
    pub timer: Timer,
}
