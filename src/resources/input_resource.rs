use crate::*;
use bevy::prelude::*;

#[derive(Resource)]
pub struct InputResource {
    pub input: IVec2,
}

#[derive(Resource)]
pub struct PlayerActed {
    pub acted: bool,
}

#[derive(Resource)]
pub struct MonstersActed {
    pub acted: bool,
}
