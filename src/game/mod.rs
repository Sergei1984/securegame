use self::ai::*;
use self::controller::*;
use self::defence::*;
use self::level::*;
use self::main_menu::*;
use self::scene::*;
use self::swarm::*;
use self::target::*;
use self::win_lose::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::Damping;
use bevy_rapier2d::prelude::Group;
use iyes_loopless::prelude::*;

mod ai;
mod controller;
mod defence;
mod level;
mod main_menu;
mod scene;
mod swarm;
mod target;
mod win_lose;

pub fn startup() -> SystemSet {
    SystemSet::new().with_system(init_camera)
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

pub fn enter_load_level() -> SystemSet {
    SystemSet::new().with_system(start_level_loading)
}

pub fn run_loading_level() -> SystemSet {
    ConditionSet::new()
        .run_in_state(GameState::LoadLevel)
        .with_system(wait_for_loading)
        .into()
}

pub fn exit_loading_level() -> SystemSet {
    SystemSet::new().with_system(cleanup_loading)
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
        .with_system(update_drawing_defence_mesh)
        .into()
}

pub fn exit_draw_defence() -> SystemSet {
    SystemSet::new()
}

pub fn enter_test_defence() -> SystemSet {
    ConditionSet::new()
        .with_system(create_defence_collider)
        .with_system(create_final_defence_mesh)
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
        .with_system(cleanup_level)
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
    LoadLevel,
    DrawDefence,
    TestDefence,
    Win,
    Lose,
}

#[derive(Resource)]
pub struct GameParameters {
    pub wasp_radius: f32,
    pub target_radius: f32,
    pub wasp: EntityPhysicsParams,
    pub defence: EntityPhysicsParams,
    pub target: EntityPhysicsParams,
    pub scene: EntityPhysicsParams,
    pub scene_group: Group,
}

impl Default for GameParameters {
    fn default() -> Self {
        Self {
            target_radius: 30.0,
            wasp_radius: 10.0,
            scene_group: Group::GROUP_32,
            wasp: EntityPhysicsParams {
                mass: 10000.0,
                damping: Damping {
                    linear_damping: 0.02,
                    angular_damping: 1.0,
                },
                restitution: 0.9,
                friction: 20.0,
            },
            defence: EntityPhysicsParams {
                mass: 10.0,
                damping: Damping {
                    linear_damping: 0.2,
                    angular_damping: 0.2,
                },
                restitution: 0.9,
                friction: 0.01,
            },
            target: EntityPhysicsParams {
                mass: 10.0,
                damping: Damping {
                    linear_damping: 0.8,
                    angular_damping: 0.9,
                },
                restitution: 0.98,
                friction: 0.01,
            },
            scene: EntityPhysicsParams {
                mass: 100_000.0,
                damping: Damping {
                    linear_damping: 0.8,
                    angular_damping: 0.9,
                },
                restitution: 0.8,
                friction: 0.1,
            },
        }
    }
}

pub struct EntityPhysicsParams {
    pub mass: f32,
    pub damping: Damping,
    pub restitution: f32,
    pub friction: f32,
}

#[derive(Resource)]
pub struct CurrentLevel {
    pub value: i32,
}
