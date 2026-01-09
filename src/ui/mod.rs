use crate::{widgets::{anchored::AnchoredUiPlugin, floating_window_ordering::FloatingWindowOrderingPlugin, floating_windows::FloatingWindowPlugin}};

use super::*;

pub mod book_ui;
pub mod crosshair;
pub mod equip_ui;
pub mod inventory_ui;
pub mod status_bar;
//mod dialog_ui;
pub mod widgets;
pub mod menu;
pub mod palette;
pub mod inspect;
pub mod start_menu;
pub mod game_over;
pub mod stats_ui;

use bevy_simple_text_input::TextInputPlugin;
use crosshair::*;
pub use equip_ui::*;
pub use inventory_ui::*;
use status_bar::*;
//use dialog_ui::*;
pub use menu::*;
pub use inspect::*;
pub use start_menu::*;
pub use game_over::*;
pub use stats_ui::*;


#[derive(AssetCollection, Resource, Reflect, Debug)]
#[reflect(Resource)]
pub struct DAUiAssets {
    #[asset(key = "health_1")]
    health_1: Handle<Image>,
    #[asset(key = "health_2")]
    health_2: Handle<Image>,
    #[asset(key = "health_3")]
    health_3: Handle<Image>,
}

#[derive(Debug, Clone, Component, Reflect)]
pub struct Hoverable(bool);

#[derive(Component, Reflect, Default)]
pub struct UiIndex(pub i32);

#[derive(Component, Reflect, Default)]
pub struct ActiveUi;

#[derive(Component, Reflect)]
pub struct UiEntity(pub Entity);

#[derive(Component, Reflect)]
pub struct UiConsole;

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(TextInputPlugin)
            .register_type::<DAUiAssets>()
            .add_plugins((
                    AnchoredUiPlugin,
                    StatusUIPlugin,
                    InspectUiPlugin,
                    InventoryUIPlugin,
                    CrosshairPlugin,
                    MenuUiPlugin,
                    GameOverUiPlugin,
                    StartMenuUiPlugin,
                    FloatingWindowPlugin,
                    FloatingWindowOrderingPlugin,
                    EquipUiPlugin,
            ))
            .add_loading_state(
                LoadingState::new(GameState::Preload)
                    .with_dynamic_assets_file::<StandardDynamicAssetCollection>("uiassets.ron")
                    .load_collection::<DAUiAssets>()
            );
    }
}
