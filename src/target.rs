use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn target_system_startup(mut commands: Commands) {
    commands
        .spawn()
        .insert(Target)
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(10.0))
        .insert(Restitution::coefficient(0.8))
        .insert(Friction::coefficient(1.0))
        .insert(AdditionalMassProperties::Mass(100.0))
        .insert(LockedAxes::TRANSLATION_LOCKED)
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, -200.0, 0.0)));
}

#[derive(Component)]
pub struct Target;
