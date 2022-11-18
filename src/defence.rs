use bevy::prelude::*;
use bevy::sprite::*;
use bevy_rapier2d::prelude::*;

pub fn defence_startup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut mesh = Mesh::new(bevy::render::render_resource::PrimitiveTopology::LineList);

    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            [-500.0, 110.0, 0.0],
            [100.0, 10.0, 0.0],
            [100.0, 10.0, 0.0],
            [500.0, 110.0, 0.0],
        ],
    );

    mesh.insert_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        vec![
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
        ],
    );

    mesh.insert_attribute(
        Mesh::ATTRIBUTE_UV_0,
        vec![[0.0, 0.0], [0.0, 0.0], [0.0, 0.0], [0.0, 0.0]],
    );

    commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: meshes.add(mesh).into(),
        transform: Transform::default()
            .with_scale(Vec3::splat(1.))
            .with_translation(Vec3::new(0.0, 0.0, 1.0)),
        material: materials.add(ColorMaterial::from(Color::BLACK)),
        ..default()
    });
}

#[derive(Component, Default, Debug)]
struct Defence {
    pub points: Vec<Vec2>,
    pub end: Option<Vec2>,
    pub handle: Mesh2dHandle,
}
