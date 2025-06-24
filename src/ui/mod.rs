use super::*;

mod crosshair;
mod inventory_ui;
mod status_bar;
//mod dialog_ui;
mod settings;

use bevy_simple_text_input::TextInputPlugin;
use crosshair::*;
use inventory_ui::*;
use status_bar::*;
//use dialog_ui::*;
use settings::*;

#[derive(Component, Reflect, Default)]
pub struct UiIndex(pub i32);

#[derive(Component, Reflect, Default)]
pub struct ActiveUi;

#[derive(Component, Reflect)]
pub struct UiEntity(pub Entity);

#[derive(Component, Reflect)]
pub struct UiConsole;

#[derive(Component, Reflect)]
pub struct UiCrosshair;

#[derive(Component, Reflect)]
pub struct UiStatus;

#[derive(Component, Reflect)]
pub struct UiInventory;

#[derive(Component, Reflect)]
pub struct UiMenu;

#[derive(Component, Reflect)]
pub struct UiSettings;

#[derive(Component, Reflect)]
pub struct VideoUiMenu;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(TextInputPlugin)
            .add_systems(Startup, (
                    draw_inventory_ui,
                    draw_status_ui,
                    draw_crosshair,
                    draw_menu_ui,
            ));
    }
}
