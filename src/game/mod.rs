use self::controller::*;
use self::defence::*;
use self::level::*;
use self::main_menu::*;
use self::scene::*;
use self::swarm::*;
use self::target::*;
use self::win_lose::*;
use bevy::prelude::*;
use iyes_loopless::prelude::*;

mod controller;
mod defence;
mod level;
mod main_menu;
mod scene;
mod swarm;
mod target;
mod win_lose;

pub fn startup() -> SystemSet {
    SystemSet::new()
        .with_system(init_camera)
        .with_system(init_level)
}

pub fn game_common() -> SystemSet {
    SystemSet::new().with_system(controller)
}

pub fn enter_main_menu() -> SystemSet {
    ConditionSet::new().with_system(init_main_menu).into()
}

pub fn exit_main_menu() -> SystemSet {
    SystemSet::new().with_system(cleanup_menu)
}

pub fn enter_draw_defence() -> SystemSet {
    ConditionSet::new()
        .with_system(init_target)
        .with_system(init_scene)
        .with_system(create_hive)
        .with_system(init_defence_drawing)
        // .with_system(load_level)
        .into()
}

pub fn run_draw_defence() -> SystemSet {
    ConditionSet::new()
        .run_in_state(GameState::DrawDefence)
        .with_system(draw_defence_core)
        .with_system(update_defence_mesh)
        .into()
}

pub fn exit_draw_defence() -> SystemSet {
    SystemSet::new()
}

pub fn enter_test_defence() -> SystemSet {
    ConditionSet::new()
        .with_system(create_defence_collider)
        .with_system(spawn_wasps)
        .with_system(unlock_target)
        .into()
}

pub fn run_test_defence() -> SystemSet {
    ConditionSet::new()
        .run_in_state(GameState::TestDefence)
        .with_system(direct_wasps)
        .with_system(detect_wasp_sting)
        .into()
}

pub fn exit_test_defence() -> SystemSet {
    SystemSet::new()
        .with_system(cleanup_swarm)
        .with_system(cleanup_target)
        .with_system(cleanup_scene)
        .with_system(cleanup_defence)
}

pub fn enter_win_lose() -> SystemSet {
    ConditionSet::new().with_system(init_win_lose).into()
}

pub fn exit_win_lose() -> SystemSet {
    SystemSet::new().with_system(cleanup_winlose)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    MainMenu,
    DrawDefence,
    TestDefence,
    Win,
    Lose,
}
