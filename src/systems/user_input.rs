use crate::*;
use bevy::prelude::*;

pub fn player_input(input: Res<ButtonInput<KeyCode>>, mut input_res: ResMut<InputResource>) {
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
    input_res.input = delta;
}
