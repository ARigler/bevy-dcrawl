use crate::*;
use bevy::prelude::*;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    //spawn 2d camera
    let camera = commands.spawn(Camera2dBundle::default()).id();

    let map_x = 80_u32;
    let map_y = 50_u32;

    //spawn map
    let map = commands.spawn(CTile_map::new(map_x, map_y)).id();
    for i in 0..map_y {
        for j in 0..map_x {
            let child_tile = commands
                .spawn((CTile::new(TileType::Floor), CPosition::new(j, i)))
                .id();
            commands.entity(map).push_children(&[child_tile]);
        }
    }

    //spawn player
    let player = commands.spawn(CPlayer).id();
}
