use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use game::{
    enter_draw_defence, enter_test_defence, run_draw_defence, run_test_defence, startup, GameState,
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
        .add_loopless_state(GameState::DrawDefence)
        .add_system(bevy::window::close_on_esc)
        // Common
        .add_startup_system_set(startup())
        // Draw Defence
        .add_enter_system_set(GameState::DrawDefence, enter_draw_defence())
        .add_system_set(run_draw_defence())
        // Test defence
        .add_enter_system_set(GameState::TestDefence, enter_test_defence())
        .add_system_set(run_test_defence())
        //
        .run();
}
