use bevy::prelude::*;
use bevy_asset_loader::dynamic_asset::DynamicAssetCollections;

use crate::{new_computer_screen, ChangeScreenEvent, DamageEvent, Description, GameState, Health, Inventory, Item, PickUpEvent, Player, RemoveEvent, };
use super::Weight;

pub struct TestsPlugin;
impl Plugin for TestsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                    dynamic_asset_test
                    //computer_test,
                    //health_test,
                    //inventory_add_test,
                    //inventory_remove_test,
            ));
    }
}


fn dynamic_asset_test(
    dynamic_assets: Res<DynamicAssetCollections<GameState>>,
    //level_asset: Res<DALevelAsset>,
) {
    println!(" DYNAMICASSETS: {:?}", dynamic_assets);
    //println!("LEVELASSET: {:?}", level_asset);
}

fn _computer_test (
    key: Res<ButtonInput<KeyCode>>,
    mut event_writer: EventWriter<ChangeScreenEvent>
) {
    trace!("SYSTEM: computer_test");

    if key.just_pressed(KeyCode::KeyK) {
        event_writer.write(ChangeScreenEvent { frame_closure: new_computer_screen });
    }
}

fn _health_test(
    key: Res<ButtonInput<KeyCode>>,
    mut player: Query<(Entity, &Health), With<Player>>,
    mut damage_event_writer: EventWriter<DamageEvent>,
) {
    trace!("SYSTEM: health_test");
    let (player_entity, _player) = player.single_mut().unwrap();
    if key.just_pressed(KeyCode::KeyK) {
        damage_event_writer.write(DamageEvent {
            target: player_entity,
            ammount: 5,
        });
    }
}

fn _inventory_add_test(
    mut commands: Commands,
    key: Res<ButtonInput<KeyCode>>,
    mut player: Query<Entity, With<Player>>,
    mut event_writer: EventWriter<PickUpEvent>,
) {
    trace!("SYSTEM: inventory_add_test");
    let player = player.single_mut().unwrap();
    if key.just_pressed(KeyCode::KeyJ) {
        let item = commands
            .spawn((Item {
                name: Name::new(format!("Test {}", rand::random::<u8>() as char)),
                description: Description("Test".to_string()),
                weight: Weight(0),
            },))
            .id();
        event_writer.write(PickUpEvent {
            actor: player,
            target: item,
        });
    }
}

fn _inventory_remove_test(
    key: Res<ButtonInput<KeyCode>>,
    mut player: Query<Entity, With<Player>>,
    mut inventory_query: Query<&Inventory, With<Player>>,
    mut event_writer: EventWriter<RemoveEvent>,
) {
    trace!("SYSTEM: inventory_remove_test");
    let player = player.single_mut().unwrap();
    if key.just_pressed(KeyCode::KeyL) {
        let inventory = inventory_query.single_mut().unwrap();
        let item = inventory.items.last().unwrap();
        event_writer.write(RemoveEvent {
            actor: player,
            target: *item,
        });
    }
}
