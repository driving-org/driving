use bevy::prelude::*;
use car::components::Car;

use crate::components::*;

/// Bevy [`Update`] [`System`] to attach a [`FollowCamera`] to every new car in
/// the scene
pub fn attach_camera(
    mut commands: Commands,
    query: Query<(Entity, &Transform), (With<Car>, Without<FollowCamera>)>,
) {
    // info!("Looking for Cars to attach camera to");
    for (car_entity, tf) in query.iter() {
        info!("Attaching camera to car");

        let default_camera_list = CameraSettingsList::default();

        // Calculate the camera's transform based on the car's transform
        let camera_translation = tf.translation
            + tf.forward() * default_camera_list[0].distance
            + tf.up() * default_camera_list[0].height;

        // Spawn a camera to follow the car
        let camera = commands
            .spawn((
                FollowCamera::new(car_entity),
                CameraSettingsList::default(),
                Camera3dBundle {
                    // TODO: Use the angle from the camera settings
                    transform: Transform::from_translation(camera_translation)
                        .looking_at(tf.translation, Vec3::Y),
                    camera: Camera {
                        is_active: true,
                        ..default()
                    },
                    projection: PerspectiveProjection {
                        fov: default_camera_list[0].fov.to_radians(),
                        ..Default::default()
                    }
                    .into(),
                    ..default()
                },
            ))
            .id();

        // Add a [`FollowCamera`] [`Component`] to the car
        commands
            .entity(car_entity)
            .insert(FollowCamera::new(camera));
    }
}

/// Bevy [`Update`] [`System`] to make the camera follow the car
pub fn follow_car(
    car_query: Query<(Entity, &Transform, &FollowCamera), (With<Car>, Without<Camera>)>,
    mut camera_query: Query<
        (Entity, &mut Transform, &CameraSettingsList, &FollowCamera),
        With<Camera>,
    >,
) {
    car_query.iter().for_each(|(car_entity, car_transform, _)| {
        // info!(
        //     "Attempting to follow car {}, with camera {}",
        //     car_entity,
        //     follow_camera_target.target.unwrap()
        // );

        if let Some(camera) = camera_query
            .iter_mut()
            .find(|(_, _, _, camera)| camera.target == Some(car_entity))
        {
            let (_, mut camera_tf, camera_settings_list, _) = camera;
            // info!("Following car {} with camera {}", car_entity, camera_entity);

            // Calculate the camera's transform based on the car's transform
            let camera_translation = car_transform.translation
                + car_transform.forward() * camera_settings_list[0].distance
                + car_transform.up() * camera_settings_list[0].height;

            // Smoothly interpolate the camera's transform
            camera_tf.translation = camera_translation.lerp(
                car_transform.translation,
                camera_settings_list[0].smoothness,
            );

            camera_tf.look_at(car_transform.translation, Vec3::Y);
        }
    });
}
