use bevy::prelude::*;
use bevy_rapier2d::prelude::RapierConfiguration;
use iyes_loopless::prelude::*;

use super::GameState;

pub fn controller(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    current_state: Res<CurrentState<GameState>>,
    rapier_config: Res<RapierConfiguration>,
) {
    match current_state.0 {
        GameState::MainMenu => {
            if keyboard_input.just_pressed(KeyCode::Space) {
                info!("SWITCH: Draw Defence");
                commands.insert_resource(NextState(GameState::LoadLevel))
            }
        }
        GameState::DrawDefence => {
            if keyboard_input.just_pressed(KeyCode::Return) {
                info!("SWITCH: TestDefence");
                commands.insert_resource(NextState(GameState::TestDefence))
            }
        }
        GameState::TestDefence => {
            if !rapier_config.physics_pipeline_active {
                if keyboard_input.just_pressed(KeyCode::Space) {
                    commands.insert_resource(NextState(GameState::Lose))
                }
            }
        }
        GameState::Win => {
            if keyboard_input.just_pressed(KeyCode::Space) {
                info!("SWITCH: TestDefence");
                commands.insert_resource(NextState(GameState::LoadLevel))
            }
        }
        GameState::Lose => {
            if keyboard_input.just_pressed(KeyCode::Space) {
                info!("SWITCH: TestDefence");
                commands.insert_resource(NextState(GameState::LoadLevel))
            }
        }
        _ => {}
    }
}
