use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;
use iyes_loopless::state::NextState;

use crate::game::GameState;

use super::swarm::Wasp;

pub fn init_target(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    info!("Init Target");

    commands
        .spawn()
        .insert(Target {
            win_timer: Timer::from_seconds(5.0, false),
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(10.0))
        .insert(Restitution::coefficient(0.8))
        .insert(Friction::coefficient(1.0))
        .insert(AdditionalMassProperties::Mass(500.0))
        .insert(LockedAxes::TRANSLATION_LOCKED)
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, -200.0, 0.0)))
        .insert_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Circle::new(10.0))).into(),
            transform: Transform::default().with_translation(Vec3::new(0.0, -200.0, 10.0)),
            material: materials.add(ColorMaterial::from(Color::GREEN)),
            ..default()
        });
}

pub fn unlock_target(
    mut target_query: Query<&mut Target>,
    mut axes_query: Query<&mut LockedAxes, With<Target>>,
) {
    info!("Starting targer survive timer");
    let mut target = target_query.single_mut();
    target.win_timer = Timer::from_seconds(20.0, false);

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
) {
    let mut t = target_query2.single_mut();
    if t.win_timer.tick(time.delta()).finished() {
        info!("Target survived!");
        commands.insert_resource(NextState(GameState::Win));
        return;
    }

    for target in target_query.iter() {
        for wasp in wasp_query.iter() {
            if rapier_context.contact_pair(target, wasp).is_some() {
                info!("Target bitten by the wasp!");
                commands.insert_resource(NextState(GameState::Lose));
            }
        }
    }
}

#[derive(Component)]
pub struct Target {
    pub win_timer: Timer,
}
