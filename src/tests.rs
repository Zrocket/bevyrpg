use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use bevy_asset_loader::dynamic_asset::DynamicAssetCollections;

use crate::{AddToInventoryEvent, ApplicatorSubstance, CraftTag, DamageEvent, Description, Equiptable, FoamGunAttachment, GameState, Health, ItemDetails, ItemId, LabKey, Mana, ManaEvent, Player, SampleItem};
use super::Weight;

pub struct TestsPlugin;
impl Plugin for TestsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                    //dynamic_asset_test
                    //health_test,
                    //mana_test,
                    //inventory_add_test,
                    inventory_add_test.run_if(in_state(GameState::Gameplay)),
                    //inventory_remove_test,
                    //equipt_ui_test.run_if(input_just_pressed(KeyCode::KeyP)),
                    check_states.run_if(input_just_pressed(KeyCode::F2))
            ));
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
    } else {
        error!("health_test: Failed to query player");
    }
}

fn mana_test(
    mut commands: Commands,
    key: Res<ButtonInput<KeyCode>>,
    mut player: Query<(Entity, &Mana), With<Player>>,
) {
    trace!("SYSTEM: health_test");
    if let Ok((player_entity, _player)) = player.single_mut()
    && key.just_pressed(KeyCode::KeyC) {
        commands.entity(player_entity).trigger(|entity| ManaEvent { entity, ammount: 5 });
    } else {
        error!("mana_test: Failed to query player");
    }
}

fn inventory_add_test(
    mut commands: Commands,
    key: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<Entity, With<Player>>,
    item_database: Res<crate::ItemDatabase>,
    asset_server: Res<AssetServer>,
) {
    trace!("SYSTEM: inventory_add_test");
    if let Ok(player) = player_query.single_mut() && key.just_pressed(KeyCode::KeyJ) {

       let Some(item_def) = item_database.0.get("tin_cup") else {
           return;
       };

       let item = crate::spawn_item_from_definition(&mut commands, &asset_server, item_def);

       commands.entity(item)
           .insert((
                LabKey,
                Name::new("tin_cup"),
                Equiptable {
                    slot: crate::EquipSlot::Arm,
                    defense: 1,
                },
                CraftTag("test".into()),
                SampleItem {
                    analyzed: false,
                    botched: false,
                },
                ApplicatorSubstance,
                FoamGunAttachment,
           ));

        println!("{:?}", item);
        commands.entity(player).trigger(|entity| AddToInventoryEvent { entity, item });
    } else {
        trace!("inventory_add_test: Failed to query player");
    }
}

fn check_states(
    meta_state: Res<State<crate::MetaState>>,
    menu_state: Res<State<crate::MenuState>>,
) {
    println!("{:?}", meta_state.get());
    println!("{:?}", menu_state.get());
}
