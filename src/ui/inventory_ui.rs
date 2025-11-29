use super::*;
use bevy::color::palettes::css::CRIMSON;

#[derive(Component, Reflect)]
pub struct UiInventory;

pub struct InventoryUIPlugin;
impl Plugin for InventoryUIPlugin {
    fn build(&self, app: &mut App) {
       app
           .add_systems(OnEnter(GameState::Gameplay), spawn_inventory_ui);
    }
}

pub fn spawn_inventory_ui(
    mut commands: Commands,
    items: Query<(Entity, &Name, &InInventory)>,
    inventory: Query<&Inventory, With<Player>>,
    _asset_server: Res<AssetServer>,
) {
    trace!("SYSTEM: draw_inventory_ui");
    let _inventory_root = commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(80.),
                height: Val::Percent(80.),
                left: Val::Percent(10.),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_self: AlignSelf::Center,
                flex_wrap: FlexWrap::Wrap,
                display:  Display::None,
                ..default()
            },
            BackgroundColor(CRIMSON.into()),
            UiInventory,
        ))
        .with_children(|inventory_window| {
            if let Ok(inventory_handle) = inventory.single() {
                for item in inventory_handle.items.iter() {
                    if let Ok((_id, item_name, _)) = items.get(*item) {
                        inventory_window.spawn((Text(item_name.to_string()),));
                    }
                }
            }
        })
        .id();
}

/*pub fn jonmo_draw_inventory_ui(
    world: &mut World,
    items: &mut QueryState<(Entity, &Name, &InInventory)>,
    inventory: &mut QueryState<&Inventory, With<Player>>,
) {
    trace!("draw_inventory_ui");
    let mut inventory_items: Vec<JonmoBuilder> = Vec::new();
    if let Ok(inventory_handle) = inventory.single(world) {
        for item in inventory_handle.items.iter() {
            if let Ok((_id, item_name, _)) = items.get(world, *item) {
                inventory_items.push(JonmoBuilder::from(Text(item_name.to_string())));
            }
        }
    }
    let _inventory_root = JonmoBuilder::from((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(80.),
                height: Val::Percent(80.),
                left: Val::Percent(10.),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_self: AlignSelf::Center,
                flex_wrap: FlexWrap::Wrap,
                display:  Display::None,
                ..default()
            },
            BackgroundColor(CRIMSON.into()),
            UiInventory,
    ))
    .children(inventory_items)
    .spawn(world);
}*/
