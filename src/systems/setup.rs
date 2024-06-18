use crate::*;
use bevy::ecs::component::*;
use bevy::prelude::*;
use bevy::utils::*;
use rand::prelude::*;
use std::cmp::{max, min};

const TILE_SIZE: Vec2 = Vec2::new(32.0, 32.0);
const MAP_X: i32 = 80;
const MAP_Y: i32 = 50;

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

pub fn setup_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    let map_x = 80;
    let map_y = 50;

    let wall_handle: Handle<Image> = asset_server.load("textures/wall_mid.png");
    //spawn map
    let map = commands
        .spawn((
            CTile_map::new(map_x, map_y),
            TransformBundle {
                ..Default::default()
            },
            VisibilityBundle {
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
                        texture: wall_handle.clone(),
                        ..default()
                    },
                ))
                .id();
            commands.entity(map).push_children(&[child_tile]);
        }
    }
}

pub fn generate_rooms(
    mut tile_map: Query<(&CPosition, &mut CTile, &mut Handle<Image>), With<CTile>>,
    asset_server: Res<AssetServer>,
) {
    let num_rooms = 32;
    let mut rooms: Vec<IRect> = Vec::new();
    let floor_handle = asset_server.load("textures/floor_1.png");

    let min_room_size = 2;
    let room_size = 6;
    let margin = 10;
    while rooms.len() < num_rooms {
        let (room_x1, room_y1) = (
            1 + rand::thread_rng().gen_range(0..MAP_X - margin),
            1 + rand::thread_rng().gen_range(0..MAP_Y - margin),
        );
        let (room_x2, room_y2) = (
            min_room_size + room_x1 + rand::thread_rng().gen_range(0..room_size),
            min_room_size + room_y1 + rand::thread_rng().gen_range(0..room_size),
        );
        let room = IRect::new(room_x1, room_y1, room_x2, room_y2);
        let mut overlap = false;
        for r in rooms.iter() {
            if r.contains(room.min)
                || r.contains(room.max)
                || r.contains(IVec2::new(room.min.x, room.max.y))
                || r.contains(IVec2::new(room.min.y, room.max.x))
                || r.contains(room.center())
            {
                overlap = true;
            }
        }
        if !overlap {
            println!("room in rooms");
            rooms.push(room);
        }
    }

    let mut rooms_mut: Vec<IRect> = rooms.clone();
    for r in rooms.iter() {
        let (room_x1, room_y1, room_x2, room_y2) = (r.min.x, r.min.y, r.max.x, r.max.y);
        println!("{} {} {} {}", room_x1, room_y1, room_x2, room_y2);
        //build each individual room
        for (tile_position, mut tile_type, mut handle) in tile_map.iter_mut() {
            for i in room_x1..room_x2 {
                for j in room_y1..room_y2 {
                    if tile_position.coords.x == i && tile_position.coords.y == j {
                        tile_type.tile_type = TileType::Floor;
                        *handle = floor_handle.clone();
                    }
                }
            }
        }
    }

    //sort rooms by x coords
    rooms_mut.sort_by(|a, b| a.center().x.cmp(&b.center().x));

    for (i, r) in rooms_mut.iter().enumerate().skip(1) {
        let prev = rooms_mut[i - 1].center();
        let new = r.center();
        if rand::thread_rng().gen_range(0..2) == 1 {
            //horizontal tunnels
            for x in min(prev.x, new.x)..=max(prev.x, new.x) {
                for (tile_position, mut tile_type, mut handle) in tile_map.iter_mut() {
                    if tile_position.coords.x == x && tile_position.coords.y == prev.y {
                        tile_type.tile_type = TileType::Floor;
                        *handle = floor_handle.clone();
                    }
                }
            }
            for y in min(prev.y, new.y)..=max(prev.y, new.y) {
                for (tile_position, mut tile_type, mut handle) in tile_map.iter_mut() {
                    if tile_position.coords.y == y && tile_position.coords.x == prev.x {
                        tile_type.tile_type = TileType::Floor;
                        *handle = floor_handle.clone();
                    }
                }
            }
            //vertical tunnels
        } else {
            for y in min(prev.y, new.y)..=max(prev.y, new.y) {
                for (tile_position, mut tile_type, mut handle) in tile_map.iter_mut() {
                    if tile_position.coords.y == y && tile_position.coords.x == prev.x {
                        tile_type.tile_type = TileType::Floor;
                        *handle = floor_handle.clone();
                    }
                }
            }
            for x in min(prev.x, new.x)..=max(prev.x, new.x) {
                for (tile_position, mut tile_type, mut handle) in tile_map.iter_mut() {
                    if tile_position.coords.x == x && tile_position.coords.y == prev.y {
                        tile_type.tile_type = TileType::Floor;
                        *handle = floor_handle.clone();
                    }
                }
            }
        }
    }
}
