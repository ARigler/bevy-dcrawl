use crate::*;
use bevy::ecs::component::*;
use bevy::prelude::*;
use bevy::utils::*;
use rand::prelude::*;
use std::cmp::{max, min};

const TILE_SIZE: Vec2 = Vec2::new(32.0, 32.0);
const MAP_X: i32 = 160;
const MAP_Y: i32 = 100;

pub fn normalise_coordinates(x: i32, y: i32) -> (f32, f32) {
    return (x as f32 * TILE_SIZE.x, y as f32 * TILE_SIZE.y);
}

pub fn populate_resources(
    asset_server: Res<AssetServer>,
    mut tile_handles: ResMut<RTiles>,
    mut creature_handles: ResMut<RCreatures>,
) {
    tile_handles.wall_handle = asset_server.load("textures/wall_mid.png");
    tile_handles.floor_handle = asset_server.load("textures/floor_1.png");

    creature_handles.player_handle = asset_server.load("textures/knight_f_idle_anim_f0.png");
    creature_handles.orc_handle = asset_server.load("textures/orc_warrior_idle_anim_f0.png");
    creature_handles.goblin_handle = asset_server.load("textures/goblin_idle_anim_f0.png");
    creature_handles.ogre_handle = asset_server.load("textures/ogre_idle_anim_f0.png");
}

pub fn setup(
    mut commands: Commands,
    space_query: Query<(&CTile, &CPosition)>,
    creature_handles: Res<RCreatures>,
    asset_server: Res<AssetServer>,
) {
    let mut coord_vec: Vec<IVec2> = Vec::new();
    while coord_vec.len() < 11 {
        let (random_x, random_y) = (
            rand::thread_rng().gen_range(1..MAP_X),
            rand::thread_rng().gen_range(1..MAP_Y),
        );

        for (c_tile, c_position) in space_query.iter() {
            if c_position.coords.x == random_x && c_position.coords.y == random_y {
                match c_tile.tile_type {
                    TileType::Floor => {
                        coord_vec.push(IVec2::new(c_position.coords.x, c_position.coords.y))
                    }
                    TileType::Wall => {}
                }
            }
        }
    }

    let player_position = coord_vec[0];
    spawn_player(&mut commands, &creature_handles, player_position);
    spawn_camera(&mut commands, player_position);
    for i in 0..1 {
        spawn_monster(&mut commands, &creature_handles, coord_vec[i + 1]);
    }
    spawn_health_hud(&mut commands, &asset_server, 20, 20);
    init_tooltips(&mut commands, &asset_server);
    commands.insert_resource(TurnState::PlayerTurn);
}

pub fn setup_map(mut commands: Commands, tile_handles: Res<RTiles>) {
    let wall_handle: Handle<Image> = tile_handles.wall_handle.clone();
    //spawn map
    let map = commands
        .spawn((
            CTile_map::new(MAP_X, MAP_Y),
            TransformBundle {
                ..Default::default()
            },
            VisibilityBundle {
                ..Default::default()
            },
        ))
        .id();
    //set everything to a wall first
    for i in 0..MAP_Y {
        for j in 0..MAP_X {
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
    tile_handles: Res<RTiles>,
) {
    let num_rooms = 64;
    let mut rooms: Vec<IRect> = Vec::new();

    let min_room_size = 3;
    let room_size = 6;
    let margin = 10;
    while rooms.len() < num_rooms {
        let (room_x1, room_y1) = (
            rand::thread_rng().gen_range(0..MAP_X - margin),
            rand::thread_rng().gen_range(0..MAP_Y - margin),
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
            //println!("room in rooms");
            rooms.push(room);
        }
    }

    for r in rooms.iter() {
        let (room_x1, room_y1, room_x2, room_y2) = (r.min.x, r.min.y, r.max.x, r.max.y);
        //println!("{} {} {} {}", room_x1, room_y1, room_x2, room_y2);
        //build each individual room
        for (tile_position, mut tile_type, mut handle) in tile_map.iter_mut() {
            for i in room_x1..room_x2 {
                for j in room_y1..room_y2 {
                    if tile_position.coords.x == i && tile_position.coords.y == j {
                        tile_type.tile_type = TileType::Floor;
                        *handle = tile_handles.floor_handle.clone();
                    }
                }
            }
        }
    }

    let mut rooms_mut: Vec<IRect> = rooms.clone();
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
                        *handle = tile_handles.floor_handle.clone();
                    }
                }
            }
            for y in min(prev.y, new.y)..=max(prev.y, new.y) {
                for (tile_position, mut tile_type, mut handle) in tile_map.iter_mut() {
                    if tile_position.coords.y == y && tile_position.coords.x == new.x {
                        tile_type.tile_type = TileType::Floor;
                        *handle = tile_handles.floor_handle.clone();
                    }
                }
            }
            //vertical tunnels
        } else {
            for y in min(prev.y, new.y)..=max(prev.y, new.y) {
                for (tile_position, mut tile_type, mut handle) in tile_map.iter_mut() {
                    if tile_position.coords.y == y && tile_position.coords.x == prev.x {
                        tile_type.tile_type = TileType::Floor;
                        *handle = tile_handles.floor_handle.clone();
                    }
                }
            }
            for x in min(prev.x, new.x)..=max(prev.x, new.x) {
                for (tile_position, mut tile_type, mut handle) in tile_map.iter_mut() {
                    if tile_position.coords.x == x && tile_position.coords.y == new.y {
                        tile_type.tile_type = TileType::Floor;
                        *handle = tile_handles.floor_handle.clone();
                    }
                }
            }
        }
    }
}

pub fn init_tooltips(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let mut tooltip_text = commands.spawn((
        TextBundle::from_section(
            "Tooltip text",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Light.ttf"),
                font_size: 20.0,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            ..default()
        }),
        CTooltipText,
    ));
}
