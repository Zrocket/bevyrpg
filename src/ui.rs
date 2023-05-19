use bevy::{
    prelude::*,
    reflect::{self, erased_serde::Result},
};

use crate::*;

#[derive(Component)]
pub struct PlayerUi;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_ui.in_schedule(OnEnter(GameState::Gameplay)))
            .add_system(update_ui.in_set(OnUpdate(GameState::Gameplay)));
    }
}

fn update_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_ui: Query<Entity, With<PlayerUi>>,
    player: Query<(Entity, &Character), With<Player>>,
) {
    if let Ok(player_entity) = player.get_single() {
        if let Ok(player_ui) = player_ui.get_single() {
            commands.entity(player_ui).despawn_recursive();
            create_ui(&mut commands, &asset_server, &player_entity.1);
        }
    }
}

fn spawn_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player: Query<(Entity, &Character), With<Player>>,
) {
    if let Ok(player) = player.get_single() {
        create_ui(&mut commands, &asset_server, &player.1);
    }
}

fn create_ui(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    player: &Character,
) -> Entity {
    let ui_icons: [Handle<Image>; 3] = [
        asset_server.load("HP/Style_1.png"),
        asset_server.load("HP/Style_2.png"),
        asset_server.load("HP/Style_3.png"),
    ];

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Auto),
                    flex_direction: FlexDirection::ColumnReverse,
                    ..default()
                },
                ..default()
            },
            PlayerUi,
        ))
        .with_children(|commands| {
            commands
                .spawn(NodeBundle {
                    background_color: BackgroundColor::from(Color::GREEN),
                    visibility: Visibility::Visible,
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(10.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|commands| {
                    commands
                        .spawn(NodeBundle {
                            visibility: Visibility::Visible,
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                                ..default()
                            },
                            ..default()
                        })
                        .insert(TextBundle {
                            text: Text::from_section(
                                "Player Health",
                                TextStyle {
                                    font: asset_server.load("FiraSans-Bold.ttf"),
                                    font_size: 50.0,
                                    color: Color::BLACK,
                                },
                            ),
                            style: Style { ..default() },
                            ..default()
                        });
                    commands.spawn(ImageBundle {
                        image: ui_icons[0].clone().into(),
                        visibility: Visibility::Visible,
                        style: Style {
                            size: Size::new(
                                Val::Percent(
                                    ((player.health as f32 / player.max_health as f32) * 100.0)
                                        .floor(),
                                ),
                                Val::Auto,
                            ),
                            ..default()
                        },
                        ..default()
                    });
                });
        })
        .id()
}
