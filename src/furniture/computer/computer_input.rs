use bevy::{camera::RenderTarget, color::palettes::css::{BLUE, RED}, input::ButtonState, picking::{backend::ray::RayMap, pointer::{Location, PointerAction, PointerInput}}, prelude::*, window::{PrimaryWindow, WindowEvent}};

use crate::{ComputerNode, Desktop, IconClickTimer, RoverCamera, RoverSpawnedMessage, furniture::computer::{CUBE_POINTER_ID, ComputerScreenCube}};

pub(crate) fn icon_over(
    over: On<Pointer<Over>>,
    mut colors: Query<&mut BackgroundColor>,
) {
    let Ok(mut colors) = colors.get_mut(over.entity) else {
        error!("ivon_over: Failed to query BackgroundColor for {}", over.entity);
        return;
    };

    //if let Ok(mut colors) = colors.get_mut(over.entity) {
    colors.0 = RED.into();
    //}
}

pub(crate) fn icon_out(
    out: On<Pointer<Out>>,
    mut colors: Query<&mut BackgroundColor>,
) {
    let Ok(mut colors) = colors.get_mut(out.entity) else {
        error!("icon_out: Failed to query BackgroundColor for {}", out.entity);
        return;
    };

    //if let Ok(mut colors) = colors.get_mut(out.entity) {
    colors.0 = BLUE.into();
    //}
}

pub(crate) fn update_click_timer(
    mut timer_query: Query<&mut IconClickTimer>,
    time: Res<Time>,
) {
    for mut timer in timer_query.iter_mut() {
        timer.0.tick(time.delta());
    }
}

pub(crate) fn refresh_rover_window(
    mut commands: Commands,
    mut rover_spawned_message_readeer: MessageReader<RoverSpawnedMessage>,
    rover_camera_query: Query<Entity, With<RoverCamera>>,
    window_query: Query<Entity, With<ComputerNode>>
) {
    for _message in rover_spawned_message_readeer.read() {
        let Ok(rover_camrea) = rover_camera_query.single() else {
            error!("refresh_rover_window: Failed to query Entity for RoverCamera");
            return;
        };
        let Ok(window_entity) = window_query.single() else {
            error!("refresh_rover_window: Failed to query Entity for ComputerNode");
            return;
        };

        commands.entity(window_entity)
            .remove::<ViewportNode>()
            .insert(ViewportNode::new(rover_camrea));
    }
}

pub(crate) fn icon_drag_observer(
    drag: On<Pointer<Drag>>,
    mut nodes: Query<(&mut Node, &ComputedNode)>,
) {
    if let Ok((mut node, computed)) = nodes.get_mut(drag.entity) {
        node.left = Val::Px(drag.pointer_location.position.x - computed.size.x / 2.0);
        node.top = Val::Px(drag.pointer_location.position.y - 50.0);
    } else {
        error!("icon_drag_observer: Failed to query Node and ComputedNode for {}", drag.entity);
    }
}

/// Because bevy has no way to know how to map a mouse input to the UI texture, we need to write a
/// system that tells it there is a pointer on the UI texture. We cast a ray into the scene and find
/// the UV (2D texture) coordinates of the raycast hit. This UV coordinate is effectively the same
/// as a pointer coordinate on a 2D UI rect.
#[allow(clippy::too_many_arguments)]
pub(crate) fn drive_diegetic_pointer(
    mut cursor_last: Local<Vec2>,
    mut hitting_screen: Local<bool>,
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

    let mut hit_this_frame = false;
    for (_id, ray) in rays.iter() {
        for (_cube, hit) in raycast.cast_ray(*ray, &raycast_settings) {
            hit_this_frame = true;
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

    if *hitting_screen && !hit_this_frame {
        pointer_inputs.write(PointerInput::new(
                CUBE_POINTER_ID,
                Location { target: target.clone(), position: *cursor_last },
                PointerAction::Cancel,
        ));
    }
    *hitting_screen = hit_this_frame;

    // Pipe pointer button presses to the virtual pointer on the UI texture.
    for window_event in window_events.read() {
        if !hit_this_frame {
            continue;
        }
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
