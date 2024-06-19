use crate::*;
use bevy::prelude::*;

pub fn send_turn(mut turn: ResMut<TurnState>) {
    let new_state = match *turn {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
    };
    *turn = new_state;
}
