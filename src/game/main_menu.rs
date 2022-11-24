use bevy::prelude::*;

pub fn init_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
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
        .insert(MainMenu)
        .with_children(|parent| {
            parent.spawn(TextBundle::from_sections([TextSection::new(
                "Defend the dog!!",
                TextStyle {
                    font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                    font_size: 50.0,
                    color: Color::BLUE,
                },
            )]));

            parent.spawn(TextBundle::from_sections([TextSection::new(
                "Press SPACE to start game",
                TextStyle {
                    font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                    font_size: 20.0,
                    color: Color::BLACK,
                },
            )]));

            parent.spawn(TextBundle::from_sections([TextSection::new(
                "Draw defence around the Dog via mouse and press Enter to spawn the Wasp Swarm",
                TextStyle {
                    font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                    font_size: 20.0,
                    color: Color::BLACK,
                },
            )]));

            parent.spawn(TextBundle::from_sections([TextSection::new(
                "Defence should stay for 20 seconds to win",
                TextStyle {
                    font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                    font_size: 20.0,
                    color: Color::BLACK,
                },
            )]));
        });
}

pub fn cleanup_menu(mut commands: Commands, menu_query: Query<(Entity, &MainMenu)>) {
    let (entity, _) = menu_query.single();

    commands.entity(entity).despawn_recursive();
}

#[derive(Component)]
pub struct MainMenu;
