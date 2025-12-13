use bevy::{color::palettes::css::{CRIMSON, DARK_CYAN, DARK_GREEN, DARK_VIOLET}, prelude::*};
use bevy_asset_loader::dynamic_asset::DynamicAssetCollections;

use crate::{AddToInventoryEvent, ChangeScreenEvent, DamageMessage, Description, GameState, Health, Inventory, Item, Player, RemoveMessage, display_inventory_event_observer, new_computer_screen, widgets::floating_windows::{FloatingWindow, close_button, floating_window_root, init_floating_window, minimize_button, resizable_borders} };
use super::Weight;

pub struct TestsPlugin;
impl Plugin for TestsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                    //dynamic_asset_test
                    //computer_test,
                    health_test,
                    //inventory_add_test,
                    inventory_add_test,
                    //inventory_remove_test,
                    floating_window_test,
            ));
    }
}

fn floating_window_test(
    mut commands: Commands,
    key: Res<ButtonInput<KeyCode>>,
) {
    if key.just_pressed(KeyCode::KeyP) {
        commands.spawn(floating_window_root("TEST WINDOW!".into(), ()));
    }
}

fn _dynamic_asset_test(
    dynamic_assets: Res<DynamicAssetCollections<GameState>>,
    //level_asset: Res<DALevelAsset>,
) {
    println!(" DYNAMICASSETS: {:?}", dynamic_assets);
    //println!("LEVELASSET: {:?}", level_asset);
}

/*fn _computer_test (
    key: Res<ButtonInput<KeyCode>>,
    mut event_writer: MessageWriter<ChangeScreenEvent>
) {
    trace!("SYSTEM: computer_test");

    if key.just_pressed(KeyCode::KeyK) {
        event_writer.write(ChangeScreenEvent { frame_closure: new_computer_screen });
    }
}*/

fn health_test(
    key: Res<ButtonInput<KeyCode>>,
    mut player: Query<(Entity, &Health), With<Player>>,
    mut damage_event_writer: MessageWriter<DamageMessage>,
) {
    trace!("SYSTEM: health_test");
    if let Ok((player_entity, _player)) = player.single_mut() {
        if key.just_pressed(KeyCode::KeyK) {
            damage_event_writer.write(DamageMessage {
                target: player_entity,
                ammount: 5,
            });
        }
    }
}

fn inventory_add_test(
    mut commands: Commands,
    key: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<Entity, With<Player>>,
) {
    trace!("SYSTEM: inventory_add_test");
    if let Ok(mut player) = player_query.single_mut() && key.just_pressed(KeyCode::KeyJ) {
        let item = commands.spawn((
                Item {
                    description: Description("Test".to_string()),
                    weight: Weight(0),
                },
                Name::new(format!("Test {}", rand::random::<u8>() as char)),
        )).id();
        println!("{:?}", item);
        commands.entity(player).trigger(|entity| AddToInventoryEvent { entity, item });
    }
}

fn inventory_remove_test(
    key: Res<ButtonInput<KeyCode>>,
    mut player: Query<Entity, With<Player>>,
    mut inventory_query: Query<&Inventory, With<Player>>,
    mut event_writer: MessageWriter<RemoveMessage>,
) {
    trace!("SYSTEM: inventory_remove_test");
    if let Ok(mut player) = player.single_mut() {
        if key.just_pressed(KeyCode::KeyL) {
            let inventory = inventory_query.single_mut().unwrap();
            let item = inventory.items.last().unwrap();
            event_writer.write(RemoveMessage {
                actor: player,
                target: *item,
            });
        }
    }
}
