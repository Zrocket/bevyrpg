use bevy::{camera::RenderTarget, color::palettes::css::{BLUE, GREEN, RED}, ecs::{lifecycle::HookContext, world::DeferredWorld}, input::ButtonState, picking::{backend::ray::RayMap, pointer::{Location, PointerAction, PointerInput}}, prelude::*, window::{PrimaryWindow, WindowEvent}};

use crate::{ComputerNode, ComputerUiNode, Desktop, IconClickTimer, Rover, RoverAttachments, RoverBackwardEvent, RoverCamera, RoverCameraDownEvent, RoverCameraUpEvent, RoverForwardEvent, RoverInteractEvent, RoverLeftEvent, RoverRecallEvent, RoverRightEvent, RoverSpawnedMessage, UseRoverAttachmentEvent, furniture::computer::{CUBE_POINTER_ID, ComputerScreenCube}, widgets::floating_windows::floating_computer_rover_window_root};

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
pub struct ForwardButton;

fn on_forward_button_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    world.commands()
        .entity(context.entity)
        .observe(icon_over)
        .observe(icon_out)
        .observe(forward_pressed)
        .observe(forward_released);
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
pub struct BackwardButton;

fn on_backward_button_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    world.commands()
        .entity(context.entity)
        .observe(icon_over)
        .observe(icon_out)
        .observe(backward_pressed)
        .observe(backward_released);
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
        .observe(icon_over)
        .observe(icon_out)
        .observe(left_pressed)
        .observe(left_released);
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
        .observe(icon_over)
        .observe(icon_out)
        .observe(right_pressed)
        .observe(right_released);
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
pub struct ButtonsNode;

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
                parent.spawn(ForwardButton);
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
                parent.spawn(BackwardButton);
                parent.spawn(CameraUpButton);
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
        .observe(icon_over)
        .observe(icon_out)
        .observe(recall_pressed);
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
        .observe(icon_over)
        .observe(icon_out)
        .observe(attachment_pressed);
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
        .observe(icon_over)
        .observe(icon_out)
        .observe(camera_up_pressed)
        .observe(camera_up_released);
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
        .observe(icon_over)
        .observe(icon_out)
        .observe(camera_down_pressed)
        .observe(camera_down_released);
}

pub(crate) fn pickup_pressed(
    _trigger: On<Pointer<Press>>,
    mut commands: Commands,
    rover_query: Query<Entity, With<Rover>>,
) {
    if let Ok(rover_entity) = rover_query.single() {
        commands.entity(rover_entity).trigger(|entity| RoverInteractEvent { entity });
    }
}

pub(crate) fn forward_pressed(
    _trigger: On<Pointer<Press>>,
    mut commands: Commands,
    rover_query: Query<Entity, With<Rover>>,
) {
    if let Ok(rover_entity) = rover_query.single() {
        commands.entity(rover_entity).trigger(|entity| RoverForwardEvent { entity });
    }
}

pub(crate) fn forward_released(
    _trigger: On<Pointer<Release>>,
    mut commands: Commands,
    rover_query: Query<Entity, With<Rover>>,
) {
    if let Ok(rover_entity) = rover_query.single() {
        commands.entity(rover_entity).trigger(|entity| RoverForwardEvent { entity });
    }
}

pub(crate) fn backward_pressed(
    _trigger: On<Pointer<Press>>,
    mut commands: Commands,
    rover_query: Query<Entity, With<Rover>>,
) {
    if let Ok(rover_entity) = rover_query.single() {
        commands.entity(rover_entity).trigger(|entity| RoverBackwardEvent { entity });
    }
}

pub(crate) fn backward_released(
    _trigger: On<Pointer<Release>>,
    mut commands: Commands,
    rover_query: Query<Entity, With<Rover>>,
) {
    if let Ok(rover_entity) = rover_query.single() {
        commands.entity(rover_entity).trigger(|entity| RoverBackwardEvent { entity });
    }
}

pub(crate) fn left_pressed(
    _trigger: On<Pointer<Press>>,
    mut commands: Commands,
    rover_query: Query<Entity, With<Rover>>,
) {
    if let Ok(rover_entity) = rover_query.single() {
        commands.entity(rover_entity).trigger(|entity| RoverLeftEvent { entity });
    }
}

pub(crate) fn left_released(
    _trigger: On<Pointer<Release>>,
    mut commands: Commands,
    rover_query: Query<Entity, With<Rover>>,
) {
    if let Ok(rover_entity) = rover_query.single() {
        commands.entity(rover_entity).trigger(|entity| RoverLeftEvent { entity });
    }
}

