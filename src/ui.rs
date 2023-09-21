use bevy::{
    prelude::*,
    reflect::{self, erased_serde::Result}, ui::widget::UiImageSize,
};
use bevy_simple_text_input::{TextInput, TextInputPlugin, TextInputSubmitEvent};

use crate::*;

#[derive(Component)]
pub struct PlayerUi {
    pub status_bar: bool,
    pub inventory: bool,
    pub console: bool,
}

#[derive(Component)]
pub struct StatusBarUi;

#[derive(Component)]
pub struct UiEntity;

#[derive(Component)]
pub struct PlayerInventory;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Gameplay), spawn_ui)
            .add_systems(
                Update,
                update_ui.run_if(in_state(GameState::Gameplay))
            );
    }
}

fn update_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    statusbar_ui: Query<Entity, With<StatusBarUi>>,
    player_ui: Query<&PlayerUi, With<PlayerUi>>,
    player: Query<(Entity, &Character), With<Player>>,
    ui_entities: Query<Entity, With<UiEntity>>,
) {
    for ui_entity in  ui_entities.iter() {
        commands.entity(ui_entity).despawn_recursive();
    }

    if let Ok(player_entity) = player.get_single() {
        if let Ok(player_ui) = player_ui.get_single() {
            if player_ui.status_bar {
                if let Ok(statusbar_ui) = statusbar_ui.get_single() {
                    create_ui(&mut commands, &asset_server, &player_entity.1);
                }
            }
            if player_ui.inventory {
                create_inventory_ui(&mut commands, &asset_server, &player_entity.1);
            }
            if player_ui.console {
                create_console_ui(&mut commands, &asset_server);
            }
        }
    }
}

fn spawn_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player: Query<(Entity, &Character), With<Player>>,
) {
    commands.spawn(
       PlayerUi {
            status_bar: true,
            inventory: false,
            console: false,
       });
    if let Ok(player) = player.get_single() {
        create_ui(&mut commands, &asset_server, &player.1);
    }
    create_crosshair(&mut commands, &asset_server);
}

fn create_crosshair(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    ) -> Entity {
    let crosshair: Handle<Image> = asset_server.load("new_crosshairs/dot.png");
    commands
        .spawn(ImageBundle {
            image: crosshair.clone().into(),
            style: Style {
                position_type: PositionType::Absolute,
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                align_self: AlignSelf::Center,
                justify_self: JustifySelf::Center,
                left: Val::Vw(45.0),
                ..default()
            },
            ..default()
        })
    .id()
}

fn create_console_ui(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) -> Entity {
    commands.spawn(
            NodeBundle {
                background_color: BackgroundColor::from(Color::BLACK),
                style: Style {
                    position_type: PositionType::Absolute,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Start,
                    width: Val::Percent(100.),
                    height: Val::Percent(50.),
                    ..default()
                },
                ..default()
            })
    .with_children(|parent| {
        parent.spawn(
            NodeBundle {
                ..default()
            }
                    );
    })
    .insert(UiEntity)
    .id()
}

fn create_inventory_ui(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    player: &Character,
) -> Entity {
    commands
        .spawn(
                NodeBundle {
                    background_color: BackgroundColor::from(Color::BLACK),
                    style: Style {
                        position_type: PositionType::Absolute,
                        width: Val::Percent(80.),
                        height: Val::Percent(80.),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Start,
                        align_self: AlignSelf::Center,
                        ..default()
                    },
                    ..default()
                }
        )
        .insert(UiEntity)
        .id()
}

fn create_ui(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    player: &Character,
) -> Entity {
    commands
        .spawn(
            NodeBundle {
            visibility: Visibility::Visible,
            style: Style {
                flex_direction: FlexDirection::ColumnReverse,
                width: Val::Percent(100.),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(
                NodeBundle {
                background_color: BackgroundColor::from(Color::GREEN),
                visibility: Visibility::Visible,
                style: Style {
                    height: Val::Percent(10.),
                    width: Val::Percent(100.),
                    ..default()
                },
                ..default()
                }
            )
            .with_children(|parent| {
                create_health_ui(asset_server, player, parent);
            })
            .with_children(|parent| {
                create_mana_ui(asset_server, player, parent);
            });
        })
        .insert(UiEntity)
        .insert(StatusBarUi)
        .id()
}

fn create_health_ui(
    asset_server: &Res<AssetServer>,
    player: &Character,
    parent: &mut ChildBuilder,
) {
    let ui_icons: [Handle<Image>; 3] = [
        asset_server.load("HP/Style_1.png"),
        asset_server.load("HP/Style_2.png"),
        asset_server.load("HP/Style_3.png"),
    ];
        parent.spawn(
            TextBundle {
                text: Text::from_section(
                  "Player Health",
                  TextStyle {
                      font: asset_server.load("FiraSans-Bold.ttf"),
                      font_size: 50.0,
                      color: Color::WHITE,
                  }
                ),
                style: Style { 
                    width: Val::Percent(30.),
                    height: Val::Percent(100.),
                    ..default()
                },
                z_index: ZIndex::Global(10),
                ..default()
            })
            .with_children(|parent| {
                parent.spawn(
                        ImageBundle {
                            image: ui_icons[0].clone().into(),
                            visibility: Visibility::Visible,
                            style: Style {
                                width: Val::Percent((player.health as f32 / player.max_health as f32) * 100.),
                                height: Val::Percent(100.),
                                ..default()
                            },
                            z_index: ZIndex::Global(9),
                            ..default()
                        }
                    );
            });
}

fn create_mana_ui(
    asset_server: &Res<AssetServer>,
    player: &Character,
    parent: &mut ChildBuilder,
) {

    parent.spawn(
        TextBundle {
            text: Text::from_section(
              "Player Mana",
              TextStyle {
                  font: asset_server.load("FiraSans-Bold.ttf"),
                  font_size: 50.0,
                  color: Color::WHITE,
              }
            ),
            style: Style { 
                width: Val::Percent(30.),
                height: Val::Percent(100.),
                ..default()
            },
            z_index: ZIndex::Global(10),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(
                NodeBundle {
                visibility: Visibility::Visible,
                background_color: BackgroundColor::from(Color::BLUE),
                style: Style {
                    width: Val::Percent((player.mana as f32 / player.max_mana as f32) * 100.),
                    height: Val::Percent(100.),
                    ..default()
                },
                z_index: ZIndex::Global(9),
                ..default()
                }
            );
        });

}
