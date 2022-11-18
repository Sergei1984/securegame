use bevy::prelude::*;
use bevy::sprite::*;
use bevy_rapier2d::prelude::*;

use crate::common::get_world_coord_from_screen;
use crate::common::MainCamera;

pub fn scene_system_startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
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

    commands.spawn().insert(BoundingBox::default());
}

pub fn scene_system_create_bounding_box(
    mut commands: Commands,
    wnds: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut query: Query<&mut BoundingBox>,
) {
    if q_camera.is_empty() {
        return;
    }

    let mut bb = query.single_mut();

    let win = wnds.get_primary().unwrap();
    let (camera, camera_transform) = q_camera.single();

    if !bb.is_init {
        bb.is_init = true;
        // Create bounding collider
        commands
            .spawn()
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
                ],
                None,
            ))
            .insert(Friction::coefficient(1.0))
            .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, -0.0, 0.0)));
    }
}

#[derive(Component, Default)]
pub struct BoundingBox {
    pub is_init: bool,
}
