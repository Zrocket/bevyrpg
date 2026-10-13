use bevy::{color::palettes::css::{BLUE, GREEN, RED}, ecs::{lifecycle::HookContext, world::DeferredWorld}, prelude::*};

use crate::{Rover, RoverLeftEvent, RoverLeftStopEvent, RoverRightStopEvent, widgets::floating_windows::floating_computer_rover_window_root};

#[derive(Component)]
#[require(
    Node {
        position_type: PositionType::Absolute,
        width: Val::Percent(11.),
        height: Val::Percent(15.),
        align_items: AlignItems::Center,
        border_radius: BorderRadius::all(Val::Px(10.)),
        left: Val::Px(20.),
        top: Val::Px(400.),
        flex_direction: FlexDirection::Column,
        overflow: Overflow { x: OverflowAxis::Hidden, y: OverflowAxis::Hidden },
        ..default()
    },
    BackgroundColor(BLUE.into()),
    super::IconClickTimer(Timer::from_seconds(1.0, TimerMode::Once)),
)]
#[component(on_add = on_computer_rover_icon_add)]
pub struct ComputerRoverIcon;

fn on_computer_rover_icon_add(
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
        Text::new("ROVER"),
    )).id();

    world.commands()
        .entity(context.entity)
        .observe(super::icon_drag_observer)
        .observe(super::icon_over)
        .observe(super::icon_out)
        .observe(rover_icon_double_click_observer)
        .add_child(icon_node)
        .add_child(text_node);
}

#[derive(Component)]
#[require (
    Node {
        ..default()
    },
    Text("v".into()),
    BackgroundColor(GREEN.into()),
)]
#[component(on_add = on_camera_down_button_add)]
pub struct CameraDownButton;

fn on_camera_down_button_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    world.commands()
        .entity(context.entity)
        .observe(super::icon_over)
        .observe(super::icon_out)
        .observe(camera_down_pressed)
        .observe(camera_down_released)
        .observe(camera_down_leave);
}
#[derive(Component)]
#[require(
    Node {
        //width: px(300),
        //height: px(50),
        ..default()
    },
    BackgroundColor(GREEN.into()),
    Text("v".into()),
)]
#[component(on_add = on_backward_button_add)]
pub struct RoverBackwardButton;

fn on_backward_button_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    world.commands()
        .entity(context.entity)
        .observe(super::icon_over)
        .observe(super::icon_out)
        .observe(rover_backward_pressed)
        .observe(rover_backward_released)
        .observe(rover_backward_leave);
}
#[derive(Component)]
#[require (
    Node {
        ..default()
    },
    Text("^".into()),
    BackgroundColor(GREEN.into()),
)]
#[component(on_add = on_camera_up_button_add)]
pub struct CameraUpButton;

fn on_camera_up_button_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    world.commands()
        .entity(context.entity)
        .observe(super::icon_over)
        .observe(super::icon_out)
        .observe(camera_up_pressed)
        .observe(camera_up_released)
        .observe(camera_up_leave);
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
#[component(on_add = on_right_button_add)]
pub struct RightButton;

fn on_right_button_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    world.commands()
        .entity(context.entity)
        .observe(super::icon_over)
        .observe(super::icon_out)
        .observe(right_pressed)
        .observe(super::right_released)
        .observe(right_leave);
}
#[derive(Component)]
#[require(
    Node {
        ..default()
    },
    Text("P".into()),
    BackgroundColor(GREEN.into()),
)]
#[component(on_add = on_pickup_button_add)]
pub struct PickupButton;

fn on_pickup_button_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    world.commands()
        .entity(context.entity)
        .observe(pickup_pressed);
}

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
#[component(on_add = on_left_button_add)]
pub struct LeftButton;

fn on_left_button_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    world.commands()
        .entity(context.entity)
        .observe(super::icon_over)
        .observe(super::icon_out)
        .observe(left_pressed)
        .observe(left_released)
        .observe(left_leave);
}

#[derive(Component)]
#[require(
    Node {
        //width: px(300),
        //height: px(50),
        ..default()
    },
    BackgroundColor(GREEN.into()),
    Text("^".into()),
)]
#[component(on_add = on_forward_button_add)]
pub struct RoverForwardButton;

