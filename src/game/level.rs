use bevy::{asset::LoadState, prelude::*};
use iyes_loopless::prelude::*;

use super::{CurrentLevel, GameState};

pub fn start_level_loading(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    current_level: Res<CurrentLevel>,
) {
    commands.insert_resource(NextState(GameState::DrawDefence));

    return;

    let bg_handle: Handle<Image> =
        asset_server.load(format!("levels/{}/level.png", current_level.value).as_str());
    let level = Level {
        bg_handle: bg_handle.clone(),
    };
    info!("Start level loading");

    commands
        .spawn(level)
        .insert(SpriteBundle {
            texture: bg_handle,
            ..default()
        })
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 1.0)));

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
    level_query: Query<&Level>,
    asset_server: Res<AssetServer>,
) {
    let level = level_query.single();
    if asset_server.get_load_state(&level.bg_handle) == LoadState::Loaded {
        commands.insert_resource(NextState(GameState::DrawDefence))
    }
}

pub fn cleanup_level(mut commands: Commands, level_query: Query<Entity, With<Level>>) {
    for e in level_query.iter() {
        commands.entity(e).despawn_recursive();
    }
}

#[derive(Component)]
pub struct Level {
    pub bg_handle: Handle<Image>,
}

#[derive(Component)]
pub struct Loading;
