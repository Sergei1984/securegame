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

pub fn target_system() -> SystemSet {
    SystemSet::new().with_system(unlock_target)
}

fn unlock_target(
    mut target_query: Query<&mut LockedAxes, With<Target>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Return) {
        if let Some(mut target) = target_query.iter_mut().next() {
            target.set(LockedAxes::TRANSLATION_LOCKED, false);
        }
    }
}
#[derive(Component)]
pub struct Target;
