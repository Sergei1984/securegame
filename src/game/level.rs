use bevy::prelude::*;

pub fn init_level(mut commands: Commands) {
    commands.spawn().insert(Level { level_num: 1 });
}

pub fn load_level(
    mut commands: Commands,
    // level_query: Query<(Entity, &Level), With<Level>>,
    asset_server: Res<AssetServer>,
) {
    info!("Load level");
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load(format!("levels/{}/level.png", 1).as_str()),
            ..default()
        })
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 1.0)));
}

#[derive(Component)]
pub struct Level {
    pub level_num: u8,
}
