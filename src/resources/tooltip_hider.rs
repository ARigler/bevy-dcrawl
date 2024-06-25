pub use crate::*;
pub use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct RTooltipHidden {
    pub value: bool,
}
