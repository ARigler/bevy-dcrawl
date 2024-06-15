use bevy::prelude::*;

pub mod bundles;
pub mod components;
pub mod plugins;
pub mod systems;

use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Startup, setup)
        .run()
}
