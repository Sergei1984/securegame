use bevy::prelude::*;
use iyes_loopless::prelude::*;

use super::GameState;

pub fn controller(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    current_state: Res<CurrentState<GameState>>,
) {
    match current_state.0 {
        GameState::MainMenu => {
            if keyboard_input.just_pressed(KeyCode::Space) {
                info!("SWITCH: Draw Defence");
                commands.insert_resource(NextState(GameState::DrawDefence))
            }
        }
        GameState::DrawDefence => {
            if keyboard_input.just_pressed(KeyCode::Return) {
                info!("SWITCH: TestDefence");
                commands.insert_resource(NextState(GameState::TestDefence))
            }
        }
        _ => {}
    }
}
