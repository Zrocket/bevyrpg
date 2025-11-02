use bevy::color::palettes::css::{CORNSILK, CRIMSON};

use super::*;

#[derive(Component, Reflect)]
pub struct UiMenu;

#[derive(Component, Reflect)]
pub struct UiSettings;

#[derive(Component, Reflect)]
pub struct UiVideoSettings;

#[derive(Component, Reflect)]
pub struct UiControllerSettings;

#[derive(Component, Reflect)]
pub struct UiSoundSettings;

#[derive(Component, Reflect)]
pub struct UiGameplaySettings;

pub struct MenuUiPlugin;
impl Plugin for MenuUiPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<UiMenu>();
            //.add_systems(OnEnter(GameState::Gameplay), (
                //jonmo_draw_menu_ui,
                //jonmo_draw_settings_ui,
                //jonmo_draw_sound_settings_ui,
                //jonmo_draw_video_settings_ui,
                //jonmo_draw_controller_settings_ui,
            //));
    }
}

/*pub fn jonmo_draw_menu_ui(
    world: &mut World,
) {
    trace!("draw_menu_ui");

    let settings_hover = LazyEntity::new();
    let save_hover = LazyEntity::new();
    let load_hover = LazyEntity::new();
    let quit_hover = LazyEntity::new();

    let asset_server = world.resource::<AssetServer>();
    JonmoBuilder::from((
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
        Name::new("PauseMenu"),
        //UiIndex(0),
    ))
    .child(jonmo_sub_menu_item(settings_hover.clone(), asset_server, "Settings", PauseMenuState::Settings))
    .child(jonmo_menu_item(save_hover.clone(), asset_server, "Save"))
    .child(jonmo_menu_item(load_hover.clone(), asset_server, "Load"))
    .child(jonmo_menu_item(quit_hover.clone(), asset_server, "Quit"))
    .spawn(world);
}*/

/*fn jonmo_menu_item(
    holder: LazyEntity,
    asset_server: &AssetServer,
    label: &'static str,
) -> JonmoBuilder {
    let hover = LazyEntity::new();
    let hover_observer_1 = hover.clone();
    let hover_observer_2 = hover.clone();
    JonmoBuilder::from((
        Node { ..default() },
        Text(label.to_string()),
        TextColor(Color::WHITE),
        TextFont {
            font: asset_server.load("FiraSans-Bold.ttf"),
            font_size: 50.0,
            ..default()
        }
    ))
    .insert(Hoverable(false))
    .entity_sync(hover.clone())
    .with_entity(move |mut entity| {
        entity.observe(move |_trigger: Trigger<Pointer<Over>>, mut hover_query: Query<&mut Hoverable>| {
            if let Ok(mut hover) = hover_query.get_mut(hover_observer_1.get()) {
                hover.0 = true;
            }
        });
        entity.observe(move |_trigger: Trigger<Pointer<Out>>, mut hover_query: Query<&mut Hoverable>| {
            if let Ok(mut hover) = hover_query.get_mut(hover_observer_2.get()) {
                hover.0 = false;
            }
        });
    })
    .component_signal(SignalBuilder::from_component_lazy(hover.clone())
        .map_in(|hover: Hoverable| hover.0)
        .dedupe()
        .map_in(move |hover: bool| {
            if hover {
                BackgroundColor(CORNSILK.into())
            } else {
                BackgroundColor(CRIMSON.into())
            }
        })
        .map_in(Some))
}*/

/*fn jonmo_sub_menu_item(
    holder: LazyEntity,
    asset_server: &AssetServer,
    label: &'static str,
    new_state: PauseMenuState,
) -> JonmoBuilder {
    let holder = LazyEntity::new();
    let hover_observer_1 = holder.clone();
    let hover_observer_2 = holder.clone();
    JonmoBuilder::from((
        Node { ..default() },
        Text(label.to_string()),
        TextColor(Color::WHITE),
        TextFont {
            font: asset_server.load("FiraSans-Bold.ttf"),
            font_size: 50.0,
            ..default()
        }
    ))
    .insert(Hoverable(false))
    .entity_sync(holder.clone())
    .with_entity(move |mut entity| {
        entity.observe(move |_trigger: Trigger<Pointer<Over>>, mut hover_query: Query<&mut Hoverable>| {
            if let Ok(mut hover) = hover_query.get_mut(hover_observer_1.get()) {
                hover.0 = true;
            }
        });
        entity.observe(move |_trigger: Trigger<Pointer<Out>>, mut hover_query: Query<&mut Hoverable>| {
            if let Ok(mut hover) = hover_query.get_mut(hover_observer_2.get()) {
                hover.0 = false;
            }
        });
        entity.observe(move |_trigger: Trigger<Pointer<Click>>, mut pause_state: ResMut<NextState<PauseMenuState>>| {
            pause_state.set(new_state.clone());
        });
    })
    .component_signal(SignalBuilder::from_component_lazy(holder.clone())
        .map_in(|hover: Hoverable| hover.0)
        .dedupe()
        .map_in(move |hover: bool| {
            if hover {
                BackgroundColor(CORNSILK.into())
            } else {
                BackgroundColor(CRIMSON.into())
            }
        })
        .map_in(Some))

}*/

