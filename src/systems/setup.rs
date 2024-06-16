use crate::*;
use bevy::prelude::*;
use bevy::utils::*;
use rand::prelude::*;

const TILE_SIZE: Vec2 = Vec2::new(32.0, 32.0);
const MAP_X: i32 = 80;
const MAP_Y: i32 = 50;

#[derive(Resource)]
pub struct MapTiles {
    floor_handle: Handle<Image>,
    wall_handle: Handle<Image>,
}

pub fn normalise_coordinates(x: i32, y: i32) -> (f32, f32) {
    return (x as f32 * TILE_SIZE.x, y as f32 * TILE_SIZE.y);
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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

pub fn setup_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut map_tiles: ResMut<MapTiles>,
) {
    let map_x = 80;
    let map_y = 50;

    map_tiles.floor_handle = asset_server.load("textures/floor_1.png");
    map_tiles.wall_handle = asset_server.load("textures/wall_mid.png");
    //spawn map
    let map = commands
        .spawn((
            CTile_map::new(map_x, map_y),
            TransformBundle {
                ..Default::default()
            },
        ))
        .id();
    //set everything to a wall first
    for i in 0..map_y {
        for j in 0..map_x {
            let normal_coords: (f32, f32) = normalise_coordinates(j, i);
            let child_tile = commands
                .spawn((
                    (CTile::new(TileType::Wall), CPosition::new(j, i)),
                    SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(TILE_SIZE),
                            ..default()
                        },
                        transform: Transform {
                            translation: Vec3::new(normal_coords.0, normal_coords.1, 0.0),
                            ..default()
                        },
                        texture: map_tiles.wall_handle.clone(),
                        ..default()
                    },
                ))
                .id();
            commands.entity(map).push_children(&[child_tile]);
        }
    }
}

pub fn generate_rooms(
    tile_map: &mut Query<(&CPosition, &mut CTile), Without<CPlayer>>,
    asset_server: Res<AssetServer>,
    mut assets: ResMut<Assets<Image>>,
    map_tiles: Res<MapTiles>,
) {
    let num_rooms = 8;
    let mut rooms: Vec<Rect> = Vec::new();

    while rooms.len() < num_rooms {
        let room = Rect::new(
            1. + rand::random::<f32>() * (MAP_X as f32 - 10.),
            1. + rand::random::<f32>() * (MAP_Y as f32 - 10.),
            2. + rand::random::<f32>() * (10.),
            2. + rand::random::<f32>() * (10.),
        );
        let mut overlap = false;
        for r in rooms.iter() {
            if r.contains(room.min) || r.contains(room.max) {
                overlap = true;
            }
        }
        if !overlap {
            rooms.push(room)
        }
    }

    for r in rooms.iter() {
        let (room_x1, room_y1, room_x2, room_y2) = (
            r.min.x as i32,
            r.min.y as i32,
            r.max.x as i32,
            r.max.y as i32,
        );
        for (tile_position, mut tile_type) in tile_map.iter_mut() {
            for i in room_x1..room_x2 {
                for j in room_y1..room_y2 {
                    if tile_position.coords.x == i && tile_position.coords.y == j {
                        tile_type.tile_type = TileType::Floor;
                    }
                }
            }
        }
    }
    //    for (mut tile_position,mut tile_type) in tile_map.iter_mut(){
    //
    //    }
}
