use bevy::{color::palettes::css::{BLUE, GREEN, RED}, ecs::{lifecycle::HookContext, world::DeferredWorld}, prelude::*};

use crate::{CctvDownEvent, CctvLeftEvent, CctvParent, CctvRightEvent, CctvUpEvent, furniture::computer::computer_input::{icon_out, icon_over}, widgets::floating_windows::floating_computer_rover_window_root};

#[derive(Component)]
#[require(
    Node {
        //width: px(300),
        //height: px(50),
        ..default()
    },
    Text("<".into()),
    BackgroundColor(GREEN.into()),
)]
#[component(on_add = on_cctv_left_button_add)]
pub struct CctvLeftButton;

fn on_cctv_left_button_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    world.commands()
        .entity(context.entity)
        .observe(icon_over)
        .observe(icon_out)
        .observe(cctv_left_pressed)
        .observe(cctv_left_released);
}

#[derive(Component)]
#[require(
    Node {
        //width: px(300),
        //height: px(50),
        ..default()
    },
    BackgroundColor(GREEN.into()),
    Text(">".into()),
)]
#[component(on_add = on_cctv_right_button_add)]
pub struct CctvRightButton;

fn on_cctv_right_button_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    world.commands()
        .entity(context.entity)
        .observe(icon_over)
        .observe(icon_out)
        .observe(cctv_right_pressed)
        .observe(cctv_right_released);
}

#[derive(Component)]
#[require (
    Node {
        ..default()
    },
    Text("^".into()),
    BackgroundColor(GREEN.into()),
)]
#[component(on_add = on_cctv_camera_up_button_add)]
pub struct CctvCameraUpButton;

fn on_cctv_camera_up_button_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    world.commands()
        .entity(context.entity)
        .observe(icon_over)
        .observe(icon_out)
        .observe(cctv_camera_up_pressed)
        .observe(cctv_camera_up_released);
}

#[derive(Component)]
#[require (
    Node {
        ..default()
    },
    Text("v".into()),
    BackgroundColor(GREEN.into()),
)]
#[component(on_add = on_cctv_camera_down_button_add)]
pub struct CctvCameraDownButton;

fn on_cctv_camera_down_button_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    world.commands()
        .entity(context.entity)
        .observe(icon_over)
        .observe(icon_out)
        .observe(cctv_camera_down_pressed)
        .observe(cctv_camera_down_released);
}

pub(crate) fn cctv_left_pressed(
    _trigger: On<Pointer<Press>>,
    mut commands: Commands,
    cctv_query: Query<Entity, With<CctvParent>>,
) {
    for entity in cctv_query.iter() {
        commands.entity(entity).trigger(|entity| CctvLeftEvent { entity });
    }
}
pub(crate) fn cctv_left_released(
    _trigger: On<Pointer<Release>>,
    mut commands: Commands,
    cctv_query: Query<Entity, With<CctvParent>>,
) {
    let Ok(entity) = cctv_query.single() else {
        error!("cctv_left_released: Failed to query Entity for CctvParent");
        return;
    };

    //if let Ok(entity) = cctv_query.single() {
    //for entity in cctv_query.iter() {
    commands.entity(entity).trigger(|entity| CctvLeftEvent { entity });
    //}
}

pub(crate) fn cctv_right_pressed(
    _trigger: On<Pointer<Press>>,
    mut commands: Commands,
    cctv_query: Query<Entity, With<CctvParent>>,
) {
    let Ok(entity) = cctv_query.single() else {
        error!("cctv_right_pressed: Failed to query Entity for CctvParent");
        return;
    };

    //if let Ok(entity) = cctv_query.single() {
    //for entity in cctv_query.iter() {
    commands.entity(entity).trigger(|entity| CctvRightEvent { entity });
    //}
}
pub(crate) fn cctv_right_released(
    _trigger: On<Pointer<Release>>,
    mut commands: Commands,
    cctv_query: Query<Entity, With<CctvParent>>,
) {
    let Ok(entity) = cctv_query.single() else {
        error!("cctv_right_released: Failed to query Entity for CctvParent");
        return;
    };

    //if let Ok(entity) = cctv_query.single() {
    //for entity in cctv_query.iter() {
    commands.entity(entity).trigger(|entity| CctvRightEvent { entity });
    //}
}

pub(crate) fn cctv_camera_up_pressed(
    _trigger: On<Pointer<Press>>,
    mut commands: Commands,
    cctv_query: Query<Entity, With<CctvParent>>,
) {
    let Ok(entity) = cctv_query.single() else {
        error!("cctv_camera_up_pressed: Failed to query Entity for CctvParent");
        return;
    };

    commands.entity(entity).trigger(|entity| CctvUpEvent { entity });
}
pub(crate) fn cctv_camera_up_released(
    _trigger: On<Pointer<Release>>,
    mut commands: Commands,
    cctv_query: Query<Entity, With<CctvParent>>,
) {
    let Ok(entity) = cctv_query.single() else {
        error!("cctv_camera_up_released: Failed to query Entity for CctvParent");
        return;
    };

    //if let Ok(entity) = cctv_query.single() {
    //for entity in cctv_query.iter() {
    commands.entity(entity).trigger(|entity| CctvUpEvent { entity });
    //}
}

pub(crate) fn cctv_camera_down_pressed(
    _trigger: On<Pointer<Press>>,
    mut commands: Commands,
    cctv_query: Query<Entity, With<CctvParent>>,
) {
    let Ok(entity) = cctv_query.single() else {
        error!("cctv_camera_down_pressed: Failed to query Entity for CctvParent");
        return;
    };

    commands.entity(entity).trigger(|entity| CctvDownEvent { entity });
}

