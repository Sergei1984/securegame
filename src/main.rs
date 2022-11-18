use bevy::prelude::*;
use bevy::sprite::*;
use bevy_rapier2d::prelude::*;
use common::MainCamera;
use defence::*;

mod common;
mod defence;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup_camera)
        .add_startup_system(setup_graphics)
        .add_startup_system(defence_system_startup)
        .add_startup_system(setup_physics)
        .add_system(spawn_a_ball)
        .add_system(defence_system_draw_defence_mesh)
        .add_system(defence_system_create_collider)
        .run();
}

fn setup_camera(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands
        .spawn()
        .insert_bundle(Camera2dBundle::default())
        .insert(MainCamera);
}

fn setup_graphics(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        transform: Transform::default().with_scale(Vec3::splat(12800.)),
        material: materials.add(ColorMaterial::from(Color::WHITE)),
        ..default()
    });
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
}

fn spawn_a_ball(mut commands: Commands, keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        commands
            .spawn()
            .insert(RigidBody::Dynamic)
            .insert(Collider::ball(20.0))
            .insert(Restitution::coefficient(1.3))
            .insert(Friction::coefficient(1.0))
            .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 400.0, 0.0)));
    }
}
