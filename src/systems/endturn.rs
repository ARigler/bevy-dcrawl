use crate::*;
use bevy::prelude::*;

pub fn send_turn(
    mut turn: ResMut<TurnState>,
    mut ev_endturn: EventReader<EndTurnEvent>,
    mut player_acted: ResMut<PlayerActed>,
    mut monsters_acted: ResMut<MonstersActed>,
) {
    let mut new_state = TurnState::PlayerTurn;
    match *turn {
        TurnState::PlayerTurn => {
            if player_acted.acted {
                new_state = TurnState::MonsterTurn;
                player_acted.acted = false;
            } else {
                new_state = TurnState::PlayerTurn
            }
        }
        TurnState::MonsterTurn => {
            if monsters_acted.acted {
                new_state = TurnState::PlayerTurn;
                monsters_acted.acted = false;
            } else {
                new_state = TurnState::MonsterTurn
            }
        }
    };
    *turn = new_state;
}
