use bevy::prelude::*;

pub fn init_level(mut commands: Commands) {
    commands.spawn().insert(Level { level_num: 0 });
}

pub fn load_level(mut commands: Commands, level_query: Query<(Entity, &Level), With<Level>>) {
    info!("Load level");
    for (entity, level) in level_query.iter() {
        commands.entity(entity).despawn_descendants();
        info!("Loading level {}", level.level_num);
    }
}

#[derive(Component)]
pub struct Level {
    pub level_num: u8,
}
