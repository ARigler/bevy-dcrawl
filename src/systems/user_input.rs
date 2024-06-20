use crate::*;
use bevy::prelude::*;
use rand::Rng;

pub fn player_input(
    input: Res<ButtonInput<KeyCode>>,
    creature: Query<Entity, With<CPlayer>>,
    mut acted: ResMut<PlayerActed>,
    mut ev_intent: EventWriter<MoveIntent>,
    mut ev_endturn: EventWriter<EndTurnEvent>,
) {
    let mut delta_var = IVec2::new(0, 0);

    if input.pressed(KeyCode::KeyW) {
        delta_var.y = 1;
        acted.acted = true;
    }
    if input.pressed(KeyCode::KeyS) {
        delta_var.y = -1;
        acted.acted = true;
    }
    if input.pressed(KeyCode::KeyA) {
        delta_var.x = -1;
        acted.acted = true;
    }
    if input.pressed(KeyCode::KeyD) {
        delta_var.x = 1;
        acted.acted = true;
    }
    if delta_var.x != 0 || delta_var.y != 0 {
        for p in creature.iter() {
            ev_intent.send(MoveIntent {
                entity: p,
                delta: delta_var,
            });
        }
    }
}

pub fn monster_input(
    mut creature: Query<(Entity, &mut CEnemy)>,
    mut ev_intent: EventWriter<MoveIntent>,
    time: Res<Time>,
    mut acted: ResMut<MonstersActed>,
) {
    let mut delta_var = IVec2::new(
        rand::thread_rng().gen_range(-1..2),
        rand::thread_rng().gen_range(-1..2),
    );
    if delta_var.x != 0 || delta_var.y != 0 {
        for (p, mut p_enemy) in creature.iter_mut() {
            p_enemy.timer.tick(time.delta());
            if p_enemy.timer.finished() {
                ev_intent.send(MoveIntent {
                    entity: p,
                    delta: delta_var,
                });
            }
        }
        acted.acted = true;
    }
}
