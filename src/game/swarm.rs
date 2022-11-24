use crate::random::rand_range;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

use super::{defence::Defence, target::Target, GameParameters};

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
            transform: Transform::default().with_translation(Vec3::new(-200.0, 200.0, 5.0)),
            material: materials.add(ColorMaterial::from(Color::BLACK)),
            ..default()
        });
}

pub fn spawn_wasps(
    game_params: Res<GameParameters>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    hive_query: Query<&Hive>,
) {
    let hive = hive_query.single();

    info!("Spawning wasps");

    for _ in 0..10 {
        let translation = Vec3::new(
            hive.position.x + rand_range(-20.0, 20.0),
            hive.position.y + rand_range(-20.0, 20.0),
            0.0,
        );

        let transform = Transform::from_xyz(translation.x, translation.y, translation.z);
        commands
            .spawn(Wasp {
                timer: Timer::from_seconds(0.05 + rand_range(0.0, 0.2), TimerMode::Repeating),
            })
            .insert(RigidBody::Dynamic)
            .insert(ExternalImpulse {
                impulse: Vec2::ZERO,
                torque_impulse: 0.0,
            })
            .insert(TransformBundle::from(transform))
            .insert(Collider::ball(5.0))
            .insert(Friction::coefficient(20.0))
            .insert(Restitution::coefficient(game_params.restitution))
            .insert(AdditionalMassProperties::Mass(game_params.wasp_mass))
            .insert(Damping {
                linear_damping: 0.1,
                angular_damping: 0.1,
            })
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
