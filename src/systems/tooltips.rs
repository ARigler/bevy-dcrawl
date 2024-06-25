pub use crate::*;
use bevy::log::tracing_subscriber::filter::combinator::And;
pub use bevy::prelude::*;
pub use bevy::window::PrimaryWindow;

pub fn tooltips(
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_positions: Query<(Entity, &Transform, &CName), (Without<CTooltipText>, Without<Camera2d>)>,
    mut q_tooltip_text: Query<
        (&mut Text, &mut Transform, &mut Style, &mut Visibility),
        With<CTooltipText>,
    >,
    q_camera: Query<&Transform, (With<Camera2d>, Without<CTooltipText>)>,
    asset_server: Res<AssetServer>,
    tooltip_hidden: Res<RTooltipHidden>,
    mut tooltip_hider: EventWriter<EHideTooltip>,
) {
    let window = q_windows.single();
    let width = window.resolution.width();
    let height = window.resolution.height();
    if let Some(position) = window.cursor_position() {
        let camera = q_camera.single();
        let (position_rel_x, position_rel_y) =
            (position.x - (width / 2.0), position.y - (height / 2.0));
        let (offset_x, offset_y) = (
            camera.translation.x - (width / 2.0),
            camera.translation.y - (height / 2.0),
        );
        let (mut ttext, mut tposition, mut tstyle, mut tvis) = q_tooltip_text.single_mut();
        for (ent, trans, name) in q_positions.iter() {
            let (startpos_x, endpos_x, startpos_y, endpos_y) = (
                (trans.translation.x - offset_x) - 16.0,
                (trans.translation.x - offset_x) + 16.0,
                (trans.translation.y - offset_y
                    + 2.0 * (camera.translation.y - trans.translation.y))
                    - 16.0,
                (trans.translation.y - offset_y
                    + 2.0 * (camera.translation.y - trans.translation.y))
                    + 16.0,
            );
            //if cursor is intersecting with the entity's sprite, show a tooltip
            if (startpos_x <= position.x && position.x <= endpos_x)
                && (startpos_y <= position.y && position.y <= endpos_y)
            {
                tooltip_hider.send(EHideTooltip { val: false });
                //tposition.translation.x = trans.translation.x + position_rel_x;
                //tposition.translation.y = trans.translation.y + position_rel_y;
                *ttext = Text::from_section(
                    &name.value,
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Light.ttf"),
                        font_size: 20.0,
                        ..default()
                    },
                );
                *tstyle = Style {
                    left: Val::Px(position.x),
                    top: Val::Px(position.y),
                    display: Display::Flex,
                    ..default()
                };
                //*tvis = Visibility::Visible;
            }
            // this condition is supposed to be "If cursor is not intersecting", but it's getting triggered anyway and I'm confused
            if (startpos_x > position.x || endpos_x < position.x)
                && (startpos_y > position.y || endpos_y < position.y)
                && !tooltip_hidden.value
            {
                tooltip_hider.send(EHideTooltip { val: true });
            }
            //*tvis = Visibility::Hidden;
        }
    }
}

pub fn tooltip_hidden_manager(
    mut tooltip_hidden: ResMut<RTooltipHidden>,
    mut tooltip_hides: EventReader<EHideTooltip>,
    mut q_tooltip_text: Query<&mut Style, With<CTooltipText>>,
) {
    for ev in tooltip_hides.read() {
        if ev.val == true {
            for mut tstyle in q_tooltip_text.iter_mut() {
                *tstyle = Style {
                    display: Display::None,
                    ..default()
                };
            }
            tooltip_hidden.value = true;
        } else {
            tooltip_hidden.value = false;
        }
    }
}
