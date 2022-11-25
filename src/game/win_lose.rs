use bevy::prelude::*;
use iyes_loopless::prelude::*;

use super::{CurrentLevel, GameState};

pub fn init_win_lose(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    current_state: Res<CurrentState<GameState>>,
    mut current_level: ResMut<CurrentLevel>,
) {
    let mut text = "You WIN!!!";
    let mut text_color = Color::GREEN;

    match current_state.0 {
        GameState::Lose => {
            text = "You'been BITTEN!";
            text_color = Color::RED;
        }
        _ => {}
    };

    if current_state.0 == GameState::Win {
        current_level.value = current_level.value + 1;
        if current_level.value > 2 {
            current_level.value = 1;
        }
    }

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
        .insert(WinLose)
        .with_children(|parent| {
            parent.spawn(TextBundle::from_sections([TextSection::new(
                text,
                TextStyle {
                    font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                    font_size: 50.0,
                    color: text_color,
                },
            )]));
        });
}

pub fn cleanup_winlose(mut commands: Commands, winlose_query: Query<(Entity, &WinLose)>) {
    let (entity, _) = winlose_query.single();

    commands.entity(entity).despawn_recursive();
}

#[derive(Component)]
pub struct WinLose;