fn on_forward_button_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    world.commands()
        .entity(context.entity)
        .observe(super::icon_over)
        .observe(super::icon_out)
        .observe(rover_forward_pressed)
        .observe(rover_forward_released)
        .observe(rover_forward_leave);
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
#[component(on_add = on_buttons_node_add)]
pub struct RoverButtonsNode;

fn on_buttons_node_add(
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
            Children::spawn(SpawnWith(|parent: &mut ChildSpawner| {
                parent.spawn(RoverForwardButton);
            })),
    )).id();

    let middle = world.commands().spawn((
            Node {
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                flex_grow: 1.,
                ..default()
            },
            Children::spawn(SpawnWith(|parent: &mut ChildSpawner| {
                parent.spawn(LeftButton);
                parent.spawn(PickupButton);
                parent.spawn(RightButton);
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
                parent.spawn(CameraUpButton);
                parent.spawn(RoverBackwardButton);
                parent.spawn(CameraDownButton);
            })),
    )).id();

    world.commands()
        .entity(context.entity)
        .add_child(top)
        .add_child(middle)
        .add_child(bottom);
}

#[derive(Component)]
#[require (
    Node {
        ..default()
    },
    Text("ATTACHMENT".into()),
    BackgroundColor(GREEN.into()),
)]
#[component(on_add = on_attachment_button_add)]
pub struct AttachmentButton;

fn on_attachment_button_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    world.commands()
        .entity(context.entity)
        .observe(super::icon_over)
        .observe(super::icon_out)
        .observe(attachment_pressed);
}

#[derive(Component)]
#[require(
    Node {
        ..default()
    },
    Text("RECALL".into()),
    BackgroundColor(GREEN.into()),
)]
#[component(on_add = on_recall_button_add)]
pub struct RecallButton;

fn on_recall_button_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    world.commands()
        .entity(context.entity)
        .observe(super::icon_over)
        .observe(super::icon_out)
        .observe(recall_pressed);
}

pub(crate) fn rover_icon_double_click_observer(
    trigger: On<Pointer<Click>>,
    mut timer_query: Query<&mut super::IconClickTimer>,
    mut commands: Commands,
    computer_ui_query: Query<Entity, With<super::ComputerUiNode>>,
    rover_camera_query: Query<Entity, With<crate::RoverCamera>>,
) {
    let Ok(mut timer) = timer_query.get_mut(trigger.entity) else {
        error!("rover_icon_double_click_observer: Failed to query IconClickTimer for {}", trigger.entity);
        return;
    };
    let Ok(computer_ui) = computer_ui_query.single() else {
        error!("rover_icon_double_click_observer: Failed to queury Entity for ComputerUiNode");
        return;
    };
    let Ok(rover_camrea) = rover_camera_query.single() else {
        error!("rover_icon_double_click_observer: Failed to query Entity for RoverCamera");
        return;
    };

    if timer.0.is_finished() {
        timer.0.reset();
    } else {
        let window = commands.spawn((
                super::ComputerNode,
                floating_computer_rover_window_root("ROVER".to_string(), (
                    Node {
                        width: Val::Auto,
                        height: px(300),
                        border: UiRect::all(px(5)),
                        overflow: Overflow { x: OverflowAxis::Hidden, y: OverflowAxis::Hidden },
                        flex_direction: FlexDirection::ColumnReverse,
                        ..default()
                    },
                    ViewportNode::new(rover_camrea),
                    BorderColor::all(Color::WHITE),
                    Children::spawn(SpawnWith(|root_parent: &mut ChildSpawner| {
                        root_parent.spawn(RecallButton);
                        root_parent.spawn(AttachmentButton);
                        root_parent.spawn(RoverButtonsNode);
                    })),
                )),
        )).id();

        commands.entity(computer_ui).add_child(window);
    }
}

pub(crate) fn recall_pressed(
    _trigger: On<Pointer<Press>>,
    mut commands: Commands,
    rover_query: Query<Entity, With<crate::Rover>>,
) {
    let Ok(rover_entity) = rover_query.single() else {
        error!("recall_pressed: Failed to query Entity forr Rover");
        return;
    };

    //if let Ok(rover_entity) = rover_query.single() {
    commands.entity(rover_entity).trigger(|entity| crate::RoverRecallEvent { entity });
    //}
}

