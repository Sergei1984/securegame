use bevy::{asset::LoadState, prelude::*};
use iyes_loopless::prelude::*;

use crate::common::{get_world_coord_from_screen, MainCamera};

use super::{CurrentLevel, GameState};

pub fn start_level_loading(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    current_level: Res<CurrentLevel>,
) {
    // commands.insert_resource(NextState(GameState::DrawDefence));

    // return;

    let level = Level {
        bg_handle: asset_server.load(format!("levels/{}/level.png", current_level.value).as_str()),
        control_plane_handle: asset_server
            .load(format!("levels/{}/control.png", current_level.value).as_str()),
        dog_handle: asset_server.load("dog.png"),
        wasp_handle: asset_server.load("bee.png"),
        hive_handle: asset_server.load("hive.png"),
        target_position: [0.0, -200.0].into(),
        hive_position: [-200.0, 300.0].into(),
        land_line_screen_coords: vec![],
    };
    info!("Start level loading");

    commands
        .spawn_empty()
        .insert(SpriteBundle {
            texture: level.bg_handle.clone(),
            ..default()
        })
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 1.0)))
        .insert(level);

    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                display: Display::Flex,
                flex_direction: FlexDirection::ColumnReverse,
                ..default()
            },
            ..default()
        })
        .insert(Loading)
        .with_children(|parent| {
            parent.spawn(TextBundle::from_sections([TextSection::new(
                "LOADING...",
                TextStyle {
                    font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                    font_size: 50.0,
                    color: Color::GRAY,
                },
            )]));
        });
}

pub fn cleanup_loading(mut commands: Commands, loading_query: Query<Entity, With<Loading>>) {
    for e in loading_query.iter() {
        commands.entity(e).despawn_recursive();
    }
}

pub fn wait_for_loading(
    mut commands: Commands,
    mut level_query: Query<&mut Level>,
    asset_server: Res<AssetServer>,
    assets: Res<Assets<Image>>,
    mut wnds: ResMut<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let mut level = level_query.single_mut();
    if level
        .all_handles()
        .iter()
        .all(|h| asset_server.get_load_state(*h) == LoadState::Loaded)
    {
        let win = wnds.get_primary_mut().unwrap();
        let (camera, camera_transform) = q_camera.single();

        let bg = assets.get(&level.control_plane_handle.clone()).unwrap();

        let control = scene_from_texture(bg);

        level.hive_position = get_world_coord_from_screen(
            [
                control.hive_screen_coords.x,
                win.height() - control.hive_screen_coords.y,
            ]
            .into(),
            win.width(),
            win.height(),
            camera,
            camera_transform,
        );

        level.target_position = get_world_coord_from_screen(
            [
                control.target_screen_coords.x,
                win.height() - control.target_screen_coords.y,
            ]
            .into(),
            win.width(),
            win.height(),
            camera,
            camera_transform,
        );

        level.land_line_screen_coords = control.land_line_screen_coords;

        commands.insert_resource(NextState(GameState::DrawDefence))
    }
}

pub fn cleanup_level(mut commands: Commands, level_query: Query<Entity, With<Level>>) {
    for e in level_query.iter() {
        commands.entity(e).despawn_recursive();
    }
}

fn scene_from_texture(bg: &Image) -> ControlPlaneInfo {
    let mut result = ControlPlaneInfo::default();

    let width = bg.size().x.floor() as usize;
    let height = bg.size().y.floor() as usize;
    let u8_per_pixel = bg.data.len() / (width * height);
    // first pixel has R:10 G:20 B:30 A: 0 or 255

    let color_marker = &bg.data[0..u8_per_pixel];

    info!(
        "Control plane {}x{}, u8 per pixel {}, color marker {}",
        width,
        height,
        u8_per_pixel,
        color_marker
            .iter()
            .map(|b| b.to_string())
            .collect::<Vec<String>>()
            .join(",")
    );

    for idx in 0..(width * height) {
        let y = idx / width;
        let x = idx - y * width;

        let start = idx * u8_per_pixel;

        let pixel = &bg.data[start..(start + u8_per_pixel)];

        if pixel[0] == 255 && pixel[1] == 0 && pixel[2] == 0 && pixel[3] == 255 {
            info!("Red pixel {}; {}x{}", idx, x, y);
            result.hive_screen_coords = [x as f32, y as f32].into();
        }

        if pixel[0] == 0 && pixel[1] == 255 && pixel[2] == 0 && pixel[3] == 255 {
            info!("Green pixel {}; {}x{}", idx, x, y);
            result.target_screen_coords = [x as f32, y as f32].into();
        }
    }

    for step in 0..(width / 10) {
        let x = step * 10;

        for y in 0..height {
            let start = y * width * u8_per_pixel + x * u8_per_pixel;
            let pixel = &bg.data[start..(start + u8_per_pixel)];

            if pixel[0] == 0 && pixel[1] == 0 && pixel[2] == 0 && pixel[3] == 255 {
                result
                    .land_line_screen_coords
                    .push([x as f32, (height - y) as f32].into());
                break;
            }
        }
    }

    let last = result.land_line_screen_coords.len() - 1;
    result.land_line_screen_coords[last].x = width as f32;

    result
}

#[derive(Default)]
struct ControlPlaneInfo {
    pub hive_screen_coords: Vec2,
    pub target_screen_coords: Vec2,
    pub land_line_screen_coords: Vec<Vec2>,
}

#[derive(Component)]
pub struct Level {
    pub bg_handle: Handle<Image>,
    pub control_plane_handle: Handle<Image>,
    pub dog_handle: Handle<Image>,
    pub wasp_handle: Handle<Image>,
    pub hive_handle: Handle<Image>,

    pub hive_position: Vec2,
    pub target_position: Vec2,
    pub land_line_screen_coords: Vec<Vec2>,
}

impl Level {
    pub fn all_handles(&self) -> Vec<&Handle<Image>> {
        vec![
            &self.bg_handle,
            &self.control_plane_handle,
            &self.dog_handle,
            &self.wasp_handle,
            &self.hive_handle,
        ]
    }
}

#[derive(Component)]
pub struct Loading;
