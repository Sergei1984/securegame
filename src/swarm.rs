use crate::random::*;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;
use rand::prelude::*;

pub fn swarm_system() -> SystemSet {
    SystemSet::new().with_system(spawn_wasps)
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
    keyboard_input: Res<Input<KeyCode>>,
    hive_query: Query<&Hive>,
) {
    let hive = hive_query.single();

    if keyboard_input.just_pressed(KeyCode::Return) {
        info!("Spawning wasps");

        for _ in 0..10 {
            let translation = Vec3::new(
                hive.position.x + rand_range(-10.0, 10.0),
                hive.position.y + rand_range(-10.0, 10.0),
                20.0,
            );

            let transform = Transform::from_xyz(translation.x, translation.y, translation.z);
            commands
                .spawn()
                .insert(Wasp {
                    tick: Timer::from_seconds(0.1 + rand::random::<f32>(), true),
                })
                .insert(RigidBody::Dynamic)
                .insert_bundle(TransformBundle::from(transform))
                .insert(Collider::ball(5.0))
                .insert(Restitution::coefficient(0.95))
                .insert(AdditionalMassProperties::Mass(200.0))
                .insert_bundle(MaterialMesh2dBundle {
                    mesh: meshes.add(Mesh::from(shape::Circle::new(5.0))).into(),
                    transform: Transform::default().with_translation(translation),
                    material: materials.add(ColorMaterial::from(Color::RED)),
                    ..default()
                });
        }
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
