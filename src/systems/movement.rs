use crate::*;
use bevy::prelude::*;
use rand::Rng;

pub fn smovement(
    mut player: Query<
        (&mut CPosition, &mut Transform, &mut CPlayer),
        (With<CPlayer>, Without<CTile>, Without<CEnemy>),
    >,
    mut monsters: Query<
        (&mut CPosition, &mut Transform, &mut CEnemy),
        (With<CEnemy>, Without<CTile>, Without<CPlayer>),
    >,
    map: Query<(&CPosition, &CTile), With<CTile>>,
    mut input: ResMut<InputResource>,
    time: Res<Time>,
    turn: Res<TurnState>,
) {
    match *turn {
        TurnState::PlayerTurn => {
            let player_delta = input.input;
            //check that destination tile is walkable
            for (mut char_cposition, mut transform, mut player_component) in player.iter_mut() {
                player_component.timer.tick(time.delta());
                if player_component.timer.finished() {
                    for (map_cposition, map_ctile) in map.iter() {
                        if map_cposition.coords.x == char_cposition.coords.x + player_delta.x
                            && map_cposition.coords.y == char_cposition.coords.y + player_delta.y
                        {
                            match map_ctile.tile_type {
                                TileType::Floor => {
                                    char_cposition.coords.x += player_delta.x;
                                    char_cposition.coords.y += player_delta.y;
                                    let normalised_coords = normalise_coordinates(
                                        char_cposition.coords.x,
                                        char_cposition.coords.y,
                                    );
                                    transform.translation =
                                        Vec3::new(normalised_coords.0, normalised_coords.1, 0.1);
                                    break;
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
            input.input = IVec2::new(0, 0);
        }
        TurnState::MonsterTurn => {
            for (mut monster_position, mut transform_monster, mut enemy_component) in
                monsters.iter_mut()
            {
                enemy_component.timer.tick(time.delta());
                if enemy_component.timer.finished() {
                    let monster_delta = IVec2::new(
                        rand::thread_rng().gen_range(-1..2),
                        rand::thread_rng().gen_range(-1..2),
                    );
                    for (map_cposition, map_ctile) in map.iter() {
                        if map_cposition.coords.x == monster_position.coords.x + monster_delta.x
                            && map_cposition.coords.y == monster_position.coords.y + monster_delta.y
                        {
                            match map_ctile.tile_type {
                                TileType::Floor => {
                                    monster_position.coords.x += monster_delta.x;
                                    monster_position.coords.y += monster_delta.y;
                                    let normalised_coords = normalise_coordinates(
                                        monster_position.coords.x,
                                        monster_position.coords.y,
                                    );
                                    transform_monster.translation =
                                        Vec3::new(normalised_coords.0, normalised_coords.1, 0.2);
                                }
                                TileType::Wall => {}
                            }
                        }
                    }
                }
            }
        }
        TurnState::AwaitingInput => {}
    }
}
