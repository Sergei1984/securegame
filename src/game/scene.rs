use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::common::get_world_coord_from_screen;
use crate::common::MainCamera;

pub fn init_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default()).insert(MainCamera);
}

pub fn init_scene(
    mut commands: Commands,
    mut wnds: ResMut<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    info!("Init scene");

    if q_camera.is_empty() {
        info!("Camera is empty");
        return;
    }

    let win = wnds.get_primary_mut().unwrap();
    let (camera, camera_transform) = q_camera.single();

    info!("Window size {}x{}", win.width(), win.height());

    win.set_resizable(false);
    let offset = 5.0;

    // Create bounding collider
    commands
        .spawn(RigidBody::Fixed)
        .insert(AdditionalMassProperties::Mass(100_000.0))
        .insert(Restitution::coefficient(0.99))
        .insert(Collider::polyline(
            vec![
                get_world_coord_from_screen(
                    Vec2::new(offset, offset),
                    win.width(),
                    win.height(),
                    camera,
                    camera_transform,
                ),
                get_world_coord_from_screen(
                    Vec2::new(win.width() - offset, offset),
                    win.width(),
                    win.height(),
                    camera,
                    camera_transform,
                ),
                get_world_coord_from_screen(
                    Vec2::new(win.width() - offset, win.height() - offset),
                    win.width(),
                    win.height(),
                    camera,
                    camera_transform,
                ),
                get_world_coord_from_screen(
                    Vec2::new(offset, win.height() - offset),
                    win.width(),
                    win.height(),
                    camera,
                    camera_transform,
                ),
                get_world_coord_from_screen(
                    Vec2::new(offset, offset),
                    win.width(),
                    win.height(),
                    camera,
                    camera_transform,
                ),
            ],
            None,
        ))
        .insert(Friction::coefficient(1000.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)));
}
