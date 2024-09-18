use bevy::color::palettes::css::CRIMSON;
use sickle_ui::{ui_builder::{UiBuilderExt, UiRoot}, widgets::layout::{column::UiColumnExt, container::UiContainerExt}};

use super::*;

pub fn draw_inventory_ui(
    mut commands: Commands,
    items: Query<(Entity, &Name, &InInventory)>,
    target: Query<Entity, With<ActiveInventoryUi>>,
    asset_server: Res<AssetServer>,
    ) {
    for target_entity in target.iter() {
        info!("Drawing InventoryUi");
        commands.ui_builder(UiRoot).container(NodeBundle {
                background_color: CRIMSON.into(),
                style: Style {
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
                ..default()
        },
        |inventory_menu| {
            inventory_menu.column(|column| {
                info!("Drawing InventoryUi Entries");
                for (_item_entity, item_name, _in_inventory) in items.iter() {
                    column.spawn(TextBundle {
                        text: Text::from_section(item_name, TextStyle {
                            font: asset_server.load("FiraSans-Bold.ttf"),
                            font_size: 50.0,
                            color: Color::WHITE,
                        }),
                        ..default()
                    });
                }
            });
        })
        .insert(UiEntity(target_entity))
        .insert(UiInventory);
    }
}
