use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use game::{
    enter_draw_defence, enter_main_menu, enter_test_defence, exit_main_menu, game_common,
    run_draw_defence, run_test_defence, startup, GameState,
};
use iyes_loopless::prelude::*;

mod common;
mod game;
mod random;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        //
        .insert_resource(ClearColor(Color::WHITE))
        //
        .add_loopless_state(GameState::MainMenu)
        .add_system(bevy::window::close_on_esc)
        // Common
        .add_startup_system_set(startup())
        .add_system_set(game_common())
        // Main menu
        .add_enter_system_set(GameState::MainMenu, enter_main_menu())
        .add_exit_system_set(GameState::MainMenu, exit_main_menu())
        // Draw Defence
        .add_enter_system_set(GameState::DrawDefence, enter_draw_defence())
        .add_system_set(run_draw_defence())
        // Test defence
        .add_enter_system_set(GameState::TestDefence, enter_test_defence())
        .add_system_set(run_test_defence())
        //
        .run();
}
