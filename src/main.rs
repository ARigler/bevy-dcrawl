use bevy::prelude::*;

pub mod bundles;
pub mod components;
pub mod plugins;
pub mod resources;
pub mod systems;

pub use components::*;
pub use resources::*;
pub use systems::*;

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
        .insert_resource(InputResource {
            input: IVec2::new(0, 0),
        })
        .add_systems(FixedUpdate, player_input)
        .add_systems(FixedUpdate, smovement.after(player_input))
        .add_systems(FixedUpdate, scamera_move.after(smovement))
        .add_systems(Startup, setup_map)
        .add_systems(Startup, generate_rooms.after(setup_map))
        .add_systems(Startup, setup)
        .run()
}
