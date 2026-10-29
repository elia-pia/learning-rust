use bevy::{prelude::*, transform};

#[derive(Component, Default)]
#[require(Transform)]
struct Position(Vec2);

#[derive(Component, Default)]
#[require(Transform)]
struct Palla;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (palla, camera))
        .add_systems(FixedUpdate, project_position)
        .run();
}

///creates an object
fn palla(mut commands: Commands) {
    commands.spawn(Palla);
}

///creates a camera
fn camera(mut commands: Commands) {
    commands.spawn(Camera2d); // .spawn_empty().insert()  is equal to .spawn()
}

fn project_position(mut positionables: Query<(&mut Transform, &Position)>) {
    for (mut Transform, position) in &mut positionables {
        Transform.translation = position.0.extend(0.);
    }
}
