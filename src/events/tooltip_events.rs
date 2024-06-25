pub use crate::*;
pub use bevy::prelude::*;

#[derive(Event)]
pub struct EHideTooltip {
    pub val: bool,
}
