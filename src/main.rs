use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use defence::*;
use game_events::*;
use scene::*;
use swarm::*;
use target::*;

mod common;
mod defence;
mod game_events;
mod random;
mod scene;
mod swarm;
mod target;

fn main() {
    App::new()
        .add_event::<SpawnSwarmEvent>()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(defence_system_startup)
        .add_startup_system(scene_system_startup)
        .add_startup_system(target_system_startup)
        .add_startup_system(swarm_system_startup)
        .add_system_set(swarm_system())
        .add_system_set(defence_system())
        .add_system_set(target_system())
        .add_system(scene_system_create_bounding_box)
        .add_system(spawn_a_ball)
        .add_system(spawn_swarm)
        .run();
}

fn spawn_swarm(
    keyboard_input: Res<Input<KeyCode>>,
    mut spawn_swarm_writer: EventWriter<SpawnSwarmEvent>,
) {
    if keyboard_input.just_pressed(KeyCode::Return) {
        info!("Spawning a swarm");
        spawn_swarm_writer.send(SpawnSwarmEvent);
    }
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
