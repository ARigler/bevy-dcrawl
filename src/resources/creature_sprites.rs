pub use crate::*;
pub use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct RCreatures {
    pub player_handle: Handle<Image>,
    pub orc_handle: Handle<Image>,
    pub goblin_handle: Handle<Image>,
    pub ogre_handle: Handle<Image>,
}
