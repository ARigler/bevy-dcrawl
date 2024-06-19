use crate::*;
use bevy::prelude::*;

#[derive(Resource)]
pub enum TurnState {
    AwaitingInput,
    PlayerTurn,
    MonsterTurn,
}
