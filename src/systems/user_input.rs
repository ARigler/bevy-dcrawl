use crate::*;
use bevy::prelude::*;

pub fn player_input(
    input: Res<ButtonInput<KeyCode>>,
    mut input_res: ResMut<InputResource>,
    mut turn: ResMut<TurnState>,
) {
    let mut delta = IVec2::new(0, 0);
    if input.pressed(KeyCode::KeyW) {
        delta.y = 1
    }
    if input.pressed(KeyCode::KeyS) {
        delta.y = -1
    }
    if input.pressed(KeyCode::KeyA) {
        delta.x = -1
    }
    if input.pressed(KeyCode::KeyD) {
        delta.x = 1
    }
    match *turn {
        TurnState::AwaitingInput => {
            if delta.x != 0 || delta.y != 0 {
                *turn = TurnState::PlayerTurn
            }
        }
        _ => {}
    }
    input_res.input = delta;
}
