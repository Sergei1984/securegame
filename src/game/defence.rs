use bevy::prelude::*;
use bevy::render::mesh::VertexAttributeValues;
use bevy::sprite::*;
use bevy_rapier2d::prelude::*;

use crate::common::{cuboid_from, get_cursor_pos, MainCamera};

use super::GameParameters;

#[derive(Component, Default, Debug)]
pub struct Defence {
    pub points: Vec<Vec2>,
    pub mesh_handle: Handle<Mesh>,
    pub adding_new_end: bool,
}

pub fn init_defence_drawing(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    info!("Init Defence drawing");

    let mut mesh = Mesh::new(bevy::render::render_resource::PrimitiveTopology::LineStrip);

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

    commands
        .spawn_empty()
        .insert(MaterialMesh2dBundle {
            mesh: def.mesh_handle.clone().into(),
            transform: Transform::default()
                .with_scale(Vec3::splat(1.))
                .with_translation(Vec3::new(0.0, 0.0, 30.0)),
            material: materials.add(ColorMaterial::from(Color::BLACK)),
            ..default()
        })
        .insert(def);
}

pub fn cleanup_defence(mut commands: Commands, defence_query: Query<Entity, With<Defence>>) {
    for e in defence_query.iter() {
        commands.entity(e).despawn_recursive();
    }
}

pub fn draw_defence_core(
    mouse_button: Res<Input<MouseButton>>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut defence_query: Query<&mut Defence>,
    wnds: Res<Windows>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    rapier: Res<RapierContext>,
) {
    if cursor_moved_events.iter().last().is_some() {
        let mut def = defence_query.single_mut();
        let len = def.points.len();

        let position = get_cursor_pos(&wnds, &camera_query);

        let start = if len == 0 {
            position + 0.01
        } else {
            def.points[len - 2]
        };

        let (pos, angle, collider) = cuboid_from(&start, &position, 5.0);

        if rapier
            .intersection_with_shape(pos, angle, &collider, QueryFilter::default())
            .is_some()
        {
            return;
        }

        if mouse_button.just_pressed(MouseButton::Left) {
            if len == 0 {
                def.points.push(position.clone());
                def.points.push(position);
            } else {
                let distance = position - def.points[len - 2];
                info!(
                    "Button pressed. Next point is {} distance {}",
                    &distance,
                    &distance.length()
                );

                if distance.length() < 0.1 {
                    return;
                }

                def.points.push(position);
            }
        } else {
            // Nothing to do until defence is empty
            if len == 0 {
                return;
            }

            let distance = position - def.points[len - 2];
            if distance.length() < 0.1 {
                return;
            }

            def.points[len - 1] = position.clone();
        }
    }
}

pub fn create_defence_collider(
    mut commands: Commands,
    mut defence_query: Query<&Defence>,
    entity_query: Query<Entity, With<Defence>>,
    game_params: Res<GameParameters>,
) {
    info!("Return pressed, creating collider");
    let def = defence_query.single_mut();
    let entity = entity_query.single();

    if def.points.len() > 1 {
        let mut colliders: Vec<(Vec2, f32, Collider)> = vec![];

        let mut prev_point = def.points.iter().next().unwrap();

        for point in def.points.iter().skip(1) {
            if (*point - *prev_point).length() > 0.1 {
                colliders.push(cuboid_from(point, prev_point, 5.0));
            }

            prev_point = point;
        }

        commands
            .entity(entity)
            .insert(RigidBody::Dynamic)
            .insert(Ccd::enabled())
            .insert(Collider::compound(colliders))
            .insert(CollisionGroups::new(game_params.scene_group, Group::ALL))
            .insert(Restitution::coefficient(game_params.defence.restitution))
            .insert(Friction::coefficient(game_params.defence.friction))
            .insert(game_params.defence.damping.clone())
            .insert(AdditionalMassProperties::Mass(game_params.defence.mass))
            .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)));
    }
}

pub fn create_final_defence_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    defence_query: Query<(Entity, &Defence), With<Defence>>,
) {
    let (entity, def) = defence_query.single();

    info!("Create defence final mesh (len={})", def.points.len());

    if def.points.len() > 1 {
        let mut prev_point = def.points.iter().next().unwrap();
        for point in def.points.iter().skip(1) {
            let v = Vec2::new(point.x - prev_point.x, point.y - prev_point.y);

            let midpoint = Vec2::new(
                (prev_point.x + point.x) / 2.0,
                (prev_point.y + point.y) / 2.0,
            );

            let angle = -v.angle_between(Vec2::new(1.0, 0.0));

            let rotation = Quat::from_rotation_z(angle);

            info!("Create defence rect ({}, {}) ", prev_point, point);

            commands.entity(entity).with_children(|parent| {
                parent.spawn(MaterialMesh2dBundle {
                    mesh: meshes
                        .add(Mesh::from(shape::Box::new(v.length(), 5.0, 1.0)))
                        .into(),
                    transform: Transform::default()
                        .with_rotation(rotation)
                        .with_translation(Vec3::new(midpoint.x, midpoint.y, 20.0)),
                    material: materials.add(ColorMaterial::from(Color::BLACK)),
                    ..default()
                });
            });

            prev_point = point;
        }
    }
}

pub fn update_drawing_defence_mesh(
    defence_query: Query<&Defence>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    if let Ok(def) = defence_query.get_single() {
        if let Some(mesh) = meshes.get_mut(&def.mesh_handle) {
            let z = 30.0;
            if def.points.len() > 1 {
                {
                    // POSITION
                    let data = mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION);

                    if let Some(VertexAttributeValues::Float32x3(p)) = data {
                        p.clear();

                        for point in def.points.iter() {
                            p.push([point[0], point[1], z]);
                        }
                    }
                }
                {
                    // NORMAL
                    let data = mesh.attribute_mut(Mesh::ATTRIBUTE_NORMAL);
                    if let Some(VertexAttributeValues::Float32x3(p)) = data {
                        p.clear();
                        for _ in def.points.iter() {
                            p.push([0.0, 0.0, 1.0]);
                        }
                    }
                }
                {
                    // UV
                    let data = mesh.attribute_mut(Mesh::ATTRIBUTE_UV_0);
                    if let Some(VertexAttributeValues::Float32x2(p)) = data {
                        p.clear();
                        for _ in def.points.iter() {
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
}
