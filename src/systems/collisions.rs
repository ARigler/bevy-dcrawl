use crate::*;
use bevy::prelude::*;

pub fn scollisions(
    mut commands: Commands,
    player_entity: Query<&CPosition, With<CPlayer>>,
    monster_query: Query<(Entity, &CPosition), With<CEnemy>>,
) {
    for player_position in player_entity.iter() {
        for (monster, monster_position) in monster_query.iter() {
            if player_position.coords == monster_position.coords {
                commands.entity(monster).despawn();
            }
        }
    }
}
