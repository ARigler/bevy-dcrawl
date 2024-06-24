use crate::*;
use bevy::prelude::*;
use bevy::utils::*;
use rand::Rng;

pub fn spawn_monster(commands: &mut Commands, creature_handles: &Res<RCreatures>, pos: IVec2) {
    let normal_coords = normalise_coordinates(pos.x, pos.y);
    let mut enemy_details = (0, "".to_string(), creature_handles.orc_handle.clone());
    let random_number = rand::thread_rng().gen_range(0..3);
    if random_number == 0 {
        enemy_details = goblin(creature_handles);
    }
    if random_number == 1 {
        enemy_details = orc(creature_handles);
    }
    if random_number == 2 {
        enemy_details = ogre(creature_handles);
    }

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
            texture: enemy_details.2,
            ..default()
        },
        CHealth {
            current: enemy_details.0,
            max: enemy_details.0,
        },
        CName {
            value: enemy_details.1,
        },
    ));
}

pub fn goblin(creature_handles: &Res<RCreatures>) -> (i32, String, Handle<Image>) {
    (
        1,
        "Goblin".to_string(),
        creature_handles.goblin_handle.clone(),
    )
}

pub fn orc(creature_handles: &Res<RCreatures>) -> (i32, String, Handle<Image>) {
    (2, "Orc".to_string(), creature_handles.orc_handle.clone())
}

pub fn ogre(creature_handles: &Res<RCreatures>) -> (i32, String, Handle<Image>) {
    (3, "Ogre".to_string(), creature_handles.ogre_handle.clone())
}

pub fn spawn_player(commands: &mut Commands, creature_handles: &Res<RCreatures>, pos: IVec2) {
    let player_texture = creature_handles.player_handle.clone();
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
            CHealth {
                current: 20,
                max: 20,
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

pub fn spawn_health_hud(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    player_health: i32,
    player_max: i32,
) {
    let p_health_str =
        "HP: ".to_string() + &player_health.to_string() + " / " + &player_max.to_string();
    commands.spawn((
        TextBundle::from_section(
            p_health_str,
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 40.0,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(20.0),
            left: Val::Percent(40.0),
            ..default()
        }),
        CHealthText,
    ));
}
