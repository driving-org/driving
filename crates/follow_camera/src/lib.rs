mod components;
mod systems;

use bevy::prelude::*;
use systems::*;

pub struct FollowCameraPlugin;

impl Plugin for FollowCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (attach_camera, follow_car));
    }
}
