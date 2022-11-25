use crate::random::rand_range;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::GameParameters;

pub fn create_hive(mut commands: Commands, level_query: Query<&super::level::Level>) {
    info!("Create hive");

    let level = level_query.single();

    let hive = Hive {
        position: level.hive_position.clone(),
    };

    commands
        .spawn_empty()
        .insert(SpriteBundle {
            sprite: Sprite {
                custom_size: Some([80.0, 80.0].into()),
                ..default()
            },
            texture: level.hive_handle.clone(),
            ..default()
        })
        .insert(TransformBundle::from(Transform::from_xyz(
            hive.position.x,
            hive.position.y,
            10.0,
        )))
        .insert(hive);
}

pub fn spawn_wasps(
    game_params: Res<GameParameters>,
    mut commands: Commands,
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

        let wasp_group: u32 = 1 << i;

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
            .insert(Collider::ball(10.0))
            .insert(CollisionGroups::new(
                Group::from_bits(wasp_group).unwrap(),
                game_params.scene_group,
            ))
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