pub(crate) fn attachment_pressed(
    _trigger: On<Pointer<Press>>,
    mut commands: Commands,
    rover_query: Query<(Entity, &crate::RoverAttachments), With<crate::Rover>>,
) {
    let Ok((_rover_entity, attachment)) = rover_query.single() else {
        error!("attachment_pressed: Failed to query Entity and RoverAttachments for Rover");
        return;
    };

    for entity in  attachment.iter() {
        commands.entity(entity).trigger(|entity| crate::UseRoverAttachmentEvent { entity });
    }
}

pub(crate) fn pickup_pressed(
    _trigger: On<Pointer<Press>>,
    mut commands: Commands,
    rover_query: Query<Entity, With<crate::Rover>>,
) {
    let Ok(rover_entity) = rover_query.single() else {
        error!("pickup_pressed: Failed to query Entity for Rover");
        return;
    };

    commands.entity(rover_entity).trigger(|entity| crate::RoverInteractEvent { entity });
}

pub(crate) fn rover_backward_pressed(
    _trigger: On<Pointer<Press>>,
    mut commands: Commands,
    rover_query: Query<Entity, With<crate::Rover>>,
) {
    let Ok(rover_entity) = rover_query.single() else {
        error!("rover_backward_pressed: Failed to query Entity for Rover");
        return;
    };

    commands.entity(rover_entity).trigger(|entity| crate::RoverBackwardEvent { entity });
}

pub(crate) fn rover_backward_released(
    _trigger: On<Pointer<Release>>,
    mut commands: Commands,
    rover_query: Query<Entity, With<crate::Rover>>,
) {
    let Ok(rover_entity) = rover_query.single() else {
        error!("rover_backward_released: Failed to query Entity for Rover");
        return;
    };

    //if let Ok(rover_entity) = rover_query.single() {
    commands.entity(rover_entity).trigger(|entity| crate::RoverBackwardStopEvent { entity });
    //}
}

pub(crate) fn rover_backward_leave(
    _trigger: On<Pointer<Leave>>,
    mut commands: Commands,
    rover_query: Query<Entity, With<crate::Rover>>,
) {
    let Ok(rover_entity) = rover_query.single() else {
        error!("rover_backward_released: Failed to query Entity for Rover");
        return;
    };

    commands.entity(rover_entity).trigger(|entity| crate::RoverBackwardStopEvent { entity });
}

pub(crate) fn camera_down_pressed(
    _trigger: On<Pointer<Press>>,
    mut commands: Commands,
    rover_query: Query<Entity, With<crate::Rover>>,
) {
    let Ok(rover_entity) = rover_query.single() else {
        error!("camera_down_pressed: Failed to query Entity for Rover");
        return;
    };

    //if let Ok(rover_entity) = rover_query.single() {
    commands.entity(rover_entity).trigger(|entity| crate::RoverCameraDownEvent { entity });
    //}
}

pub(crate) fn camera_down_released(
    _trigger: On<Pointer<Release>>,
    mut commands: Commands,
    rover_query: Query<Entity, With<crate::Rover>>,
) {
    let Ok(rover_entity) = rover_query.single() else {
        error!("camera_down_released: Failed to query Entity forr Rover");
        return;
    };

    commands.entity(rover_entity).trigger(|entity| crate::RoverCameraDownStopEvent { entity });
}

pub(crate) fn camera_down_leave(
    _trigger: On<Pointer<Leave>>,
    mut commands: Commands,
    rover_query: Query<Entity, With<crate::Rover>>,
) {
    let Ok(rover_entity) = rover_query.single() else {
        error!("camera_down_released: Failed to query Entity forr Rover");
        return;
    };

    commands.entity(rover_entity).trigger(|entity| crate::RoverCameraDownStopEvent { entity });
}

pub(crate) fn rover_forward_pressed(
    _trigger: On<Pointer<Press>>,
    mut commands: Commands,
    rover_query: Query<Entity, With<crate::Rover>>,
) {
    let Ok(rover_entity) = rover_query.single() else {
        error!("rover_forward_pressed: Failed to query Entity for Rover");
        return;
    };

    commands.entity(rover_entity).trigger(|entity| crate::RoverForwardEvent { entity });
}

pub(crate) fn rover_forward_released(
    _trigger: On<Pointer<Release>>,
    mut commands: Commands,
    rover_query: Query<Entity, With<crate::Rover>>,
) {
    let Ok(rover_entity) = rover_query.single() else {
        error!("rover_forward_released: Failed to query Entity for Rover");
        return;
    };

    commands.entity(rover_entity).trigger(|entity| crate::RoverForwardStopEvent { entity });
}

