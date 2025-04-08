use super::*;

mod console;
mod crosshair;
mod inventory_ui;
mod status_bar;
//mod dialog_ui;
mod settings;

use bevy_simple_text_input::TextInputPlugin;
use console::*;
use crosshair::*;
use inventory_ui::*;
use status_bar::*;
//use dialog_ui::*;
use settings::*;

#[derive(Component, Reflect, Default)]
pub struct UiIndex(pub i32);

#[derive(Component, Reflect, Default)]
pub struct ActiveConsole;

#[derive(Event)]
pub struct UiInventoryEvent {
    pub actor: Entity,
}

#[derive(Component, Reflect, Default)]
pub struct ActiveUi;

#[derive(Component, Reflect, Default)]
pub struct ActiveMenuUi;

#[derive(Component, Reflect, Default)]
pub struct ActiveVideoMenuUi;

#[derive(Component, Reflect, Default)]
pub struct ActiveInventoryUi;

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
pub struct VideoUiMenu;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<UiInventoryEvent>()
            .add_plugins(TextInputPlugin)
            .add_systems(Startup, draw_inventory_ui)
            //.add_systems(Update, draw_menu_ui::<InInventory>)
            .add_systems(Startup, draw_status_ui)
            .add_systems(Startup, draw_console_ui)
            .add_systems(Startup, draw_crosshair)
            .add_systems(Startup, draw_menu_ui)
            .add_systems(
                Update,
                inventory_ui_event_handler.run_if(in_state(GameState::Gameplay)),
            );
            /*.add_systems(Update, cleanup_system::<UiEntity>)
            .add_systems(Update, cleanup_system::<UiStatus>)
            .add_systems(Update, cleanup_system::<UiConsole>)
            .add_systems(Update, cleanup_system::<UiCrosshair>)
            .add_systems(Update, cleanup_system::<UiMenu>)
            .add_systems(Update, cleanup_system::<UiInventory>);*/
    }
}

pub fn inventory_ui_event_handler(
    mut commands: Commands,
    mut inventory_ui_events: EventReader<UiInventoryEvent>,
) {
    for event in inventory_ui_events.read() {
        commands.entity(event.actor).insert(ActiveUi);
    }
}
