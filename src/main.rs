use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use game::{
    enter_draw_defence, enter_load_level, enter_main_menu, enter_test_defence, enter_win_lose,
    exit_draw_defence, exit_loading_level, exit_main_menu, exit_test_defence, exit_win_lose,
    game_common, run_draw_defence, run_loading_level, run_test_defence, startup, CurrentLevel,
    GameParameters, GameState,
};
use iyes_loopless::prelude::*;

mod common;
mod game;
mod random;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        //
        .insert_resource(ClearColor(Color::WHITE))
        .insert_resource(GameParameters::default())
        .insert_resource(CurrentLevel { value: 1 })
        .insert_resource(RapierConfiguration {
            timestep_mode: TimestepMode::Variable {
                max_dt: 1.0 / 60.0,
                time_scale: 1.0,
                substeps: 10,
            },
            ..default()
        })
        .add_loopless_state(GameState::MainMenu)
        .add_system(bevy::window::close_on_esc)
        // Common
        .add_startup_system_set(startup())
        .add_system_set(game_common())
        // Main menu
        .add_enter_system_set(GameState::MainMenu, enter_main_menu())
        .add_exit_system_set(GameState::MainMenu, exit_main_menu())
        // Load level
        .add_enter_system_set(GameState::LoadLevel, enter_load_level())
        .add_system_set(run_loading_level())
        .add_exit_system_set(GameState::LoadLevel, exit_loading_level())
        // Draw Defence
        .add_enter_system_set(GameState::DrawDefence, enter_draw_defence())
        .add_system_set(run_draw_defence())
        .add_exit_system_set(GameState::DrawDefence, exit_draw_defence())
        // Test defence
        .add_enter_system_set(GameState::TestDefence, enter_test_defence())
        .add_system_set(run_test_defence())
        .add_exit_system_set(GameState::TestDefence, exit_test_defence())
        // Win/Lose
        .add_enter_system_set(GameState::Win, enter_win_lose())
        .add_exit_system_set(GameState::Win, exit_win_lose())
        .add_enter_system_set(GameState::Lose, enter_win_lose())
        .add_exit_system_set(GameState::Lose, exit_win_lose())
        //
        .run();
}
