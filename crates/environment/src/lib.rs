use bevy::{
    color::palettes::css::*,
    pbr::CascadeShadowConfigBuilder,
    prelude::*,
    render::camera::{Exposure, PhysicalCameraParameters},
};

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        // insert cam parameters here instead
        app.insert_resource(Parameters(PhysicalCameraParameters {
            aperture_f_stops: 1.0,
            shutter_speed_s:  1.0 / 125.0,
            sensitivity_iso:  100.0,
            sensor_height:    0.01866,
        }))
        .add_systems(Startup, setup);
    }
}

#[derive(Resource, Default, Deref, DerefMut)]
struct Parameters(PhysicalCameraParameters);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    cam_parameters: Res<Parameters>,
) {
    // ground plane
    commands.spawn((PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(10.0, 10.0)),
        material: materials.add(StandardMaterial {
            base_color: Color::WHITE,
            perceptual_roughness: 1.0,
            ..default()
        }),
        ..default()
    },));

    // ambient light
    commands.insert_resource(AmbientLight {
        color:      ORANGE_RED.into(),
        brightness: 0.02,
    });

    // directional 'sun' light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::PI / 4.),
            ..default()
        },
        // The default cascade config is designed to handle large scenes.
        // As this example has a much smaller world, we can tighten the shadow
        // bounds for better visual quality.
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 10.0,
            ..default()
        }
        .into(),
        ..default()
    });

    // camera
    // commands.spawn(Camera3dBundle {
    //     transform: Transform::from_xyz(0.0, 10.0, 0.0).looking_at(Vec3::ZERO,
    // -Vec3::Z),     exposure:
    // Exposure::from_physical_camera(**cam_parameters),     ..default()
    // });
}
