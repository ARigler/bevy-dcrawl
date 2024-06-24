pub use crate::*;
use bevy::log::tracing_subscriber::filter::combinator::And;
pub use bevy::prelude::*;
pub use bevy::window::PrimaryWindow;

pub fn tooltips(
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_positions: Query<(Entity, &Transform, &CName), (Without<CTooltipText>, Without<Camera2d>)>,
    mut q_tooltip_text: Query<(&mut Text, &mut Transform), With<CTooltipText>>,
    q_camera: Query<&Transform, (With<Camera2d>, Without<CTooltipText>)>,
    asset_server: Res<AssetServer>,
) {
    let window = q_windows.single();
    let width = window.resolution.width();
    let height = window.resolution.height();
    if let Some(position) = window.cursor_position() {
        let camera = q_camera.single();
        let (mut ttext, mut tposition) = q_tooltip_text.single_mut();
        for (ent, trans, name) in q_positions.iter() {
            println!(
                "{:?}, {:?}, {:?}",
                trans.translation, camera.translation, position
            );
            if camera.translation.x - trans.translation.x == (width / 2.0) - position.x
                && camera.translation.y - trans.translation.y == (height / 2.0) - position.y
            {
                tposition.translation.x = trans.translation.x;
                tposition.translation.y = trans.translation.y;
                *ttext = Text::from_section(
                    &name.value,
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Light.ttf"),
                        font_size: 20.0,
                        ..default()
                    },
                );
            }
        }
    }
}
