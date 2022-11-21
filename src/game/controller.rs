use bevy::prelude::*;
use iyes_loopless::prelude::*;

use super::GameState;

pub fn controller(mut commands: Commands, keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::Return) {
        info!("SWITCH: TestDefence");
        commands.insert_resource(NextState(GameState::TestDefence))
    }
}