pub(crate) fn right_pressed(
    _trigger: On<Pointer<Press>>,
    mut commands: Commands,
    rover_query: Query<Entity, With<Rover>>,
) {
    if let Ok(rover_entity) = rover_query.single() {
        commands.entity(rover_entity).trigger(|entity| RoverRightEvent { entity });
    }
}

pub(crate) fn right_released(
    _trigger: On<Pointer<Release>>,
    mut commands: Commands,
    rover_query: Query<Entity, With<Rover>>,
) {
    if let Ok(rover_entity) = rover_query.single() {
        commands.entity(rover_entity).trigger(|entity| RoverRightEvent { entity });
    }
}

pub(crate) fn recall_pressed(
    _trigger: On<Pointer<Press>>,
    mut commands: Commands,
    rover_query: Query<Entity, With<Rover>>,
) {
    if let Ok(rover_entity) = rover_query.single() {
        commands.entity(rover_entity).trigger(|entity| RoverRecallEvent { entity });
    }
}

pub(crate) fn attachment_pressed(
    _trigger: On<Pointer<Press>>,
    mut commands: Commands,
    rover_query: Query<(Entity, &RoverAttachments), With<Rover>>,
) {
    if let Ok((rover_entity, attachment)) = rover_query.single() {
    for entity in  attachment.iter() {
        commands.entity(entity).trigger(|entity| UseRoverAttachmentEvent { entity });
    }
    }
}

pub(crate) fn camera_up_pressed(
    _trigger: On<Pointer<Press>>,
    mut commands: Commands,
    rover_query: Query<Entity, With<Rover>>,
) {
    if let Ok(rover_entity) = rover_query.single() {
        commands.entity(rover_entity).trigger(|entity| RoverCameraUpEvent { entity });
    }
}

pub(crate) fn camera_up_released(
    _trigger: On<Pointer<Release>>,
    mut commands: Commands,
    rover_query: Query<Entity, With<Rover>>,
) {
    if let Ok(rover_entity) = rover_query.single() {
        commands.entity(rover_entity).trigger(|entity| RoverCameraUpEvent { entity });
    }
}

pub(crate) fn camera_down_pressed(
    _trigger: On<Pointer<Press>>,
    mut commands: Commands,
    rover_query: Query<Entity, With<Rover>>,
) {
    if let Ok(rover_entity) = rover_query.single() {
        commands.entity(rover_entity).trigger(|entity| RoverCameraDownEvent { entity });
    }
}

pub(crate) fn camera_down_released(
    _trigger: On<Pointer<Release>>,
    mut commands: Commands,
    rover_query: Query<Entity, With<Rover>>,
) {
    if let Ok(rover_entity) = rover_query.single() {
        commands.entity(rover_entity).trigger(|entity| RoverCameraDownEvent { entity });
    }
}

pub(crate) fn icon_over(
    over: On<Pointer<Over>>,
    mut colors: Query<&mut BackgroundColor>,
) {
    if let Ok(mut colors) = colors.get_mut(over.entity) {
        colors.0 = RED.into();
    }
}

pub(crate) fn icon_out(
    out: On<Pointer<Out>>,
    mut colors: Query<&mut BackgroundColor>,
) {
    if let Ok(mut colors) = colors.get_mut(out.entity) {
        colors.0 = BLUE.into();
    }
}

pub(crate) fn update_click_timer(
    mut timer_query: Query<&mut IconClickTimer>,
    time: Res<Time>,
) {
    for mut timer in timer_query.iter_mut() {
        timer.0.tick(time.delta());
    }
}

pub(crate) fn rover_icon_double_click_observer(
    trigger: On<Pointer<Click>>,
    mut timer_query: Query<&mut IconClickTimer>,
    mut commands: Commands,
    computer_ui_query: Query<Entity, With<ComputerUiNode>>,
    rover_camera_query: Query<Entity, With<RoverCamera>>,
) {
    if let Ok(mut timer) = timer_query.get_mut(trigger.entity)
    && let Ok(computer_ui) = computer_ui_query.single()
    && let Ok(rover_camrea) = rover_camera_query.single() {
        if timer.0.is_finished() {
            timer.0.reset();
        } else {
            let window = commands.spawn((
                    ComputerNode,
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
                            root_parent.spawn(ButtonsNode);
                        })),
                    )),
            )).id();

            commands.entity(computer_ui).add_child(window);
        }
    }
}

