use crate::random::rand_range;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

use super::target::Target;

pub fn create_hive(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    info!("Create hive");

    commands
        .spawn(Hive {
            position: Vec2::new(-200.0, 300.0),
        })
        .insert(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Circle::new(10.0))).into(),
            transform: Transform::default().with_translation(Vec3::new(-200.0, 300.0, 5.0)),
            material: materials.add(ColorMaterial::from(Color::BLACK)),
            ..default()
        });
}

pub fn spawn_wasps(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    hive_query: Query<&Hive>,
) {
    let hive = hive_query.single();

    info!("Spawning wasps");

    for _ in 0..10 {
        let translation = Vec3::new(
            hive.position.x + rand_range(-10.0, 10.0),
            hive.position.y + rand_range(-10.0, 10.0),
            0.0,
        );

        let transform = Transform::from_xyz(translation.x, translation.y, translation.z);
        commands
            .spawn(Wasp {
                timer: Timer::from_seconds(0.1 + rand_range(0.0, 0.9), TimerMode::Repeating),
            })
            .insert(RigidBody::Dynamic)
            .insert(ExternalImpulse::default())
            .insert(TransformBundle::from(transform))
            .insert(Collider::ball(5.0))
            .insert(Friction::coefficient(2.0))
            .insert(Restitution::coefficient(0.95))
            .insert(AdditionalMassProperties::Mass(50.0))
            .insert(GravityScale(0.0))
            .insert(MaterialMesh2dBundle {
                mesh: meshes.add(Mesh::from(shape::Circle::new(5.0))).into(),
                transform: Transform::default().with_translation(Vec3::new(
                    translation.x,
                    translation.y,
                    10.0,
                )),
                material: materials.add(ColorMaterial::from(Color::RED)),
                ..default()
            });
    }
}

pub fn direct_wasps(
    time: Res<Time>,
    mut wasps_query: Query<(&mut Wasp, &Transform, &mut ExternalImpulse), With<Wasp>>,
    transform_query: Query<&Transform, With<Target>>,
) {
    let target_transform = transform_query.single();

    for (mut wasp, wasp_transform, mut external_impulse) in wasps_query.iter_mut() {
        if wasp.timer.tick(time.delta()).just_finished() {
            let impulse = Vec2::new(
                target_transform.translation.x - wasp_transform.translation.x,
                target_transform.translation.y - wasp_transform.translation.y,
            )
            .normalize();

            external_impulse.impulse = impulse * 3000.0;
        }
    }
}

pub fn cleanup_swarm(
    mut commands: Commands,
    hive_query: Query<Entity, With<Hive>>,

    wasp_query: Query<Entity, With<Wasp>>,
) {
    let hive = hive_query.single();

    commands.entity(hive).despawn_recursive();

    for w in wasp_query.iter() {
        commands.entity(w).despawn_recursive();
    }
}

#[derive(Component, Debug)]
pub struct Hive {
    pub position: Vec2,
}

#[derive(Component, Debug)]
pub struct Wasp {
    pub timer: Timer,
}
