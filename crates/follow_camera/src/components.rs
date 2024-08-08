use bevy::prelude::Component;

// Bevy marker [`Component`] for a follow camera
#[derive(Component)]
pub struct FollowCamera;

// Bevy camera list [`Component`] of camera settings
// essentially enables the player to change change between different preset
// "cameras"
#[derive(Component)]
pub struct CameraSettingsList(pub Vec<FollowCameraSettings>);

// delegate indexing to the inner Vec
impl std::ops::Index<usize> for CameraSettingsList {
    type Output = FollowCameraSettings;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl Default for CameraSettingsList {
    fn default() -> Self {
        Self(vec![FollowCameraSettings::default()])
    }
}

// Bevy settings [`Component`] for a follow camera
#[derive(Component)]
pub struct FollowCameraSettings {
    pub fov: f32,      // field of view in degrees
    pub distance: f32, // offset in meters from the center of the car
    pub height: f32,   // offset in meters from the center of the car
    pub angle: f32,    // angle in degrees relative to the car's forward direction
    pub smoothness: f32, /* contrapositive of stiffness
                        * pub transition_duration: f32, // seconds */
}

impl Default for FollowCameraSettings {
    fn default() -> Self {
        Self {
            fov: 90.0,
            distance: -5.0,
            height: 0.25,
            angle: -3.0,
            smoothness: 0.1,
            // transition_duration: 0.5,
        }
    }
}
