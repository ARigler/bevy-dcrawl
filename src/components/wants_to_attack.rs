pub use crate::*;
pub use bevy::prelude::*;

#[derive(Component)]
pub struct WantsToAttack {
    pub attacker: Entity,
    pub victim: Entity,
}
