use bevy::prelude::*;

use crate::components::*;

/// Bevy [`System`] to handle velocity, taking delta time into account
pub fn handle_velocity(
    mut query: Query<(Entity, &Velocity, &mut Transform)>,
    time: Res<Time<Virtual>>,
) {
    for (_, velocity, mut transform) in query.iter_mut() {
        // info!(
        //     "Handling velocity for entity {}, with velocity {:?}",
        //     entity, velocity
        // );
        transform.translation += velocity.linear * time.delta_seconds();
        transform.rotate(Quat::from_rotation_y(
            velocity.angular.y * time.delta_seconds(),
        ));
    }
}

/// Bevy [`System`] to handle acceleration, taking delta time into account
pub fn handle_acceleration(
    mut query: Query<(Entity, &Acceleration, &mut Velocity)>,
    time: Res<Time<Virtual>>,
) {
    for (_, acceleration, mut velocity) in query.iter_mut() {
        velocity.linear += acceleration.linear * time.delta_seconds();
        velocity.angular += acceleration.angular * time.delta_seconds();
    }
}

/// Bevy [`System`] to apply friction to velocity
pub fn apply_friction(mut query: Query<(&mut Velocity, &Acceleration)>, time: Res<Time<Virtual>>) {
    for (mut velocity, acceleration) in query.iter_mut() {
        info!(
            "Applying friction to velocity {:?} with acceleration {:?}",
            velocity, acceleration
        );
        let friction_coefficient_linear = 1.0;
        let friction_coefficient_angular = 1.0;
        let previous_velocity = velocity.clone();
        velocity.linear *= 1.0
            - previous_velocity.linear.length()
                * time.delta_seconds()
                * friction_coefficient_linear;
        velocity.angular *= 1.0
            - previous_velocity.angular.length()
                * time.delta_seconds()
                * friction_coefficient_angular;

        // if velocity.linear.length() < 0.01 {
        //     velocity.linear = Vec3::ZERO;
        // }
    }
}
