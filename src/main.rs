use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use iyes_loopless::prelude::*;
use states::{
    enter_draw_defence, enter_test_defence, run_draw_defence, run_test_defence, startup, GameState,
    SpawnSwarmEvent,
};

mod common;
mod random;
mod states;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_event::<SpawnSwarmEvent>()
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

fn spawn_swarm(
    keyboard_input: Res<Input<KeyCode>>,
    mut spawn_swarm_writer: EventWriter<SpawnSwarmEvent>,
) {
    if keyboard_input.just_pressed(KeyCode::Return) {
        info!("Spawning a swarm");
        spawn_swarm_writer.send(SpawnSwarmEvent);
    }
}
