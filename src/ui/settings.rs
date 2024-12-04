use bevy::color::palettes::css::CRIMSON;
use sickle_ui::{ui_builder::{UiBuilderExt, UiRoot}, widgets::layout::{column::UiColumnExt, container::UiContainerExt}};

use super::*;

pub fn draw_menu_ui(
    mut commands: Commands,
    target: Query<Entity, With<ActiveMenuUi>>,
    asset_server: Res<AssetServer>,
    ) {
    for _target_entity in target.iter() {
        info!("Drawing MenuUi");
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
        |settings_menu| {
                info!("Drawing MenuUi Items");
                settings_menu.column(|column| {
                    column.container(ButtonBundle {
                        ..default()
                    },
                    |parent| {
                        parent.spawn(
                            TextBundle {
                            text: Text::from_section("Settings", TextStyle {
                                font: asset_server.load("FiraSans-Bold.ttf"),
                                font_size: 50.0,
                                color: Color::WHITE,
                            }),
                            ..default()
                        });
                    });

                    column.container(ButtonBundle {
                        ..default()
                    },
                    |parent| {
                        parent.spawn(TextBundle {
                            text: Text::from_section("Save", TextStyle {
                                font: asset_server.load("FiraSans-Bold.ttf"),
                                font_size: 50.0,
                                color: Color::WHITE, 
                            }),
                            ..default()
                        });
                    });

                    column.container(ButtonBundle {
                        ..default()
                    },
                    |parent| {
                        parent.spawn(TextBundle {
                            text: Text::from_section("Load", TextStyle {
                                font: asset_server.load("FiraSans-Bold.ttf"),
                                font_size: 50.0,
                                color: Color::WHITE, 
                            }),
                            ..default()
                        });
                    });

                    column.container(ButtonBundle {
                        ..default()
                    },
                    |parent| {
                        parent.spawn(TextBundle {
                            text: Text::from_section("Quit", TextStyle {
                                font: asset_server.load("FiraSans-Bold.ttf"),
                                font_size: 50.0,
                                color: Color::WHITE, 
                            }),
                            ..default()
                        });
                    });
            });
        })
        .insert(UiMenu)
        .insert(UiIndex(0));
    }
}

pub fn draw_settings_ui(
    mut commands: Commands,
    target: Query<Entity, With<ActiveMenuUi>>,
    asset_server: Res<AssetServer>,
    ) {
    for _target_entity in target.iter() {
        info!("Drawing MenuUi");
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
        |settings_menu| {
                info!("Drawing MenuUi Items");
                settings_menu.column(|column| {

                    column.container(ButtonBundle {
                        ..default()
                    },
                    |parent| {
                        parent.spawn(TextBundle {
                            text: Text::from_section("Video Settings", TextStyle {
                                font: asset_server.load("FiraSans-Bold.ttf"),
                                font_size: 50.0,
                                color: Color::WHITE, 
                            }),
                            ..default()
                        });
                    });

                    column.container(ButtonBundle {
                        ..default()
                    },
                    |parent| {
                        parent.spawn(TextBundle {
                            text: Text::from_section("Controller Settings", TextStyle {
                                font: asset_server.load("FiraSans-Bold.ttf"),
                                font_size: 50.0,
                                color: Color::WHITE, 
                            }),
                            ..default()
                        });
                    });

                    column.container(ButtonBundle {
                        ..default()
                    },
                    |parent| {
                        parent.spawn(TextBundle {
                            text: Text::from_section("Sound Settings", TextStyle {
                                font: asset_server.load("FiraSans-Bold.ttf"),
                                font_size: 50.0,
                                color: Color::WHITE, 
                            }),
                            ..default()
                        });
                    });

                    column.container(ButtonBundle {
                        ..default()
                    },
                    |parent| {
                        parent.spawn(TextBundle {
                            text: Text::from_section("Gameplay Settings", TextStyle {
                                font: asset_server.load("FiraSans-Bold.ttf"),
                                font_size: 50.0,
                                color: Color::WHITE, 
                            }),
                            ..default()
                        });
                    });

            });
        })
        .insert(UiMenu)
        .insert(UiIndex(0));
    }
}

