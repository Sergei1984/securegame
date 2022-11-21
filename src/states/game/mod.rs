use self::defence::*;
use self::scene::*;
use self::swarm::*;
use self::target::*;
use bevy::prelude::*;
use iyes_loopless::prelude::*;

use super::GameState;

mod defence;
mod scene;
mod swarm;
mod target;

pub fn startup() -> SystemSet {
    SystemSet::new()
        .with_system(scene_system_startup)
        .with_system(target_system_startup)
}

pub fn enter_draw_defence() -> SystemSet {
    ConditionSet::new()
        .with_system(defence_system_startup)
        .into()
}

pub fn run_draw_defence() -> SystemSet {
    ConditionSet::new()
        .run_in_state(GameState::DrawDefence)
        .with_system(draw_defence_core)
        .with_system(update_defence_mesh)
        .into()
}

pub fn enter_test_defence() -> SystemSet {
    ConditionSet::new()
        .with_system(create_defence_collider)
        .with_system(create_hive)
        .with_system(spawn_wasps)
        .into()
}

pub fn run_test_defence() -> SystemSet {
    ConditionSet::new()
        .run_in_state(GameState::TestDefence)
        .with_system(direct_wasps)
        .with_system(unlock_target)
        .into()
}
