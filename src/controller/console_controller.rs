use crate::{Player, UiConsole};
use bevy::prelude::*;
use bevy_simple_text_input::TextInputInactive;
use leafwing_input_manager::prelude::ActionState;

use super::Action;

pub fn manage_console(
    key: Query<&ActionState<Action>, With<Player>>,
    mut console_query: Query<(&mut Node, &mut TextInputInactive), With<UiConsole>>,
) {
    if let Ok(key) = key.get_single() {
        if key.just_pressed(&Action::OpenConsole) {
            info!("Console key pressed");
            if let Ok((mut console, mut text_inactive)) = console_query.get_single_mut() {
                match console.display {
                    Display::None =>  {
                        console.display = Display::Flex;
                        text_inactive.0 = false;
                    },
                    _ =>  {
                        console.display = Display::None;
                        text_inactive.0 = true;
                    }
                }
            }
        }
    }
}
