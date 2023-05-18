use bevy::{prelude::*, reflect};

use crate::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(create_ui.in_schedule(OnEnter(GameState::Gameplay)));
    }
}

fn create_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let ui_icons: [Handle<Image>; 3] = [
        asset_server.load("HP/Style_1.png"),
        asset_server.load("HP/Style_2.png"),
        asset_server.load("HP/Style_3.png"),
    ];

    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Auto),
                flex_direction: FlexDirection::ColumnReverse,
                ..default()
            },
            ..default()
        })
        .with_children(|commands| {
            commands
                .spawn(NodeBundle {
                    background_color: BackgroundColor::from(Color::GREEN),
                    visibility: Visibility::Visible,
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Auto),
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
                            size: Size::new(Val::Percent(100.0), Val::Auto),
                            ..default()
                        },
                        ..default()
                    });
                });
        });
}
