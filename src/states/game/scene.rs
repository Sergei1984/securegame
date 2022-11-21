use bevy::prelude::*;
use bevy::sprite::*;
use bevy_rapier2d::prelude::*;

use crate::common::get_world_coord_from_screen;
use crate::common::MainCamera;

pub fn scene_system_startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut wnds: ResMut<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    commands
        .spawn()
        .insert_bundle(Camera2dBundle::default())
        .insert(MainCamera);

    // Create background
    commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        transform: Transform::default().with_scale(Vec3::splat(12800.)),
        material: materials.add(ColorMaterial::from(Color::WHITE)),
        ..default()
    });

    if q_camera.is_empty() {
        return;
    }

    let win = wnds.get_primary_mut().unwrap();
    let (camera, camera_transform) = q_camera.single();

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
