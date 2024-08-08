pub mod components;
mod systems;

use bevy::prelude::*;

pub struct CarPlugin;

impl Plugin for CarPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(movement::MovementPlugin)
            .add_systems(Startup, systems::spawn_car)
            .add_systems(Update, systems::input);
    }
}
