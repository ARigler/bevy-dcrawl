pub use crate::*;
pub use bevy::prelude::*;

pub fn health_ui_update(
    player: Query<&CHealth, With<CPlayer>>,
    mut hud: Query<&mut Text, With<CHealthText>>,
) {
    let (mut health_val, mut health_max): (i32, i32) = (0, 0);
    for health in player.iter() {
        health_val = health.current;
        health_max = health.max;
    }

    for mut text in hud.iter_mut() {
        text.sections[0].value =
            "HP: ".to_string() + &health_val.to_string() + " / " + &health_max.to_string();
    }
}
