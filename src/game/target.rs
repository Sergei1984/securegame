use bevy::prelude::*;
use bevy_rapier2d::{prelude::*, rapier::prelude::ColliderBuilder};

use super::swarm::Wasp;

pub fn init_target(mut commands: Commands) {
    commands
        .spawn()
        .insert(Target)
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(10.0))
        .insert(Restitution::coefficient(0.8))
        .insert(Friction::coefficient(1.0))
        .insert(AdditionalMassProperties::Mass(100.0))
        .insert(LockedAxes::TRANSLATION_LOCKED)
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, -200.0, 0.0)));
}

pub fn unlock_target(mut target_query: Query<&mut LockedAxes, With<Target>>) {
    if let Some(mut target) = target_query.iter_mut().next() {
        target.set(LockedAxes::TRANSLATION_LOCKED, false);
    }
}

pub fn detect_wasp_sting(
    mut commands: Commands,
    rapier_context: Res<RapierContext>,
    wasp_query: Query<Entity, With<Wasp>>,
    target_query: Query<Entity, With<Target>>,
) {
    for target in target_query.iter() {
        for wasp in wasp_query.iter() {
            if rapier_context.contact_pair(target, wasp).is_some() {
                info!("Target bitten by the wasp!");
            }
        }
    }
}

#[derive(Component)]
pub struct Target;
