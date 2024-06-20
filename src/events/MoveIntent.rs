use crate::*;
use bevy::prelude::*;

#[derive(Event)]
pub struct MoveIntent {
    pub entity: Entity,
    pub delta: IVec2,
}
