use bevy::prelude::*;

pub mod bundles;
pub mod components;
pub mod plugins;
pub mod systems;

use components::*;
use systems::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "DungeonCrawler".into(),
                        resolution: (800.0, 600.0).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(FixedUpdate, smovement)
        .add_systems(Startup, setup_map)
        .add_systems(Startup, setup)
        .run()
}