pub(crate) fn cctv_camera_down_released(
    _trigger: On<Pointer<Release>>,
    mut commands: Commands,
    cctv_query: Query<Entity, With<CctvParent>>,
) {
    let Ok(entity) = cctv_query.single() else {
        error!("cctv_camera_down_released: Failed to query Entity for CctvParent");
        return;
    };

    //if let Ok(entity) = cctv_query.single() {
    //for entity in cctv_query.iter() {
    commands.entity(entity).trigger(|entity| CctvDownEvent { entity });
    //}
}
#[derive(Component)]
#[require(
    Node {
        width: px(50),
        //height: px(50),
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Stretch,
        ..default()
    },
    BackgroundColor(RED.into()),
)]
#[component(on_add = on_cctv_buttons_node_add)]
pub struct CctvButtonsNode;

fn on_cctv_buttons_node_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    let top = world.commands().spawn((
            Node {
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                flex_grow: 1.,
                ..default()
            },
            //Children::spawn(SpawnWith(|parent: &mut ChildSpawner| {
            //    parent.spawn(ForwardButton);
            //})),
    )).id();

    let middle = world.commands().spawn((
            Node {
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                flex_grow: 1.,
                ..default()
            },
            Children::spawn(SpawnWith(|parent: &mut ChildSpawner| {
                parent.spawn(CctvLeftButton);
                parent.spawn(CctvRightButton);
            })),
    )).id();

    let bottom = world.commands().spawn((
            Node {
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                flex_grow: 1.,
                ..default()
            },
            Children::spawn(SpawnWith(|parent: &mut ChildSpawner| {
                //parent.spawn(BackwardButton);
                parent.spawn(CctvCameraUpButton);
                parent.spawn(CctvCameraDownButton);
            })),
    )).id();

    world.commands()
        .entity(context.entity)
        .add_child(top)
        .add_child(middle)
        .add_child(bottom);
}

#[derive(Component)]
#[require(
    Node {
        position_type: PositionType::Absolute,
        width: Val::Percent(10.),
        height: Val::Percent(15.),
        align_items: AlignItems::Center,
        border_radius: BorderRadius::all(Val::Px(10.)),
        left: Val::Px(20.),
        top: Val::Px(300.),
        flex_direction: FlexDirection::Column,
        overflow: Overflow { x: OverflowAxis::Hidden, y: OverflowAxis::Hidden },
        ..default()
    },
    BackgroundColor(BLUE.into()),
    crate::IconClickTimer(Timer::from_seconds(1.0, TimerMode::Once)),
)]
#[component(on_add = on_computer_cctv_icon_add)]
pub struct ComputerCctvIcon;

fn on_computer_cctv_icon_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    let icon: Handle<Image> = world.resource::<AssetServer>().load("icons/geometrica/save-block.png");

    let icon_node = world.commands().spawn((
        Node {
            width: Val::Auto,
            height: Val::Percent(75.),
            ..default()
        },
        ImageNode::new(icon),
    )).id();

    let text_node = world.commands().spawn((
        Node {
            width: Val::Auto,
            height: Val::Percent(25.),
            ..default()
        },
        Text::new("CCTV"),
    )).id();

    world.commands()
        .entity(context.entity)
        .observe(super::icon_drag_observer)
        .observe(super::icon_over)
        .observe(super::icon_out)
        .observe(cctv_icon_double_click_observer)
        .add_child(icon_node)
        .add_child(text_node);
}

pub(crate) fn cctv_icon_double_click_observer(
    trigger: On<Pointer<Click>>,
    mut timer_query: Query<&mut super::IconClickTimer>,
    mut commands: Commands,
    computer_ui_query: Query<Entity, With<crate::ComputerUiNode>>,
    cctv_camera_query: Query<Entity, With<crate::CctvCam>>,
) {
    trace!("OBSERVER: cctv_icon_double_click_observer");

    let Ok(mut timer) = timer_query.get_mut(trigger.entity) else {
        error!("cctv_icon_double_click_observer: Failed to query IconClickTimer for {}", trigger.entity);
        return;
    };
    let Ok(computer_ui) = computer_ui_query.single() else {
        error!("cctv_icon_double_click_observer: Failed to query Entity for ComputerUiNode");
        return;
    };
    let Ok(cctv_camrea) = cctv_camera_query.single() else {
        error!("cctv_icon_double_click_observer: Failed to query Entity for CctvCam");
        return;
    };

    if timer.0.is_finished() {
        timer.0.reset();
    } else {
        let window = commands.spawn((
                crate::ComputerNode,
                floating_computer_rover_window_root("CCTV".to_string(), (
                    Node {
                        width: Val::Auto,
                        height: px(300),
                        border: UiRect::all(px(5)),
                        overflow: Overflow { x: OverflowAxis::Hidden, y: OverflowAxis::Hidden },
                        flex_direction: FlexDirection::ColumnReverse,
                        ..default()
                    },
                    ViewportNode::new(cctv_camrea),
                    BorderColor::all(Color::WHITE),
                    Children::spawn(SpawnWith(|root_parent: &mut ChildSpawner| {
                        root_parent.spawn(CctvButtonsNode);
                    })),
                )),
        )).id();

        commands.entity(computer_ui).add_child(window);
    }
}
