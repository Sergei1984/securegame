use crate::random::rand_range;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

use super::GameParameters;

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
    level_query: Query<&super::level::Level>,
) {
    let hive = hive_query.single();
    let level = level_query.single();

    info!("Spawning wasps");

    for i in 0..10 {
        let translation = Vec3::new(
            hive.position.x + rand_range(-20.0, 20.0),
            hive.position.y + rand_range(-20.0, 20.0),
            30.0,
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
            .insert(Collider::ball(20.0))
            .insert(CollisionGroups::new((1 << i) as Group, Group::ALL))
            .insert(Friction::coefficient(game_params.wasp.friction))
            .insert(Restitution::coefficient(game_params.wasp.restitution))
            .insert(AdditionalMassProperties::Mass(game_params.wasp.mass))
            .insert(game_params.wasp.damping.clone())
            .insert(GravityScale(0.0))
            .insert(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some([20.0, 20.0].into()),
                    ..default()
                },
                texture: level.wasp_handle.clone(),
                ..default()
            })
            .insert(TransformBundle::from(transform));
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