pub(crate) fn cctv_icon_double_click_observer(
    trigger: On<Pointer<Click>>,
    mut timer_query: Query<&mut IconClickTimer>,
    mut commands: Commands,
    computer_ui_query: Query<Entity, With<ComputerUiNode>>,
    rover_camera_query: Query<Entity, With<RoverCamera>>,
) {
    if let Ok(mut timer) = timer_query.get_mut(trigger.entity)
    && let Ok(computer_ui) = computer_ui_query.single()
    && let Ok(rover_camrea) = rover_camera_query.single() {
        if timer.0.is_finished() {
            timer.0.reset();
        } else {
            let window = commands.spawn((
                    ComputerNode,
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
                            root_parent.spawn(ButtonsNode);
                        })),
                    )),
            )).id();

            commands.entity(computer_ui).add_child(window);
        }
    }
}

pub(crate) fn refresh_rover_window(
    mut commands: Commands,
    mut rover_spawned_message_readeer: MessageReader<RoverSpawnedMessage>,
    rover_camera_query: Query<Entity, With<RoverCamera>>,
    window_query: Query<Entity, With<ComputerNode>>
) {
    for _message in rover_spawned_message_readeer.read() {
        if let Ok(rover_camrea) = rover_camera_query.single()
        && let Ok(window_entity) = window_query.single() {
            commands.entity(window_entity)
                .remove::<ViewportNode>()
                .insert(ViewportNode::new(rover_camrea));
        }
    }
}

pub(crate) fn icon_drag_observer(
    drag: On<Pointer<Drag>>,
    mut nodes: Query<(&mut Node, &ComputedNode)>,
) {
    if let Ok((mut node, computed)) = nodes.get_mut(drag.entity) {
        node.left = Val::Px(drag.pointer_location.position.x - computed.size.x / 2.0);
        node.top = Val::Px(drag.pointer_location.position.y - 50.0);
    }
}

/// Because bevy has no way to know how to map a mouse input to the UI texture, we need to write a
/// system that tells it there is a pointer on the UI texture. We cast a ray into the scene and find
/// the UV (2D texture) coordinates of the raycast hit. This UV coordinate is effectively the same
/// as a pointer coordinate on a 2D UI rect.
#[allow(clippy::too_many_arguments)]
pub(crate) fn drive_diegetic_pointer(
    mut cursor_last: Local<Vec2>,
    mut raycast: MeshRayCast,
    rays: Res<RayMap>,
    cubes: Query<&Mesh3d, Or<(With<ComputerScreenCube>, With<Desktop>)>>,
    ui_camera: Query<&RenderTarget, With<Camera2d>>,
    primary_window: Query<Entity, With<PrimaryWindow>>,
    windows: Query<(Entity, &Window)>,
    images: Res<Assets<Image>>,
    manual_texture_views: Res<ManualTextureViews>,
    mut window_events: MessageReader<WindowEvent>,
    mut pointer_inputs: MessageWriter<PointerInput>,
) -> Result {
    // Get the size of the texture, so we can convert from dimensionless UV coordinates that span
    // from 0 to 1, to pixel coordinates.
    let target = ui_camera
        .single()?
        .normalize(primary_window.single().ok())
        .unwrap();
    let target_info = target
        .get_render_target_info(windows, &images, &manual_texture_views)
        .unwrap();
    let size = target_info.physical_size.as_vec2();

    // Find raycast hits and update the virtual pointer.
    let raycast_settings = MeshRayCastSettings {
        visibility: RayCastVisibility::VisibleInView,
        filter: &|entity| cubes.contains(entity),
        early_exit_test: &|_| false,
    };
    for (_id, ray) in rays.iter() {
        for (_cube, hit) in raycast.cast_ray(*ray, &raycast_settings) {
            let position = size * hit.uv.unwrap();
            if position != *cursor_last {
                pointer_inputs.write(PointerInput::new(
                    CUBE_POINTER_ID,
                    Location {
                        target: target.clone(),
                        position,
                    },
                    PointerAction::Move {
                        delta: position - *cursor_last,
                    },
                ));
                *cursor_last = position;
            }
        }
    }

    // Pipe pointer button presses to the virtual pointer on the UI texture.
    for window_event in window_events.read() {
        if let WindowEvent::MouseButtonInput(input) = window_event {
            let button = match input.button {
                MouseButton::Left => PointerButton::Primary,
                MouseButton::Right => PointerButton::Secondary,
                MouseButton::Middle => PointerButton::Middle,
                _ => continue,
            };
            let action = match input.state {
                ButtonState::Pressed => PointerAction::Press(button),
                ButtonState::Released => PointerAction::Release(button),
            };
            pointer_inputs.write(PointerInput::new(
                CUBE_POINTER_ID,
                Location {
                    target: target.clone(),
                    position: *cursor_last,
                },
                action,
            ));
        }
    }

    Ok(())
}
