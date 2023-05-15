use bevy::{prelude::*, reflect};

use crate::*;

fn create_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Auto),
            flex_direction: FlexDirection::ColumnReverse,
            ..default()
        },
        ..default()
    })
    .with_children(|commands| {
        commands.spawn(NodeBundle {
        background_color: BackgroundColor::from(Color::GREEN),
        visibility: Visibility::Visible,
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Auto),
            ..default()
        },
            ..default()
        })
        .with_children(|commands| {
            commands.spawn(NodeBundle {
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
                          color: Color::BLACK 
                      }
                ),
                style: Style {
                    ..default()
                },
                ..default()
            });
            commands.spawn(NodeBundle {
                background_color: BackgroundColor::from(Color::RED),
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
