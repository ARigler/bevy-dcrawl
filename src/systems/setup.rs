use crate::*;
use bevy::prelude::*;
use bevy::utils::*;

const TILE_SIZE: Vec2 = Vec2::new(32.0, 32.0);

pub fn normalise_coordinates(x: i32, y: i32) -> (f32, f32) {
    return (x as f32 * TILE_SIZE.x, y as f32 * TILE_SIZE.y);
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let map_x = 80;
    let map_y = 50;

    let floor_texture = asset_server.load("textures/floor_1.png");
    //spawn map
    let map = commands
        .spawn((
            CTile_map::new(map_x, map_y),
            TransformBundle {
                ..Default::default()
            },
        ))
        .id();
    for i in 0..map_y {
        for j in 0..map_x {
            let normal_coords: (f32, f32) = normalise_coordinates(j, i);
            let child_tile = commands
                .spawn((
                    (CTile::new(TileType::Floor), CPosition::new(j, i)),
                    SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(TILE_SIZE),
                            ..default()
                        },
                        transform: Transform {
                            translation: Vec3::new(normal_coords.0, normal_coords.1, 0.0),
                            ..default()
                        },
                        texture: floor_texture.clone(),
                        ..default()
                    },
                ))
                .id();
            commands.entity(map).push_children(&[child_tile]);
        }
    }

    let player_texture = asset_server.load("textures/knight_f_idle_anim_f0.png");
    //spawn player
    let normal_coords: (f32, f32) = normalise_coordinates(40, 25);
    let player = commands
        .spawn((
            CPlayer {
                timer: Timer::new(Duration::from_millis(50), TimerMode::Repeating),
            },
            CPosition::new(40, 25),
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(16.0, 32.0)),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(normal_coords.0, normal_coords.1, 0.1),
                    ..default()
                },
                texture: player_texture,
                ..default()
            },
        ))
        .id();

    //spawn 2d camera
    let camera = commands
        .spawn(Camera2dBundle {
            transform: Transform {
                translation: Vec3::new(normal_coords.0, normal_coords.1, 0.0),
                ..default()
            },
            ..default()
        })
        .id();
}