pub(crate) fn rover_forward_leave(
    _trigger: On<Pointer<Leave>>,
    mut commands: Commands,
    rover_query: Query<Entity, With<crate::Rover>>,
) {
    let Ok(rover_entity) = rover_query.single() else {
        error!("rover_forward_released: Failed to query Entity for Rover");
        return;
    };

    commands.entity(rover_entity).trigger(|entity| crate::RoverForwardStopEvent { entity });
}

pub(crate) fn right_pressed(
    _trigger: On<Pointer<Press>>,
    mut commands: Commands,
    rover_query: Query<Entity, With<crate::Rover>>,
) {
    let Ok(rover_entity) = rover_query.single() else {
        error!("right_pressed: Failed to query Entity for Rover");
        return;
    };

    commands.entity(rover_entity).trigger(|entity| crate::RoverRightEvent { entity });
}

pub(crate) fn right_released(
    _trigger: On<Pointer<Release>>,
    mut commands: Commands,
    rover_query: Query<Entity, With<Rover>>,
) {
    let Ok(rover_entity) = rover_query.single() else {
        error!("right_released: Failed to query Entity for Rover");
        return;
    };

    commands.entity(rover_entity).trigger(|entity| RoverRightStopEvent { entity });
}

pub(crate) fn left_pressed(
    _trigger: On<Pointer<Press>>,
    mut commands: Commands,
    rover_query: Query<Entity, With<Rover>>,
) {
    let Ok(rover_entity) = rover_query.single() else {
        error!("left_pressed: Failed to query Entity for Rover");
        return;
    };

    commands.entity(rover_entity).trigger(|entity| RoverLeftEvent { entity });
}

pub(crate) fn left_released(
    _trigger: On<Pointer<Release>>,
    mut commands: Commands,
    rover_query: Query<Entity, With<Rover>>,
) {
    let Ok(rover_entity) = rover_query.single() else {
        error!("left_released: Failed to query Entity for Rover");
        return;
    };

    commands.entity(rover_entity).trigger(|entity| RoverLeftStopEvent { entity });
}

pub(crate) fn left_leave(
    _trigger: On<Pointer<Leave>>,
    mut commands: Commands,
    rover_query: Query<Entity, With<Rover>>,
) {
    let Ok(rover_entity) = rover_query.single() else {
        error!("left_released: Failed to query Entity for Rover");
        return;
    };

    commands.entity(rover_entity).trigger(|entity| RoverLeftStopEvent { entity });
}

pub(crate) fn right_leave(
    _trigger: On<Pointer<Leave>>,
    mut commands: Commands,
    rover_query: Query<Entity, With<Rover>>,
) {
    let Ok(rover_entity) = rover_query.single() else {
        error!("right_released: Failed to query Entity for Rover");
        return;
    };
    commands.entity(rover_entity).trigger(|entity| RoverRightStopEvent { entity });
}

pub(crate) fn camera_up_released(
    _trigger: On<Pointer<Release>>,
    mut commands: Commands,
    rover_query: Query<Entity, With<Rover>>,
) {
    let Ok(rover_entity) = rover_query.single() else {
        error!("camera_up_released: Failed to query Entity for Rover");
        return;
    };

    commands.entity(rover_entity).trigger(|entity| crate::RoverCameraUpStopEvent { entity });
}

pub(crate) fn camera_up_leave(
    _trigger: On<Pointer<Leave>>,
    mut commands: Commands,
    rover_query: Query<Entity, With<Rover>>,
) {
    let Ok(rover_entity) = rover_query.single() else {
        error!("camera_up_released: Failed to query Entity for Rover");
        return;
    };

    commands.entity(rover_entity).trigger(|entity| crate::RoverCameraUpStopEvent { entity });
}

pub(crate) fn camera_up_pressed(
    _trigger: On<Pointer<Press>>,
    mut commands: Commands,
    rover_query: Query<Entity, With<Rover>>,
) {
    let Ok(rover_entity) = rover_query.single() else {
        error!("camera_up_pressed: Failed to query Entity for Rover");
        return;
    };

    commands.entity(rover_entity).trigger(|entity| crate::RoverCameraUpEvent { entity });
}
