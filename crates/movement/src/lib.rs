pub mod components;
pub mod systems;

use bevy::prelude::*;
use systems::*;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (handle_velocity, handle_acceleration, apply_friction),
        );
    }
}
