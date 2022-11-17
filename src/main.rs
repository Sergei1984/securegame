use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup_graphics)
        .add_startup_system(setup_physics)
        .add_system(print_ball_altitude)
        .run();
}

fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands.spawn_bundle(Camera2dBundle::default());
}

fn setup_physics(mut commands: Commands) {
    /* Create the ground. */
    commands
        .spawn()
        // .insert(Collider::cuboid(1000.0, 50.0))
        .insert(Collider::polyline(
            vec![
                Vec2::new(-500.0, 100.0),
                Vec2::new(100.0, 0.0),
                Vec2::new(500.0, 100.0),
            ],
            None,
        ))
        .insert(Friction::coefficient(1.0))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, -100.0, 0.0)));

    commands
        .spawn()
        .insert(Collider::polyline(
            vec![Vec2::new(50.0, 60.0), Vec2::new(200.0, 20.0)],
            None,
        ))
        .insert(Friction::coefficient(1.0));

    /* Create the bouncing ball. */
    commands
        .spawn()
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(20.0))
        .insert(Restitution::coefficient(1.3))
        .insert(Friction::coefficient(1.0))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 400.0, 0.0)));

    commands
        .spawn()
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(20.0))
        .insert(Restitution::coefficient(1.3))
        .insert(Friction::coefficient(1.0))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(
            -10.0, 200.0, 0.0,
        )));
}

fn print_ball_altitude(positions: Query<&Transform, With<RigidBody>>) {
    for transform in positions.iter() {
        println!("Ball altitude: {}", transform.translation.y);
    }
}