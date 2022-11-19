use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use defence::*;
use scene::*;
use target::*;

mod common;
mod defence;
mod scene;
mod target;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(defence_system_startup)
        .add_startup_system(scene_system_startup)
        .add_startup_system(target_system_startup)
        .add_system(scene_system_create_bounding_box)
        .add_system(spawn_a_ball)
        .add_system(defence_system_draw_defence_mesh)
        .add_system(defence_system_create_collider)
        .run();
}

fn spawn_a_ball(mut commands: Commands, keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        commands
            .spawn()
            .insert(RigidBody::Dynamic)
            .insert(Collider::ball(20.0))
            .insert(Restitution::coefficient(0.8))
            .insert(Friction::coefficient(1.0))
            .insert(AdditionalMassProperties::Mass(100.0))
            .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 200.0, 0.0)));
    }
}
