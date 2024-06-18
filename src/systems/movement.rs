use crate::*;
use bevy::prelude::*;

pub fn smovement(
    mut characters: Query<
        (&mut CPosition, &mut Transform, &mut CPlayer),
        (With<CPlayer>, Without<CTile>),
    >,
    map: Query<(&CPosition, &CTile)>,
    mut input: ResMut<InputResource>,
    time: Res<Time>,
) {
    let delta = input.input;
    //check that destination tile is walkable
    for (mut char_cposition, mut transform, mut player_component) in characters.iter_mut() {
        player_component.timer.tick(time.delta());
        if player_component.timer.finished() {
            for (map_cposition, map_ctile) in map.iter() {
                if map_cposition.coords.x == char_cposition.coords.x + delta.x
                    && map_cposition.coords.y == char_cposition.coords.y + delta.y
                {
                    match map_ctile.tile_type {
                        TileType::Floor => {
                            char_cposition.coords.x += delta.x;
                            char_cposition.coords.y += delta.y;
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
