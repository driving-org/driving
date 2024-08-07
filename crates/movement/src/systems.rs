use bevy::prelude::*;

use crate::components::*;

/// Bevy [`System`] to handle velocity, taking delta time into account
pub fn handle_velocity(
    mut query: Query<(Entity, &Velocity, &mut Transform)>,
    time: Res<Time<Virtual>>,
) {
    for (entity, velocity, mut transform) in query.iter_mut() {
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
    for (entity, acceleration, mut velocity) in query.iter_mut() {
        info!(
            "Handling acceleration for entity {}, with acceleration {:?}",
            entity, acceleration
        );
        velocity.linear += acceleration.linear * time.delta_seconds();
        velocity.angular += acceleration.angular * time.delta_seconds();
    }
}
