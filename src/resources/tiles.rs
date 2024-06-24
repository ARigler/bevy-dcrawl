pub use crate::*;
pub use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct RTiles {
    pub wall_handle: Handle<Image>,
    pub floor_handle: Handle<Image>,
}
