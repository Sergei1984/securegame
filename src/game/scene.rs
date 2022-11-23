use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::common::get_world_coord_from_screen;
use crate::common::MainCamera;

pub fn init_camera(mut commands: Commands) {
    commands
        .spawn()
        .insert_bundle(Camera2dBundle::default())
        .insert(MainCamera);
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

    // Create bounding collider
    commands
        .spawn()
        .insert(RigidBody::Fixed)
        .insert(AdditionalMassProperties::Mass(100_000.0))
        .insert(Restitution::coefficient(0.99))
        .insert(Collider::polyline(
            vec![
                get_world_coord_from_screen(
                    Vec2::new(1.0, 1.0),
                    win.width(),
                    win.height(),
                    camera,
                    camera_transform,
                ),
                get_world_coord_from_screen(
                    Vec2::new(win.width() - 1.0, 1.0),
                    win.width(),
                    win.height(),
                    camera,
                    camera_transform,
                ),
                get_world_coord_from_screen(
                    Vec2::new(win.width() - 1.0, win.height() - 1.0),
                    win.width(),
                    win.height(),
                    camera,
                    camera_transform,
                ),
                get_world_coord_from_screen(
                    Vec2::new(1.0, win.height() - 1.0),
                    win.width(),
                    win.height(),
                    camera,
                    camera_transform,
                ),
                get_world_coord_from_screen(
                    Vec2::new(1.0, 1.0),
                    win.width(),
                    win.height(),
                    camera,
                    camera_transform,
                ),
            ],
            None,
        ))
        .insert(Friction::coefficient(1000.0))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, -0.0, 0.0)));
}
