use bevy::prelude::*;
use car::components::Car;

use crate::components::*;

pub fn attach_camera(
    mut commands: Commands,
    query: Query<(Entity, &Transform), (With<Car>, Without<FollowCamera>)>,
) {
    for (entity, tf) in query.iter() {
        info!("Attaching camera to car");

        let default_camera_list = CameraSettingsList::default();
        let camera_translation = tf.translation
            + tf.rotation.mul_vec3(Vec3::new(
                default_camera_list[0].distance,
                -default_camera_list[0].height,
                0.0,
            ));
        commands.entity(entity).insert((
            FollowCamera,
            CameraSettingsList::default(),
            Camera3dBundle {
                transform: Transform::from_translation(camera_translation)
                    .looking_at(tf.translation, Vec3::Y),
                // projection: PerspectiveProjection {
                //     fov: default_camera_list[0].fov.to_radians(),
                //     ..Default::default()
                // }
                // .into(),
                ..default()
            },
        ));
    }
}
