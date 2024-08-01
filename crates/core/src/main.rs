use bevy::prelude::*;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

// main function
fn main() {
    let result = add(2, 2);
    println!("Result: {}", result);

    let mut app = App::new();

    app.add_plugins(DefaultPlugins);

    app.run();
}
