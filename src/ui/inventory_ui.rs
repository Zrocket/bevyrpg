use super::*;
use bevy::color::palettes::css::CRIMSON;

pub fn draw_inventory_ui(
    mut commands: Commands,
    items: Query<(Entity, &Name, &InInventory)>,
    inventory: Query<&Inventory, With<Player>>,
    target: Query<Entity, With<ActiveInventoryUi>>,
    _asset_server: Res<AssetServer>,
) {
    trace!("draw_inventory_ui");
    for target_entity in target.iter() {
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
                    ..default()
                },
                BackgroundColor(CRIMSON.into()),
                UiEntity(target_entity),
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

        //info!("Drawing InventoryUi");
        /*commands.ui_builder(UiRoot).container(Node {
                background_color: CRIMSON.into(),
                position_type: PositionType::Absolute,
                width: Val::Percent(80.),
                height: Val::Percent(80.),
                left: Val::Percent(10.),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_self: AlignSelf::Center,
                flex_wrap: FlexWrap::Wrap,
                ..default()
        },
        |inventory_menu| {
            inventory_menu.column(|column| {
                if let inventory = inventory.single() {
                    for item in inventory.items.iter() {
                        if let Ok((_id, item_name, _)) = items.get(*item) {
                            column.row(|row| {
                                row.container(
                                    ButtonBundle {
                                        ..default()
                                    },
                                    |parent| {
                                        parent.spawn(TextBundle {
                                            text: Text::from_section(item_name, TextStyle {
                                            font: asset_server.load("FiraSans-Bold.ttf"),
                                            font_size: 50.0,
                                            color: Color::WHITE,
                                        }),
                                        ..default()
                                        });
                                    }
                                );
                            });
                        }
                    }
                }
            });
        })
        .insert(UiEntity(target_entity))
        .insert(UiInventory);*/
    }
}
