mod components;
mod systems;

use bevy::prelude::*;

pub struct FollowCameraPlugin;

impl Plugin for FollowCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, systems::attach_camera);
    }
}
