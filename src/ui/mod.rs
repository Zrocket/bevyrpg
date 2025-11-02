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
pub use settings::*;


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

#[derive(Component, Reflect)]
pub struct UiCrosshair;

#[derive(Component, Reflect)]
pub struct UiStatus;

#[derive(Component, Reflect)]
pub struct UiInventory;

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(TextInputPlugin)
            .register_type::<DAUiAssets>()
            .add_plugins((
                    StatusUIPlugin,
                    InventoryUIPlugin,
                    CrosshairPlugin,
                    MenuUiPlugin,
            ))
            //.add_systems(OnEnter(GameState::Gameplay), (
                    //draw_inventory_ui,
                    //jonmo_draw_inventory_ui,
                    //draw_status_ui,
                    //jonmo_draw_status_ui,
                    //draw_crosshair,
                    //jonmo_draw_crosshair,
                    //draw_menu_ui,
            //        jonmo_draw_menu_ui,
            //))
            .add_loading_state(
                LoadingState::new(GameState::Preload)
                    .with_dynamic_assets_file::<StandardDynamicAssetCollection>("uiassets.ron")
                    .load_collection::<DAUiAssets>()
            );
    }
}
