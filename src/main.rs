use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup_graphics)
        .add_startup_system(setup_physics)
        // .add_system(print_ball_altitude)
        .add_system(keyboard_input_system)
        .add_system(spawn_a_ball)
        .run();
}

fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands.spawn_bundle(Camera2dBundle::default());
}

fn setup_physics(mut commands: Commands) {
    commands.spawn().insert(KbState::default());

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
}

fn spawn_a_ball(mut commands: Commands, mut query: Query<&mut KbState>) {
    let mut kb = query.single_mut();

    if kb.spawn_ball && kb.count == 0 {
        commands
            .spawn()
            .insert(RigidBody::Dynamic)
            .insert(Collider::ball(20.0))
            .insert(Restitution::coefficient(1.3))
            .insert(Friction::coefficient(1.0))
            .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 400.0, 0.0)));

        kb.count = kb.count + 1;
    }
}

fn keyboard_input_system(keyboard_input: Res<Input<KeyCode>>, mut query: Query<&mut KbState>) {
    let mut kb = query.single_mut();

    if keyboard_input.pressed(KeyCode::Space) && !kb.spawn_ball {
        kb.spawn_ball = true;
        info!("Spawn a ball");
    }

    if !keyboard_input.pressed(KeyCode::Space) && kb.spawn_ball {
        kb.spawn_ball = false;
        kb.count = 0;
        info!("Press space to spawn next ball");
    }
}
#[derive(Component, Default)]
struct KbState {
    pub spawn_ball: bool,
    pub count: i8,
}
