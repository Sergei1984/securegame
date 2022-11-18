use bevy::prelude::*;
use bevy::render::mesh::VertexAttributeValues;
use bevy::sprite::*;
use bevy_rapier2d::prelude::*;

use crate::common::{get_cursor_pos, MainCamera};

pub fn defence_system_startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut mesh = Mesh::new(bevy::render::render_resource::PrimitiveTopology::LineList);

    let empty3: Vec<[f32; 3]> = vec![];
    let empty2: Vec<[f32; 2]> = vec![];

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, empty3.clone());
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, empty3.clone());
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, empty2);

    let def = Defence {
        points: vec![],
        mesh_handle: meshes.add(mesh),
        adding_new_end: false,
    };

    commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: def.mesh_handle.clone().into(),
        transform: Transform::default()
            .with_scale(Vec3::splat(1.))
            .with_translation(Vec3::new(0.0, 0.0, 1.0)),
        material: materials.add(ColorMaterial::from(Color::BLACK)),
        ..default()
    });

    commands.spawn().insert(def);
}

pub fn defence_system_draw_defence_mesh(
    mouse_button: Res<Input<MouseButton>>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut query: Query<&mut Defence>,
    meshes: ResMut<Assets<Mesh>>,
    wnds: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let mut def = query.single_mut();

    if cursor_moved_events.iter().last().is_some() {
        let position = get_cursor_pos(&wnds, &q_camera);

        if mouse_button.just_pressed(MouseButton::Left) && !def.adding_new_end {
            def.points.push(position.clone());
            if def.points.len() == 1 {
                def.points.push(position.clone());
            }

            def.adding_new_end = true;
        }

        if mouse_button.just_released(MouseButton::Left) {
            def.adding_new_end = false;
        }

        if def.adding_new_end {
            let last_index = def.points.len() - 1;
            def.points[last_index] = position.clone();
            defence_to_mesh(def, meshes);
        }
    }
}

pub fn defence_system_create_collider(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Defence>,
    meshes: ResMut<Assets<Mesh>>,
) {
    if keyboard_input.just_pressed(KeyCode::Return) {
        info!("Return pressed, creating collider");
        let mut def = query.single_mut();

        commands
            .spawn()
            .insert(RigidBody::Dynamic)
            .insert(Collider::polyline(def.points.clone(), None))
            .insert(Restitution::coefficient(0.9))
            .insert(Friction::coefficient(1.0))
            .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)));

        def.points.clear();

        defence_to_mesh(def, meshes);
    }
}

fn defence_to_mesh(def: Mut<Defence>, mut meshes: ResMut<Assets<Mesh>>) {
    if let Some(mesh) = meshes.get_mut(&def.mesh_handle) {
        if def.points.len() > 1 {
            let mut need_add_new_line = false;
            {
                // POSITION
                let data = mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION);
                if let Some(VertexAttributeValues::Float32x3(p)) = data {
                    let num_lines = def.points.len() - 1;
                    need_add_new_line = p.len() / 2 < num_lines;

                    let l = def.points.len();
                    let pre_last_point = &def.points[l - 2];
                    let last_point = &def.points[l - 1];

                    if need_add_new_line {
                        p.push([pre_last_point[0], pre_last_point[1], 0.0]);
                        p.push([last_point[0], last_point[1], 0.0]);
                    } else {
                        let last_index = p.len() - 1;
                        p[last_index] = [last_point[0], last_point[1], 0.0];
                    }
                }
            }
            {
                // NORMAL
                let data = mesh.attribute_mut(Mesh::ATTRIBUTE_NORMAL);
                if let Some(VertexAttributeValues::Float32x3(p)) = data {
                    if need_add_new_line {
                        p.push([0.0, 0.0, 1.0]);
                        p.push([0.0, 0.0, 1.0]);
                    }
                }
            }
            {
                // UV
                let data = mesh.attribute_mut(Mesh::ATTRIBUTE_UV_0);
                if let Some(VertexAttributeValues::Float32x2(p)) = data {
                    if need_add_new_line {
                        p.push([0.0, 0.0]);
                        p.push([0.0, 0.0]);
                    }
                }
            }
        } else {
            mesh.remove_attribute(Mesh::ATTRIBUTE_POSITION);
            mesh.remove_attribute(Mesh::ATTRIBUTE_NORMAL);
            mesh.remove_attribute(Mesh::ATTRIBUTE_UV_0);

            let empty3: Vec<[f32; 3]> = vec![];
            let empty2: Vec<[f32; 2]> = vec![];

            mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, empty3.clone());
            mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, empty3.clone());
            mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, empty2);
        }
    }
}

#[derive(Component, Default, Debug)]
pub struct Defence {
    pub points: Vec<Vec2>,
    pub mesh_handle: Handle<Mesh>,
    pub adding_new_end: bool,
}
