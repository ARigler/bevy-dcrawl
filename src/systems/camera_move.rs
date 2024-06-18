use crate::*;
use bevy::prelude::*;

pub fn scamera_move(
    mut camera: Query<&mut Transform, With<Camera>>,
    player: Query<&CPosition, With<CPlayer>>,
) {
    for mut c_trans in camera.iter_mut() {
        for p_pos in player.iter() {
            let new_transform = normalise_coordinates(p_pos.coords.x, p_pos.coords.y);
            c_trans.translation.x = new_transform.0;
            c_trans.translation.y = new_transform.1;
        }
    }
}
