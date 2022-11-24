use bevy::prelude::*;
use iyes_loopless::prelude::*;

use super::GameState;

pub fn init_win_lose(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    current_state: Res<CurrentState<GameState>>,
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
