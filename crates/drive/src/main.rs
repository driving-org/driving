use bevy::prelude::*;
// main function
fn main() {
    let mut app = App::new();

    // Native Bevy plugins
    app.add_plugins(DefaultPlugins);

    // Custom Plugins
    app.add_plugins((
        car::CarPlugin,
        environment::EnvironmentPlugin,
        follow_camera::FollowCameraPlugin,
    ));

    app.run();
}
