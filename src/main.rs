use bevy::prelude::*;

pub mod bundles;
pub mod components;
pub mod events;
pub mod plugins;
pub mod resources;
pub mod systems;

pub use components::*;
pub use events::*;
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
        .insert_resource(PlayerActed { acted: false })
        .insert_resource(MonstersActed { acted: false })
        .init_resource::<RCreatures>()
        .init_resource::<RTiles>()
        .init_resource::<RTooltipHidden>()
        .add_event::<MoveIntent>()
        .add_event::<EndTurnEvent>()
        .add_event::<EHideTooltip>()
        .add_systems(FixedUpdate, player_input)
        .add_systems(FixedUpdate, monster_input.after(player_input))
        .add_systems(FixedUpdate, smovement.after(player_input))
        .add_systems(FixedUpdate, scamera_move.after(smovement))
        .add_systems(Startup, populate_resources)
        .add_systems(Startup, setup_map.after(populate_resources))
        .add_systems(Startup, generate_rooms.after(setup_map))
        .add_systems(Startup, setup.after(generate_rooms))
        //        .add_systems(FixedUpdate, scollisions.after(smovement))
        .add_systems(Update, send_turn)
        .add_systems(Update, health_ui_update)
        .add_systems(FixedUpdate, tooltips.after(smovement))
        .add_systems(Update, tooltip_hidden_manager)
        .run()
}
