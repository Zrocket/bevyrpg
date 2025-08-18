use bevy::{color::palettes::css::CRIMSON, window::SystemCursorIcon};
use jonmo::prelude::*;

use super::*;

pub struct MenuUiPlugin;
impl Plugin for MenuUiPlugin {
    fn build(&self, app: &mut App) {
        app;
    }
}

pub fn draw_menu_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    trace!("draw_menu_ui");
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
                display:  Display::None,
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

pub fn jonmo_draw_menu_ui(
    world: &mut World,
) {
    trace!("draw_menu_ui");
    let asset_server = world.resource::<AssetServer>();
    let menu_parent = JonmoBuilder::from((
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(80.),
            height: Val::Percent(80.),
            left: Val::Percent(10.),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_self: AlignSelf::Center,
            flex_wrap: FlexWrap::Wrap,
            display:  Display::None,
            ..default()
        },
        BackgroundColor(CRIMSON.into()),
        UiMenu,
        UiIndex(0),
    ));

    let item_settings = JonmoBuilder::from((
        Node { ..default() },
        Text("Settings".to_string()),
        TextColor(Color::WHITE),
        TextFont {
            font: asset_server.load("FiraSans-Bold.ttf"),
            font_size: 50.0,
            ..default()
        }
    ));

    let item_save = JonmoBuilder::from((
        Node { ..default() },
        Text("Save".to_string()),
        TextColor(Color::WHITE),
        TextFont {
            font: asset_server.load("FiraSans-Bold.ttf"),
            font_size: 50.0,
            ..default()
        }
    ));

    let item_load = JonmoBuilder::from((
        Node { ..default() },
        Text("Load".to_string()),
        TextColor(Color::WHITE),
        TextFont {
            font: asset_server.load("FiraSans-Bold.ttf"),
            font_size: 50.0,
            ..default()
        }
    ));

    let item_quit = JonmoBuilder::from((
        Node { ..default() },
        Text("Quit".to_string()),
        TextColor(Color::WHITE),
        TextFont {
            font: asset_server.load("FiraSans-Bold.ttf"),
            font_size: 50.0,
            ..default()
        }
    ));

    menu_parent.child(item_settings)
        .child(item_save)
        .child(item_load)
        .child(item_quit)
        .spawn(world);
}

fn jonmo_menu_button(
    color: Color,
    label: &'static str,
) -> JonmoBuilder {
    JonmoBuilder::from((
        Node {
            ..default()
        },
        BorderRadius::MAX,
        BackgroundColor(color),
    ))
    .with_entity(move |mut entity| {
        entity.observe(
            move |trigger: Trigger<Pointer<Click>>| {
            }
        );
    })
}

pub fn _draw_settings_ui(
    mut commands: Commands,
    target: Query<Entity, With<ActiveUi>>,
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

pub fn jonmo_draw_settings_ui(
    world: &mut World,
    target: Query<Entity, With<ActiveUi>>,
) {
    trace!("draw_settings_ui");
    for _target_entity in target.iter() {
        let asset_server = world.resource::<AssetServer>();
        let menu_parent = JonmoBuilder::from((
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
        ));
        let item_video = JonmoBuilder::from((
                Node { ..default() },
                Button,
                Text("Video Settings".to_string()),
                TextColor(Color::WHITE),
                TextFont {
                    font: asset_server.load("FiraSans.Bold.ttf"),
                    font_size: 50.0,
                    ..default()
                },
        ));
        let item_controller = JonmoBuilder::from((
                Node { ..default() },
                Button,
                Text("Controller Settings".to_string()),
                TextColor(Color::WHITE),
                TextFont {
                    font: asset_server.load("FiraSans.Bold.ttf"),
                    font_size: 50.0,
                    ..default()
                },
        ));
        let item_sound = JonmoBuilder::from((
                Node { ..default() },
                Button,
                Text("Sound Settings".to_string()),
                TextColor(Color::WHITE),
                TextFont {
                    font: asset_server.load("FiraSans.Bold.ttf"),
                    font_size: 50.0,
                    ..default()
                },
        ));
        let item_gameplay = JonmoBuilder::from((
                Node { ..default() },
                Button,
                Text("Gameplay Settings".to_string()),
                TextColor(Color::WHITE),
                TextFont {
                    font: asset_server.load("FiraSans.Bold.ttf"),
                    font_size: 50.0,
                    ..default()
                },
        ));

        menu_parent.child(item_video)
            .child(item_controller)
            .child(item_sound)
            .child(item_gameplay)
            .spawn(world);
    }
}

pub fn _draw_controller_settings_ui(
    mut commands: Commands,
    target: Query<Entity, With<ActiveUi>>,
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

pub fn jonmo_draw_controller_settings_ui(
    world: &mut World,
    target: Query<Entity, With<ActiveUi>>,
) {
    trace!("draw_controller_settings_ui");
    for _target_entity in target.iter() {
        let asset_server = world.resource::<AssetServer>();
        info!("Drawing MenuUi");

        let menu_parent = JonmoBuilder::from((
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
        ));
        let item_mouse_sensetivity = JonmoBuilder::from((
                Node { ..default() },
                Button,
                Text("Mouse Sensetivity".to_string()),
                TextColor(Color::WHITE),
                TextFont {
                    font: asset_server.load("FiraSans.Bold.ttf"),
                    font_size: 50.0,
                    ..default()
                },
        ));
        let item_key_bindings = JonmoBuilder::from((
                Node { ..default() },
                Button,
                Text("Key Bindings".to_string()),
                TextColor(Color::WHITE),
                TextFont {
                    font: asset_server.load("FiraSans.Bold.ttf"),
                    font_size: 50.0,
                    ..default()
                },
        ));

        menu_parent.child(item_mouse_sensetivity)
            .child(item_key_bindings)
            .spawn(world);
    }
}

pub fn _draw_sound_settings_ui(
    mut commands: Commands,
    target: Query<Entity, With<ActiveUi>>,
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

pub fn jonmo_draw_sound_settings_ui(
    world: &mut World,
    target: Query<Entity, With<ActiveUi>>,
) {
    trace!("draw_sound_settings_ui");
    for _target_entity in target.iter() {
        let asset_server = world.resource::<AssetServer>();
        let menu_parent = JonmoBuilder::from((
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
        ));
        let item_music_volume = JonmoBuilder::from((
                Node { ..default() },
                Button,
                Text("Music Volume".to_string()),
                TextColor(Color::WHITE),
                TextFont {
                    font: asset_server.load("FiraSans.Bold.ttf"),
                    font_size: 50.0,
                    ..default()
                },
        ));
        let item_sound_volume = JonmoBuilder::from((
                Node { ..default() },
                Button,
                Text("Sound Volume".to_string()),
                TextColor(Color::WHITE),
                TextFont {
                    font: asset_server.load("FiraSans.Bold.ttf"),
                    font_size: 50.0,
                    ..default()
                },
        ));

        menu_parent.child(item_music_volume)
            .child(item_sound_volume)
            .spawn(world);
    }
}

pub fn _draw_video_settings_ui(
    mut commands: Commands,
    target: Query<Entity, With<ActiveUi>>,
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

pub fn jonmo_draw_video_settings_ui(
    world: &mut World,
    target: Query<Entity, With<ActiveUi>>,
    _projection: Query<&Projection>,
) {
    trace!("draw_video_settings_ui");
    for _target_entity in target.iter() {
        let asset_server = world.resource::<AssetServer>();
        let _menu_parent = JonmoBuilder::from((
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
        .spawn(world);
    }
}
