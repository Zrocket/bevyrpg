use bevy::color::palettes::css::CRIMSON;

use super::*;

pub fn draw_menu_ui(
    mut commands: Commands,
    active_menu_query: Query<Entity, With<ActiveMenuUi>>,
    asset_server: Res<AssetServer>,
) {
    trace!("draw_menu_ui");
    for _active_menu in active_menu_query.iter() {
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

        commands.entity(menu_parent)
            .add_child(item_settings)
            .add_child(item_save)
            .add_child(item_load)
            .add_child(item_quit);
    }
}

pub fn _draw_settings_ui(
    mut commands: Commands,
    target: Query<Entity, With<ActiveMenuUi>>,
    asset_server: Res<AssetServer>,
) {
    trace!("draw_settings_ui");
    for _target_entity in target.iter() {
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

        commands.entity(menu_parent)
            .add_child(item_video)
            .add_child(item_controller)
            .add_child(item_sound)
            .add_child(item_gameplay);
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

        commands.entity(menu_parent)
            .add_child(item_mouse_sensetivity)
            .add_child(item_key_bindings);
    }
}

pub fn _draw_sound_settings_ui(
    mut commands: Commands,
    target: Query<Entity, With<ActiveMenuUi>>,
    asset_server: Res<AssetServer>,
) {
    trace!("draw_sound_settings_ui");
    for _target_entity in target.iter() {
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

        commands.entity(menu_parent)
            .add_child(item_music_volume)
            .add_child(item_sound_volume);
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
    }
}
