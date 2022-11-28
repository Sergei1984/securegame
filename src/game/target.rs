use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use iyes_loopless::state::NextState;

use crate::game::GameState;

use super::{swarm::Wasp, GameParameters};

pub fn init_target(
    game_params: Res<GameParameters>,
    mut commands: Commands,
    level_query: Query<&super::level::Level>,
) {
    info!("Init Target");

    let level = level_query.single();

    commands
        .spawn(Target {
            win_timer: Timer::from_seconds(5.0, TimerMode::Once),
            // was_bitten: false,
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(30.0))
        .insert(CollisionGroups::new(game_params.scene_group, Group::ALL))
        .insert(LockedAxes::TRANSLATION_LOCKED)
        .insert(Restitution::coefficient(game_params.target.restitution))
        .insert(Friction::coefficient(game_params.target.friction))
        .insert(AdditionalMassProperties::Mass(game_params.target.mass))
        .insert(game_params.target.damping.clone())
        .insert(SpriteBundle {
            sprite: Sprite {
                custom_size: Some([60.0, 60.0].into()),
                ..default()
            },
            texture: level.dog_handle.clone(),
            ..default()
        })
        .insert(TransformBundle::from(Transform::from_xyz(
            level.target_position.x,
            level.target_position.y,
            20.0,
        )));
}

pub fn unlock_target(
    mut target_query: Query<&mut Target>,
    mut axes_query: Query<&mut LockedAxes, With<Target>>,
    mut rapier: ResMut<RapierConfiguration>,
) {
    info!("Starting targer survive timer");
    let mut target = target_query.single_mut();
    target.win_timer = Timer::from_seconds(20.0, TimerMode::Once);
    rapier.physics_pipeline_active = true;

    if let Some(mut axes) = axes_query.iter_mut().next() {
        axes.set(LockedAxes::TRANSLATION_LOCKED, false);
    }
}

pub fn detect_wasp_sting(
    mut commands: Commands,
    time: Res<Time>,
    rapier_context: Res<RapierContext>,
    wasp_query: Query<Entity, With<Wasp>>,
    target_query: Query<Entity, With<Target>>,
    mut target_query2: Query<&mut Target>,
    mut rapier: ResMut<RapierConfiguration>,
) {
    if rapier.physics_pipeline_active {
        let mut t = target_query2.single_mut();
        if t.win_timer.tick(time.delta()).finished() {
            info!("Target survived!");
            commands.insert_resource(NextState(GameState::Win));
            return;
        }

        for target_entity in target_query.iter() {
            for wasp in wasp_query.iter() {
                if let Some(contact_pair) = rapier_context.contact_pair(target_entity, wasp) {
                    if contact_pair.has_any_active_contacts() {
                        info!("Target bitten by the wasp!");
                        rapier.physics_pipeline_active = false;
                    }
                }
            }
        }
    }
}

pub fn cleanup_target(mut commands: Commands, target_query: Query<Entity, With<Target>>) {
    for t in target_query.iter() {
        commands.entity(t).despawn_recursive();
    }
}

#[derive(Component)]
pub struct Target {
    pub win_timer: Timer,
}
