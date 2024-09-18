use bevy::color::palettes::css::{BLUE, GREEN};
use sickle_ui::{ui_builder::{UiBuilderExt, UiRoot}, widgets::layout::{container::UiContainerExt, row::UiRowExt}};

use super::*;

pub fn draw_status_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    health_query: Query<(&Health, &MaxHealth), With<Player>>,
    mana_query: Query<(&Mana, &MaxMana), With<Player>>,
    //target: Query<Entity, (With<ActiveUi>, With<UiStatus>)>,
    //asset_server: Res<AssetServer>,
) {
    trace!("Creating status bar UiNode");
    debug!("Creating health UiNode");
    debug!("Creating mana UiNode");
    let health_ui_icons: [Handle<Image>; 3] = [
        asset_server.load("HP/Style_1.png"),
        asset_server.load("HP/Style_2.png"),
        asset_server.load("HP/Style_3.png"),
    ];
    if let Ok((mana, max_mana)) = mana_query.get_single() {
        if let Ok((health, max_health)) = health_query.get_single() {
            // Root node
            commands.ui_builder(UiRoot)
                .container(NodeBundle {
                    background_color: BackgroundColor::from(GREEN),
                    visibility: Visibility::Visible,
                    style: Style {
                        width: Val::Percent(100.),
                        height: Val::Percent(10.),
                        ..default()
                    },
                    ..default()
                },
                // Status bar
                |root| {
                    root.row(|status_bar| {
                        status_bar.container(
                            TextBundle {
                                text: Text::from_section("Player Health",TextStyle {font: asset_server.load("FiraSans-Bold.ttf"),font_size: 50.0,color: Color::WHITE,}),
                                style: Style { width: Val::Percent(30.),height: Val::Percent(100.),..default()}
                                ,z_index: ZIndex::Global(10),..default()
                            },
                            |health_bar| {
                                health_bar.spawn(
                                    ImageBundle {
                                        image: health_ui_icons[0].clone().into(),
                                        visibility: Visibility::Visible,
                                        style: Style {width: Val::Percent((health.0 as f32 / max_health.0 as f32) * 100.),
                                        height: Val::Percent(100.),..default()},
                                        z_index: ZIndex::Global(9),..default()
                                });
                            }
                        );
                        status_bar.container(
                            TextBundle {
                                text: Text::from_section("Player Mana",TextStyle {font: asset_server.load("FiraSans-Bold.ttf"),font_size: 50.0,color: Color::WHITE,}),
                                style: Style { width: Val::Percent(30.),height: Val::Percent(100.),..default()}
                                ,z_index: ZIndex::Global(10),..default()
                            },
                            |mana_bar| {
                                mana_bar.spawn(
                                    NodeBundle {
                                    visibility: Visibility::Visible,
                                    background_color: BackgroundColor::from(BLUE),
                                    style: Style {
                                        width: Val::Percent((mana.0 as f32 / max_mana.0 as f32) * 100.),
                                        height: Val::Percent(100.),
                                        ..default()
                                    },
                                    z_index: ZIndex::Global(9),
                                    ..default()
                                    }
                                );
                            }
                        );
                    });
                })
            .insert(UiStatus);
            }
        }
}
