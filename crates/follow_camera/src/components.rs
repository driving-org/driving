use bevy::prelude::{Component, Entity};

// Bevy marker [`Component`] for a follow camera
#[derive(Component, Debug)]
pub struct FollowCamera {
    pub target: Option<Entity>,
}

impl Default for FollowCamera {
    fn default() -> Self {
        Self { target: None }
    }
}

impl FollowCamera {
    pub fn new(target: Entity) -> Self {
        Self {
            target: Some(target),
        }
    }
}

// Bevy camera list [`Component`] of camera settings
// essentially enables the player to change change between different preset
// "cameras"
#[derive(Component, Debug)]
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
#[derive(Component, Debug)]
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
            fov: 70.0,
            distance: -2.0,
            height: 0.75,
            angle: -3.0,
            smoothness: 0.1,
            // transition_duration: 0.5,
        }
    }
}
