use bevy::app::Plugin;
use bevy_enhanced_input::prelude::{InputAction, EnhancedInputPlugin, InputContextAppExt};

use crate::Player;

pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app
            .add_plugins(EnhancedInputPlugin)
            .add_input_context::<Player>();
    }
}

#[derive(InputAction)]
#[action_output(bool)]
pub struct Weapon1Action;

#[derive(InputAction)]
#[action_output(bool)]
pub struct Weapon2Action;

#[derive(InputAction)]
#[action_output(bool)]
pub struct Weapon3Action;

#[derive(InputAction)]
#[action_output(bool)]
pub struct Weapon4Action;

#[derive(InputAction)]
#[action_output(bool)]
pub struct RunAction;

#[derive(InputAction)]
#[action_output(bool)]
pub struct JumpAction;

#[derive(InputAction)]
#[action_output(bool)]
pub struct ForwardAction;

#[derive(InputAction)]
#[action_output(bool)]
pub struct BackwardAction;

#[derive(InputAction)]
#[action_output(bool)]
pub struct LeftAction;

#[derive(InputAction)]
#[action_output(bool)]
pub struct RightAction;

#[derive(InputAction)]
#[action_output(bool)]
pub struct CrouchAction;

#[derive(InputAction)]
#[action_output(bool)]
pub struct UpAction;

#[derive(InputAction)]
#[action_output(bool)]
pub struct DownAction;

#[derive(InputAction)]
#[action_output(bool)]
pub struct InteractAction;

#[derive(InputAction)]
#[action_output(bool)]
pub struct Interact2Action;

#[derive(InputAction)]
#[action_output(bool)]
pub struct OpenInventoryAction;

#[derive(InputAction)]
#[action_output(bool)]
pub struct OpenEquipAction;

#[derive(InputAction)]
#[action_output(bool)]
pub struct OpenStatsAction;

#[derive(InputAction)]
#[action_output(bool)]
pub struct OpenConsoleAction;

#[derive(InputAction)]
#[action_output(bool)]
pub struct FlashlightAction;
