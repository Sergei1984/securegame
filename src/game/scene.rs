use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::common::cuboid_from;
use crate::common::get_world_coord_from_screen;
use crate::common::MainCamera;

use super::GameParameters;

pub fn init_camera(mut commands: Commands, mut wnds: ResMut<Windows>) {
    commands.spawn(Camera2dBundle::default()).insert(MainCamera);

    let win = wnds.get_primary_mut().unwrap();

    win.set_resizable(false);
}

pub fn init_scene(
    mut commands: Commands,
    mut wnds: ResMut<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    game_params: Res<GameParameters>,
    level_query: Query<&super::level::Level>,
) {
    info!("Init scene");

    if q_camera.is_empty() {
        info!("Camera is empty");
        return;
    }

    let win = wnds.get_primary_mut().unwrap();
    let (camera, camera_transform) = q_camera.single();

    let level = level_query.single();

    let mut land_colliders: Vec<(Vec2, f32, Collider)> = vec![];

    if level.land_lines.len() > 1 {
        let mut prev_point = level.land_lines.iter().next().unwrap();

        for point in level.land_lines.iter().skip(1) {
            let collider = cuboid_from(prev_point, point, 5.0);

            land_colliders.push(collider);

            prev_point = point;
        }
    }

    info!(
        "Window size {}x{}, world coords: {} -> {}",
        win.width(),
        win.height(),
        get_world_coord_from_screen(
            Vec2::new(0.0, 0.0),
            win.width(),
            win.height(),
            camera,
            camera_transform
        ),
        get_world_coord_from_screen(
            Vec2::new(win.width(), win.height()),
            win.width(),
            win.height(),
            camera,
            camera_transform
        ),
    );

    // Create bounding collider
    commands
        .spawn(Bounds)
        .insert(RigidBody::Fixed)
        .insert(AdditionalMassProperties::Mass(game_params.scene.mass))
        .insert(Restitution::coefficient(game_params.scene.restitution))
        .insert(Friction::coefficient(game_params.scene.friction))
        .insert(Collider::compound(land_colliders))
        .insert(CollisionGroups::new(game_params.scene_group, Group::ALL))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)));
}

pub fn cleanup_scene(mut commands: Commands, bounds_query: Query<Entity, With<Bounds>>) {
    for e in bounds_query.iter() {
        commands.entity(e).despawn_recursive();
    }
}

#[derive(Component)]
pub struct Bounds;
