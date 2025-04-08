use super::*;
use bevy::color::palettes::css::CRIMSON;

pub fn draw_inventory_ui(
    mut commands: Commands,
    items: Query<(Entity, &Name, &InInventory)>,
    inventory: Query<&Inventory, With<Player>>,
    _asset_server: Res<AssetServer>,
) {
    info!("draw_inventory_ui");
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
            let inventory_handle = inventory.single();

            for item in inventory_handle.items.iter() {
                if let Ok((_id, item_name, _)) = items.get(*item) {
                    inventory_window.spawn((Text(item_name.to_string()),));
                }
            }
        })
        .id();
}