/*fn jonmo_menu_button(
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
            move |_trigger: Trigger<Pointer<Click>>| {
            }
        );
    })
}*/

/*pub fn jonmo_draw_settings_ui(
    world: &mut World,
) {
    trace!("draw_settings_ui");
    let asset_server = world.resource::<AssetServer>();

    let video_hover = LazyEntity::new();
    let controller_hover = LazyEntity::new();
    let sound_hover = LazyEntity::new();
    let gameplay_hover = LazyEntity::new();

    JonmoBuilder::from((
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(80.),
            height: Val::Percent(80.),
            left: Val::Percent(10.),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_self: AlignSelf::Center,
            flex_wrap: FlexWrap::Wrap,
            display: Display::None,
            ..default()
        },
        BackgroundColor(CRIMSON.into()),
        UiSettings,
        Name::new("SettingsMenu"),
    ))
    .child(jonmo_sub_menu_item(video_hover.clone(), asset_server, "Video Settings", PauseMenuState::VideoSettings))
    .child(jonmo_sub_menu_item(controller_hover.clone(), asset_server, "Controller Settings", PauseMenuState::ControllerSettings))
    .child(jonmo_sub_menu_item(sound_hover.clone(), asset_server, "Sound Settings", PauseMenuState::SoundSettings))
    .child(jonmo_sub_menu_item(gameplay_hover.clone(), asset_server, "Gameplay Settings", PauseMenuState::GameplaySettings))
    .spawn(world);
}*/

/*pub fn jonmo_draw_controller_settings_ui(
    world: &mut World,
) {
    trace!("draw_controller_settings_ui");
    let asset_server = world.resource::<AssetServer>();
    info!("Drawing MenuUi");

    let mouse_sensetivity_hover = LazyEntity::new();
    let key_bindings_hover = LazyEntity::new();

    JonmoBuilder::from((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(80.),
                height: Val::Percent(80.),
                left: Val::Percent(10.),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_self: AlignSelf::Center,
                flex_wrap: FlexWrap::Wrap,
                display: Display::None,
                ..default()
            },
            BackgroundColor(CRIMSON.into()),
            UiControllerSettings,
    ))
    .child(jonmo_menu_item(mouse_sensetivity_hover, &asset_server, "Mouse Sensetivity"))
    .child(jonmo_menu_item(key_bindings_hover, &asset_server, "Key Bindings"))
    .spawn(world);
}*/

/*pub fn jonmo_draw_sound_settings_ui(
    world: &mut World,
) {
    trace!("draw_sound_settings_ui");
    let asset_server = world.resource::<AssetServer>();

    let music_volume_hover = LazyEntity::new();
    let sound_volume_hover = LazyEntity::new();

    JonmoBuilder::from((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(80.),
                height: Val::Percent(80.),
                left: Val::Percent(10.),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_self: AlignSelf::Center,
                flex_wrap: FlexWrap::Wrap,
                display: Display::None,
                ..default()
            },
            BackgroundColor(CRIMSON.into()),
            UiSoundSettings,
            UiIndex(0),
    ))
    .child(jonmo_menu_item(music_volume_hover, &asset_server, "Music Volume"))
    .child(jonmo_menu_item(sound_volume_hover, &asset_server, "Sound Volume"))
    .spawn(world);
}*/

/*pub fn jonmo_draw_video_settings_ui(
    world: &mut World,
    _projection: Query<&Projection>,
) {
    trace!("draw_video_settings_ui");
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
            UiVideoSettings,
            UiIndex(0),
    ))
    .spawn(world);
}*/
