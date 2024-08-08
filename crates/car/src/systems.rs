use bevy::prelude::*;
use movement::components::Acceleration;

use crate::components::*;

pub fn spawn_car(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    info!("Spawning car");
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(0.5, 0.25, 1.0)),
            material: materials.add(Color::linear_rgb(0.0, 0.0, 1.0)),
            transform: Transform::from_translation(Vec3::new(0.0, 0.125, 0.0)),
            ..Default::default()
        },
        Car,
        movement::components::MovementBundle::default(),
    ));
}

/// Bevy [`System`] to handle car input.
/// Use WASD or the arrow keys to move the car.
pub fn input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut movement::components::Acceleration, &Transform), With<Car>>,
) {
    for (mut prev_acceleration, transform) in query.iter_mut() {
        let mut new_acceleration = Acceleration::ZERO;

        if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
            new_acceleration.linear += *transform.forward();
        }
        if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
            new_acceleration.linear -= *transform.forward();
        }
        if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
            new_acceleration.angular += Vec3::Y;
        }
        if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
            new_acceleration.angular -= Vec3::Y;
        }

        *prev_acceleration = new_acceleration.normalize().clone();
    }
}
