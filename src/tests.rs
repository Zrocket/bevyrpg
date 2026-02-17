use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use bevy_asset_loader::dynamic_asset::DynamicAssetCollections;

use crate::{AddToInventoryEvent, DamageEvent, Description, DisplayEquipEvent, Equiptable, GameState, Health, Item, Player};
use super::Weight;

pub struct TestsPlugin;
impl Plugin for TestsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                    //dynamic_asset_test
                    health_test,
                    //inventory_add_test,
                    inventory_add_test,
                    //inventory_remove_test,
                    equipt_ui_test.run_if(input_just_pressed(KeyCode::KeyP)),
            ));
    }
}

fn equipt_ui_test(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
) {
    if let Ok(entity) = player_query.single() {
        commands.entity(entity).trigger(|entity| DisplayEquipEvent { entity });
    }
}

fn _dynamic_asset_test(
    dynamic_assets: Res<DynamicAssetCollections<GameState>>,
    //level_asset: Res<DALevelAsset>,
) {
    println!(" DYNAMICASSETS: {:?}", dynamic_assets);
    //println!("LEVELASSET: {:?}", level_asset);
}

fn health_test(
    mut commands: Commands,
    key: Res<ButtonInput<KeyCode>>,
    mut player: Query<(Entity, &Health), With<Player>>,
) {
    trace!("SYSTEM: health_test");
    if let Ok((player_entity, _player)) = player.single_mut()
    && key.just_pressed(KeyCode::KeyV) {
        commands.entity(player_entity).trigger(|entity| DamageEvent { entity, ammount: 5 });
    }
}

fn inventory_add_test(
    mut commands: Commands,
    key: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<Entity, With<Player>>,
) {
    trace!("SYSTEM: inventory_add_test");
    if let Ok(player) = player_query.single_mut() && key.just_pressed(KeyCode::KeyJ) {
        let item = commands.spawn((
                Item {
                    description: Description("Test".to_string()),
                    weight: Weight(0),
                },
                Name::new(format!("Test {}", rand::random::<u8>() as char)),
                Equiptable {
                    slot: crate::EquipSlot::Arm,
                    defense: 1,
                }
        )).id();
        println!("{:?}", item);
        commands.entity(player).trigger(|entity| AddToInventoryEvent { entity, item });
    }
}
