use bevy::prelude::*;

///creates an object
fn palla(mut commands: Commands) {
    commands.spawn_empty();
}

///creates a camera
fn camera(mut commands: Commands) {
    commands.spawn((
        Camera2d, // .spawn_empty().insert()  is equal to .spawn()
        Transform::from_xyz(0., 0., 0.),
    ));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, palla)
        .run();
}
