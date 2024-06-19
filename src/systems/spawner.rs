use crate::*;
use bevy::prelude::*;
use bevy::utils::*;

pub fn spawn_monster(commands: &mut Commands, asset_server: &Res<AssetServer>, pos: IVec2) {
    let normal_coords = normalise_coordinates(pos.x, pos.y);
    let enemy_texture = asset_server.load("textures/ogre_idle_anim_f0.png");

    commands.spawn((
        CEnemy {
            delta: IVec2::new(0, 0),
            timer: Timer::new(Duration::from_millis(50), TimerMode::Repeating),
        },
        CPosition::new(pos.x, pos.y),
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(32.0, 36.0)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(normal_coords.0, normal_coords.1, 0.1),
                ..default()
            },
            texture: enemy_texture,
            ..default()
        },
    ));
}

pub fn spawn_player(commands: &mut Commands, asset_server: &Res<AssetServer>, pos: IVec2) {
    let player_texture = asset_server.load("textures/knight_f_idle_anim_f0.png");
    let normal_coords: (f32, f32) = normalise_coordinates(pos.x, pos.y);

    let player = commands
        .spawn((
            CPlayer {
                timer: Timer::new(Duration::from_millis(50), TimerMode::Repeating),
            },
            CPosition::new(pos.x, pos.y),
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
}

pub fn spawn_camera(commands: &mut Commands, pos: IVec2) {
    let normal_coords: (f32, f32) = normalise_coordinates(pos.x, pos.y);
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
