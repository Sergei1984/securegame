use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

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
) {
    info!("Init scene");

    if q_camera.is_empty() {
        info!("Camera is empty");
        return;
    }

    let win = wnds.get_primary_mut().unwrap();
    let (camera, camera_transform) = q_camera.single();

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

    let offset = 5.0;

    // Create bounding collider
    commands
        .spawn(Bounds)
        .insert(RigidBody::Fixed)
        .insert(AdditionalMassProperties::Mass(game_params.scene.mass))
        .insert(Restitution::coefficient(game_params.scene.restitution))
        .insert(Friction::coefficient(game_params.scene.friction))
        .insert(Collider::compound(vec![
            cuboid_from_screen_coords(
                Vec2::new(offset, offset),
                Vec2::new(win.width() - offset, offset),
                win.width(),
                win.height(),
                camera,
                camera_transform,
            ),
            cuboid_from_screen_coords(
                Vec2::new(win.width() - offset, offset),
                Vec2::new(win.width() - offset, win.height() - offset),
                win.width(),
                win.height(),
                camera,
                camera_transform,
            ),
            cuboid_from_screen_coords(
                Vec2::new(win.width() - offset, win.height() - offset),
                Vec2::new(offset, win.height() - offset),
                win.width(),
                win.height(),
                camera,
                camera_transform,
            ),
            cuboid_from_screen_coords(
                Vec2::new(offset, win.height() - offset),
                Vec2::new(offset, offset),
                win.width(),
                win.height(),
                camera,
                camera_transform,
            ),
        ]))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)));
}

pub fn cleanup_scene(mut commands: Commands, bounds_query: Query<Entity, With<Bounds>>) {
    for e in bounds_query.iter() {
        commands.entity(e).despawn_recursive();
    }
}

#[derive(Component)]
pub struct Bounds;

fn cuboid_from_screen_coords(
    start_screen: Vec2,
    end_screen: Vec2,
    window_width: f32,
    window_height: f32,
    camera: &Camera,
    camera_transform: &GlobalTransform,
) -> (Vec2, f32, Collider) {
    let start = get_world_coord_from_screen(
        start_screen,
        window_width,
        window_height,
        camera,
        camera_transform,
    );
    let end = get_world_coord_from_screen(
        end_screen,
        window_width,
        window_height,
        camera,
        camera_transform,
    );

    let center = Vec2::new((start.x + end.x) / 2.0, (start.y + end.y) / 2.0);
    let width = ((start.x - end.x).abs()).max(20.0);
    let height = ((start.y - end.y).abs()).max(20.0);

    return (center, 0.0, Collider::cuboid(width / 2.0, height / 2.0));
}
