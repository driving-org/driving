pub mod components;
pub mod systems;

use bevy::prelude::*;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                systems::handle_velocity,
                // systems::handle_acceleration
            ),
        );
    }
}
