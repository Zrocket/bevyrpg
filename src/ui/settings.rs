use bevy::color::palettes::css::CRIMSON;

use super::*;

pub fn draw_menu_ui(
    mut commands: Commands,
    target: Query<Entity, With<ActiveMenuUi>>,
    asset_server: Res<AssetServer>,
) {
    trace!("draw_menu_ui");
    for _target_entity in target.iter() {
        info!("Drawing MenuUi");

        let menu_parent = commands
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
                UiMenu,
                UiIndex(0),
            ))
            .id();

        let item_settings = commands
            .spawn((
                Node { ..default() },
                Button,
                Text("Settings".to_string()),
                TextColor(Color::WHITE),
                TextFont {
                    font: asset_server.load("FiraSans-Bold.ttf"),
                    font_size: 50.0,
                    ..default()
                },
            ))
            .id();
        let item_save = commands
            .spawn((
                Node { ..default() },
                Button,
                Text("Save".to_string()),
                TextColor(Color::WHITE),
                TextFont {
                    font: asset_server.load("FiraSans-Bold.ttf"),
                    font_size: 50.0,
                    ..default()
                },
            ))
            .id();
        let item_load = commands
            .spawn((
                Node { ..default() },
                Button,
                Text("Load".to_string()),
                TextColor(Color::WHITE),
                TextFont {
                    font: asset_server.load("FiraSans-Bold.ttf"),
                    font_size: 50.0,
                    ..default()
                },
            ))
            .id();
        let item_quit = commands
            .spawn((
                Node { ..default() },
                Button,
                Text("Quit".to_string()),
                TextColor(Color::WHITE),
                TextFont {
                    font: asset_server.load("FiraSans-Bold.ttf"),
                    font_size: 50.0,
                    ..default()
                },
            ))
            .id();

        commands.queue(AddChild {
            parent: menu_parent,
            child: item_settings,
        });
        commands.queue(AddChild {
            parent: menu_parent,
            child: item_save,
        });
        commands.queue(AddChild {
            parent: menu_parent,
            child: item_load,
        });
        commands.queue(AddChild {
            parent: menu_parent,
            child: item_quit,
        });

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
        .insert(UiIndex(0));*/
    }
}

pub fn _draw_settings_ui(
    mut commands: Commands,
    target: Query<Entity, With<ActiveMenuUi>>,
    asset_server: Res<AssetServer>,
) {
    trace!("draw_settings_ui");
    for _target_entity in target.iter() {
        info!("Drawing MenuUi");

        let menu_parent = commands
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
                UiMenu,
                UiIndex(0),
            ))
            .id();

        let item_video = commands
            .spawn((
                Node { ..default() },
                Button,
                Text("Video Settings".to_string()),
                TextColor(Color::WHITE),
                TextFont {
                    font: asset_server.load("FiraSans.Bold.ttf"),
                    font_size: 50.0,
                    ..default()
                },
            ))
            .id();
        let item_controller = commands
            .spawn((
                Node { ..default() },
                Button,
                Text("Controller Settings".to_string()),
                TextColor(Color::WHITE),
                TextFont {
                    font: asset_server.load("FiraSans.Bold.ttf"),
                    font_size: 50.0,
                    ..default()
                },
            ))
            .id();
        let item_sound = commands
            .spawn((
                Node { ..default() },
                Button,
                Text("Sound Settings".to_string()),
                TextColor(Color::WHITE),
                TextFont {
                    font: asset_server.load("FiraSans.Bold.ttf"),
                    font_size: 50.0,
                    ..default()
                },
            ))
            .id();
        let item_gameplay = commands
            .spawn((
                Node { ..default() },
                Button,
                Text("Gameplay Settings".to_string()),
                TextColor(Color::WHITE),
                TextFont {
                    font: asset_server.load("FiraSans.Bold.ttf"),
                    font_size: 50.0,
                    ..default()
                },
            ))
            .id();

        commands.queue(AddChild {
            parent: menu_parent,
            child: item_video,
        });
        commands.queue(AddChild {
            parent: menu_parent,
            child: item_controller,
        });
        commands.queue(AddChild {
            parent: menu_parent,
            child: item_sound,
        });
        commands.queue(AddChild {
            parent: menu_parent,
            child: item_gameplay,
        });

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
        .insert(UiIndex(0));*/
    }
}

pub fn _draw_controller_settings_ui(
    mut commands: Commands,
    target: Query<Entity, With<ActiveMenuUi>>,
    asset_server: Res<AssetServer>,
) {
    trace!("draw_controller_settings_ui");
    for _target_entity in target.iter() {
        info!("Drawing MenuUi");

        let menu_parent = commands
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
                UiMenu,
                UiIndex(0),
            ))
            .id();

        let item_mouse_sensetivity = commands
            .spawn((
                Node { ..default() },
                Button,
                Text("Mouse Sensetivity".to_string()),
                TextColor(Color::WHITE),
                TextFont {
                    font: asset_server.load("FiraSans.Bold.ttf"),
                    font_size: 50.0,
                    ..default()
                },
            ))
            .id();
        let item_key_bindings = commands
            .spawn((
                Node { ..default() },
                Button,
                Text("Key Bindings".to_string()),
                TextColor(Color::WHITE),
                TextFont {
                    font: asset_server.load("FiraSans.Bold.ttf"),
                    font_size: 50.0,
                    ..default()
                },
            ))
            .id();

        commands.queue(AddChild {
            parent: menu_parent,
            child: item_mouse_sensetivity,
        });
        commands.queue(AddChild {
            parent: menu_parent,
            child: item_key_bindings,
        });

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
        .insert(UiIndex(0));*/
    }
}

pub fn _draw_sound_settings_ui(
    mut commands: Commands,
    target: Query<Entity, With<ActiveMenuUi>>,
    asset_server: Res<AssetServer>,
) {
    trace!("draw_sound_settings_ui");
    for _target_entity in target.iter() {
        info!("Drawing MenuUi");

        let menu_parent = commands
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
                UiMenu,
                UiIndex(0),
            ))
            .id();

        let item_music_volume = commands
            .spawn((
                Node { ..default() },
                Button,
                Text("Music Volume".to_string()),
                TextColor(Color::WHITE),
                TextFont {
                    font: asset_server.load("FiraSans.Bold.ttf"),
                    font_size: 50.0,
                    ..default()
                },
            ))
            .id();
        let item_sound_volume = commands
            .spawn((
                Node { ..default() },
                Button,
                Text("Sound Volume".to_string()),
                TextColor(Color::WHITE),
                TextFont {
                    font: asset_server.load("FiraSans.Bold.ttf"),
                    font_size: 50.0,
                    ..default()
                },
            ))
            .id();

        commands.queue(AddChild {
            parent: menu_parent,
            child: item_music_volume,
        });
        commands.queue(AddChild {
            parent: menu_parent,
            child: item_sound_volume,
        });

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
        .insert(UiIndex(0));*/
    }
}

pub fn _draw_video_settings_ui(
    mut commands: Commands,
    target: Query<Entity, With<ActiveMenuUi>>,
    _projection: Query<&Projection>,
    _asset_server: Res<AssetServer>,
) {
    trace!("draw_video_settings_ui");
    for _target_entity in target.iter() {
        info!("Drawing MenuUi");

        let _menu_parent = commands
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
                UiMenu,
                UiIndex(0),
            ))
            .id();

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
        .insert(UiIndex(0));*/
    }
}