pub fn draw_controller_settings_ui(
    mut commands: Commands,
    target: Query<Entity, With<ActiveMenuUi>>,
    asset_server: Res<AssetServer>,
    ) {
    for _target_entity in target.iter() {
        info!("Drawing MenuUi");
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
        |settings_menu| {
                info!("Drawing MenuUi Items");
                settings_menu.column(|column| {

                    column.spawn(TextBundle {
                        text: Text::from_section("Mouse Sensetivity", TextStyle {
                            font: asset_server.load("FiraSans-Bold.ttf"),
                            font_size: 50.0,
                            color: Color::WHITE, 
                        }),
                        ..default()
                    });

                    column.spawn(TextBundle {
                        text: Text::from_section("Key Bindings", TextStyle {
                            font: asset_server.load("FiraSans-Bold.ttf"),
                            font_size: 50.0,
                            color: Color::WHITE, 
                        }),
                        ..default()
                    });
            });
        })
        .insert(UiMenu)
        .insert(UiIndex(0));
    }
}

pub fn draw_sound_settings_ui(
    mut commands: Commands,
    target: Query<Entity, With<ActiveMenuUi>>,
    asset_server: Res<AssetServer>,
    ) {
    for _target_entity in target.iter() {
        info!("Drawing MenuUi");
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
        |settings_menu| {
                info!("Drawing MenuUi Items");
                settings_menu.column(|column| {


                    column.spawn(TextBundle {
                        text: Text::from_section("Music Volume", TextStyle {
                            font: asset_server.load("FiraSans-Bold.ttf"),
                            font_size: 50.0,
                            color: Color::WHITE, 
                        }),
                        ..default()
                    });


                    column.spawn(TextBundle {
                        text: Text::from_section("Sound Volume", TextStyle {
                            font: asset_server.load("FiraSans-Bold.ttf"),
                            font_size: 50.0,
                            color: Color::WHITE, 
                        }),
                        ..default()
                    });
            });
        })
        .insert(UiMenu)
        .insert(UiIndex(0));
    }
}

pub fn draw_video_settings_ui(
    mut commands: Commands,
    target: Query<Entity, With<ActiveMenuUi>>,
    projection: Query<&Projection>,
    asset_server: Res<AssetServer>,
    ) {
    for _target_entity in target.iter() {
        info!("Drawing MenuUi");
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
        |settings_menu| {
                info!("Drawing MenuUi Items");
                settings_menu.column(|column| {
                for projection in projection.iter() {
                    match projection {
                        Projection::Perspective(perspective) => {
                            info!("PERSPECTIVE PROJECTION");


                            column.spawn(TextBundle {
                                text: Text::from_section(format!("FOV: {}", perspective.fov), TextStyle { 
                                    font: asset_server.load("FiraSans-Bold.ttf"),
                                    font_size: 50.0,
                                    color: Color::WHITE,
                                }),
                                ..default()
                            });


                            column.spawn(TextBundle {
                                text: Text::from_section(format!("Aspect Ratio: {}", perspective.aspect_ratio), TextStyle { 
                                    font: asset_server.load("FiraSans-Bold.ttf"),
                                    font_size: 50.0,
                                    color: Color::WHITE,
                                }),
                                ..default()
                            });


                            column.spawn(TextBundle {
                                text: Text::from_section(format!("Perspective Near: {}", perspective.near), TextStyle { 
                                    font: asset_server.load("FiraSans-Bold.ttf"),
                                    font_size: 50.0,
                                    color: Color::WHITE,
                                }),
                                ..default()
                            });


                            column.spawn(TextBundle {
                                text: Text::from_section(format!("Perspective Far: {}", perspective.far), TextStyle { 
                                    font: asset_server.load("FiraSans-Bold.ttf"),
                                    font_size: 50.0,
                                    color: Color::WHITE,
                                }),
                                ..default()
                            });
                        },
                        Projection::Orthographic(perspective) => {
                            info!("Orthographic PROJECTION");


                            column.spawn(TextBundle {
                                text: Text::from_section(format!("Perspective Near: {}", perspective.near), TextStyle { 
                                    font: asset_server.load("FiraSans-Bold.ttf"),
                                    font_size: 50.0,
                                    color: Color::WHITE,
                                }),
                                ..default()
                            });


                            column.spawn(TextBundle {
                                text: Text::from_section(format!("Perspective Far: {}", perspective.far), TextStyle { 
                                    font: asset_server.load("FiraSans-Bold.ttf"),
                                    font_size: 50.0,
                                    color: Color::WHITE,
                                }),
                                ..default()
                            });
                        },
                    }
                }
            });

        })
        .insert(UiMenu)
        .insert(UiIndex(0));
    }
}
