use crate::{game_events::SpawnSwarmEvent, random::*, target::Target};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

pub fn swarm_system() -> SystemSet {
    SystemSet::new()
        .with_system(spawn_wasps)
        .with_system(direct_wasps)
}

pub fn swarm_system_startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn()
        .insert(Hive {
            position: Vec2::new(-200.0, 300.0),
        })
        .insert_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Circle::new(10.0))).into(),
            transform: Transform::default().with_translation(Vec3::new(-200.0, 300.0, 1.0)),
            material: materials.add(ColorMaterial::from(Color::BLACK)),
            ..default()
        });
}

fn spawn_wasps(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut spawn_swarm_events: EventReader<SpawnSwarmEvent>,
    hive_query: Query<&Hive>,
) {
    let hive = hive_query.single();

    if spawn_swarm_events.iter().next().is_some() {
        info!("Spawning wasps");

        for _ in 0..10 {
            let translation = Vec3::new(
                hive.position.x + rand_range(-10.0, 10.0),
                hive.position.y + rand_range(-10.0, 10.0),
                10.0,
            );

            let velocity = Vec2::new(rand_range(-3.0, 3.0), rand_range(-3.0, 3.0)) * 50.0;

            let transform = Transform::from_xyz(translation.x, translation.y, translation.z);
            commands
                .spawn()
                .insert(Wasp {
                    tick: Timer::from_seconds(0.1 + rand::random::<f32>(), true),
                })
                .insert(RigidBody::KinematicVelocityBased)
                .insert(Velocity::linear(velocity))
                .insert_bundle(TransformBundle::from(transform))
                .insert(Collider::ball(5.0))
                .insert(Friction::coefficient(2.0))
                .insert(Restitution::coefficient(0.95))
                .insert(AdditionalMassProperties::Mass(50.0))
                .insert_bundle(MaterialMesh2dBundle {
                    mesh: meshes.add(Mesh::from(shape::Circle::new(5.0))).into(),
                    transform: Transform::default().with_translation(translation),
                    material: materials.add(ColorMaterial::from(Color::RED)),
                    ..default()
                });
        }
    }
}

fn direct_wasps(
    mut wasps_query: Query<(&Transform, &mut Velocity), With<Wasp>>,
    transform_query: Query<&Transform, With<Target>>,
) {
    let target_transform = transform_query.single();

    for (mut wasp_transform, mut velocity) in wasps_query.iter_mut() {
        let vel = Vec2::new(
            target_transform.translation.x - wasp_transform.translation.x,
            target_transform.translation.y - wasp_transform.translation.y,
        )
        .normalize();

        velocity.linvel = vel * 50.0;
    }
}

#[derive(Component, Debug)]
pub struct Hive {
    pub position: Vec2,
}

#[derive(Component, Debug)]
pub struct Wasp {
    pub tick: Timer,
}
