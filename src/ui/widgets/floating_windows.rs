use bevy::{color::palettes::css::{CRIMSON, DARK_CYAN, DARK_GREEN, DARK_VIOLET}, ecs::{reflect, system::{IntoObserverSystem, entity_command::observe}}, feathers::cursor::{self, CursorIconPlugin}, math::{I8Vec2, VectorSpace, bounding::Aabb2d}, picking::hover::Hovered, prelude::*, ui::Pressed, window::{SystemCursorIcon, WindowClosed}};
use rand::Rng;

use crate::widgets::floating_windows;
use crate::widgets::floating_window_ordering::UiZOrderLayer;

#[derive(EntityEvent)]
#[entity_event(propagate, auto_propagate)]
pub struct CloseWindowEvent {
    entity: Entity,
}

#[derive(EntityEvent)]
#[entity_event(propagate, auto_propagate)]
pub struct MinimizeWindowEvent {
    entity: Entity
}

#[derive(EntityEvent)]
#[entity_event(propagate, auto_propagate)]
pub struct MaximizeWindowEvent {
    entity: Entity
}

#[derive(Component)]
struct WindowTitalBar;

#[derive(Component, Copy, Clone)]
struct WindowResize;

pub struct FloatingWindowPlugin;
impl Plugin for FloatingWindowPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CursorIconPlugin);

       app.add_observer(window_on_drag_start)
           .add_observer(window_on_drag)
           .add_observer(window_on_drag_end);

        app.add_observer(window_resize_drag_start)
            .add_observer(window_resize_drag)
            .add_observer(window_resize_drag_end);

        app.insert_resource(FloatingWindowResizeState::default());
        app.add_systems(Update, update_bevy_feathers_cursor);

        app.add_systems(PostUpdate, 
            update_floating_window_node);

        app.add_systems(
            PostUpdate,
            init_floating_window);

        app.register_type::<FloatingWindow>();
    }
}

#[derive(Component)]
#[require(Hovered)]
pub struct WindowResizeDragDirection(pub I8Vec2);

/// Stores current state related to window resizing
#[derive(Resource, Default)]
pub struct FloatingWindowResizeState {
    /// Stores information about which border is dragged to resize wiwndow
    ///
    /// Usefull to update cursor values.
    pub dragging: Option<Entity>,
}

#[derive(Component, Default)]
pub struct FloatingWindowInteractionState {
    // For dragging
    currerntly_drag: bool,
    initial_dragz_pos: Vec2,
    drag_last_offset: Option<Vec2>,

    // For resize
    currently_resize: bool,
    initial_resize_size: Vec2,
    initial_resize_offset: Vec2,
}

#[derive(Component)]
pub struct FloatingWindowLocation {
    offset_px: Vec2,
    size_px: Vec2,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(FloatingWindowInteractionState, UiZOrderLayer)]
pub struct FloatingWindow {
    pub initial_width: Val,
    pub initial_height: Val,
    pub min_width: Val,
    pub min_height: Val,
    pub max_width: Val,
    pub max_height: Val,

    /// Ratio of how much floating window should be inside
    /// window.
    ///
    /// Ration used to move windows inside screen when
    /// they are fully outside the view.
    pub overlap_ratio: Option<f32>,
}

impl Default for FloatingWindow {
    fn default() -> Self {
        Self {
            initial_width: Val::Px(50.),
            initial_height: Val::Px(50.),
            min_width: Val::Vw(90.),
            min_height: Val::Vh(90.),
            max_width: Val::Auto,
            max_height: Val::Auto,
            overlap_ratio: Some(0.075),
        }
    }
}

pub fn init_floating_window(
    mut window_query: Query<(&mut FloatingWindow, &mut Node), Added<FloatingWindow>>,
) {
    let mut rng = rand::rng();
    for (mut window, mut node) in window_query.iter_mut() {
        node.min_width = window.initial_width;
        node.min_height = window.initial_height;
        node.max_width = window.initial_width;
        node.max_height = window.initial_height;

        node.left = px(rng.random_range(10..500i32) as f32);
        node.top = px(rng.random_range(10..500i32) as f32);
    }
}

#[allow(clippy::type_complexity)]
fn update_floating_window_node(
    mut query: Query<(
        &FloatingWindow,
        &mut Node,
        &FloatingWindowInteractionState,
        &ComputedNode,
        &ComputedUiRenderTargetInfo,
        &UiGlobalTransform,
    ),
    Or<(
        Changed<FloatingWindow>,
        Changed<FloatingWindowInteractionState>,
        Changed<ComputedNode>,
        Changed<ComputedUiRenderTargetInfo>,
        Changed<UiGlobalTransform>,
    )>,
    >,
) {
    for (
        floating_window,
        mut node,
        interaction_state,
        comp_node,
        comp_target_info,
        global_transform
    ) in query.iter_mut() {
        let Some(overlap_ratio) = floating_window.overlap_ratio else {
            continue;
        };

        if interaction_state.currerntly_drag || interaction_state.currently_resize {
            continue;
        }

        if (node.width == Val::Auto || node.height == Val::Auto)
            // Detect if layout calculations have been done
            && comp_node.size.min_element() > 10.
        {
            // Fixed size is needed to force overflowing elements to not overflow
            // See `Bevy Scrollareas` bevy_immediate example floating window behaviour.
            node.min_width = px(comp_node.size.x * comp_node.inverse_scale_factor);
            node.min_height = px(comp_node.size.y * comp_node.inverse_scale_factor);
        }

        let window_node = Aabb2d::new(global_transform.translation, comp_node.size * 0.5);
        let camera = Aabb2d {
            max: comp_target_info.physical_size().as_vec2(),
            min: Vec2::ZERO,
        };

        if fully_inside(&window_node, &camera) {
            // Fully inside, no need to check overlap
            continue;
        }

        let int_min = window_node.min.max(camera.min);
        let int_max = window_node.max.min(camera.max);

        let overlap = (int_max - int_min).max(Vec2::ZERO);
        let needed_overlap =
            (comp_target_info.physical_size().as_vec2() * overlap_ratio).min(comp_node.size);

        if overlap.x >= needed_overlap.x && overlap.y >= needed_overlap.y {
            continue;
        }

        let mut offset_to_add = (needed_overlap - overlap).max(Vec2::ZERO);
        if window_node.min.x > 0. {
            offset_to_add.x *= -1.;
        }
        if window_node.min.y > 0. {
            offset_to_add.y *= -1.;
        }

        let left = resolve_x(node.left, comp_target_info).unwrap_or(0.) + offset_to_add.x;
        let top = resolve_x(node.top, comp_target_info).unwrap_or(0.) + offset_to_add.y;

        node.left = px(left * comp_node.inverse_scale_factor);
        node.top = px(top * comp_node.inverse_scale_factor);
    }
}

/// Calculate if [`Aabb2d`] is fully inside another [`Aabb2d`]
fn fully_inside(inside: &Aabb2d, outside: &Aabb2d) -> bool {
    outside.min.x <= inside.min.x
        && inside.max.x <= outside.max.x
        && outside.min.y <= inside.min.y
        && inside.max.y <= outside.max.y
}

/// Wrapper function for resolving x into physical pixels
fn resolve_x(
    y: Val,
    target_info: &ComputedUiRenderTargetInfo,
) -> Result<f32, ValArithmeticError> {
    y.resolve(
        target_info.scale_factor(),
        target_info.physical_size().x as f32,
        target_info.physical_size().as_vec2()
    )
}

/// Wrapper function for resolving y into physical pixels
fn resolve_y(
    y: Val,
    target_info: &ComputedUiRenderTargetInfo,
) -> Result<f32, ValArithmeticError> {
    y.resolve(
        target_info.scale_factor(),
        target_info.physical_size().y as f32,
        target_info.physical_size().as_vec2()
    )
}

pub fn resizable_borders(border_thinkness: f32, additional: impl Bundle + Copy) -> impl Bundle {
    //children![
        (
        Node {
            display: Display::Grid,
            position_type: PositionType::Absolute,
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            left: Val::Px(-border_thinkness),
            top: Val::Px(-border_thinkness),
            grid_template_columns: vec![
                RepeatedGridTrack::px(1, border_thinkness),
                RepeatedGridTrack::percent(1, 100.),
                RepeatedGridTrack::px(1, border_thinkness),
            ],
            grid_template_rows: vec![
                RepeatedGridTrack::px(1, border_thinkness),
                RepeatedGridTrack::percent(1, 100.),
                RepeatedGridTrack::px(1, border_thinkness),
            ],
            row_gap: Val::Px(0.),
            column_gap: Val::Px(0.),
            align_items: AlignItems::Stretch,
            justify_items: JustifyItems::Stretch,
            ..default()
        },
        Pickable {
            should_block_lower: false,
            is_hoverable: false,
        },
        ZIndex(1),
        children![
            (
                Node::DEFAULT,
                WindowResizeDragDirection(I8Vec2 { x: -1, y: -1 }),
                //#[cfg(feature = "bevy_feathers")]
                cursor::EntityCursor::System(SystemCursorIcon::NwResize),
                additional
            ),
            (
                Node::DEFAULT,
                WindowResizeDragDirection(I8Vec2 { x: 0, y: -1 }),
                //#[cfg(feature = "bevy_feathers")]
                cursor::EntityCursor::System(SystemCursorIcon::NResize),
                additional
            ),
            (
                Node::DEFAULT,
                WindowResizeDragDirection(I8Vec2 { x: 1, y: -1 }),
                //#[cfg(feature = "bevy_feathers")]
                cursor::EntityCursor::System(SystemCursorIcon::NeResize),
                additional
            ),
            (
                Node::DEFAULT,
                WindowResizeDragDirection(I8Vec2 { x: -1, y: 0 }),
                //#[cfg(feature = "bevy_feathers")]
                cursor::EntityCursor::System(SystemCursorIcon::WResize),
                additional
            ),
            (
                Node::DEFAULT,
                Pickable {
                    should_block_lower: false,
                    is_hoverable: false,
                },
            ),
            (
                Node::DEFAULT,
                WindowResizeDragDirection(I8Vec2 { x: 1, y: 0 }),
                //#[cfg(feature = "bevy_feathers")]
                cursor::EntityCursor::System(SystemCursorIcon::EResize),
                additional
            ),
            (
                Node::DEFAULT,
                WindowResizeDragDirection(I8Vec2 { x: -1, y: 1 }),
                //#[cfg(feature = "bevy_feathers")]
                cursor::EntityCursor::System(SystemCursorIcon::SwResize),
                additional
            ),
            (
                Node::DEFAULT,
                WindowResizeDragDirection(I8Vec2 { x: 0, y: 1 }),
                //#[cfg(feature = "bevy_feathers")]
                cursor::EntityCursor::System(SystemCursorIcon::SResize),
                additional
            ),
            (
                Node::DEFAULT,
                WindowResizeDragDirection(I8Vec2 { x: 1, y: 1 }),
                //#[cfg(feature = "bevy_feathers")]
                cursor::EntityCursor::System(SystemCursorIcon::SeResize),
                additional
            ),
        ]
    )
   // ]
}

/// Function to update bevy feathers cursor while resizing window
fn update_bevy_feathers_cursor(
    default_cursor: Option<ResMut<cursor::DefaultCursor>>,
    resize_state: ResMut<FloatingWindowResizeState>,
    entity_cursor_query: Query<&cursor::EntityCursor>,
    mut stored_cursor: Local<Option<Option<cursor::EntityCursor>>>,
) {
    // TODO IN bevy 0.18 use the new mechanism
    let Some(mut default_cursor) = default_cursor else {
        return;
    };

    match (resize_state.dragging, stored_cursor.as_mut()) {
        (None, None) | (Some(_), Some(_)) => {
            // State didn't change
        },
        (None, Some(test)) => {
            // Restore stored cursor
            let cursor = stored_cursor.take().unwrap();
            if let Some(cursor) = cursor {
                default_cursor.0 = cursor;
            }
        },
        (Some(entity), None) => {
            let new_cursor = entity_cursor_query.get(entity).map(|c| c.clone()).ok();

            if let Some(mut new_cursor) = new_cursor {
                std::mem::swap(&mut new_cursor, &mut default_cursor.0);
                *stored_cursor = Some(Some(new_cursor));
            } else {
                *stored_cursor = Some(None);
            }
        }
    }
}

fn window_on_drag_start(
    mut drag_start: On<Pointer<DragStart>>,
    mut scroll_position_query: Query<(&UiGlobalTransform, &mut FloatingWindowInteractionState)>,
) {
    if let Ok((transform, mut state)) = scroll_position_query.get_mut(drag_start.entity) {
        // Store initial position
        state.initial_dragz_pos = transform.translation;
        state.drag_last_offset = None;
        // Set currently draging state to true
        state.currerntly_drag = true;

        // Disable event propagation
        drag_start.propagate(false);
    }
}

fn block_pointer_events(
    mut trigger: On<Pointer<Drag>>,
) {
    trigger.propagate(false);
}

fn window_on_drag(
    mut drag: On<Pointer<Drag>>,
    mut scroll_position_query: Query<(
        &mut FloatingWindowInteractionState,
        &mut Node,
        &ComputedNode,
        Option<&LayoutConfig>,
    )>,
    ui_scale: Res<UiScale>,
    mut global_transform: Query<&mut UiGlobalTransform>,
    children: Query<&Children>,
) {
    let Ok((mut state, mut node, comp_node, layout_config)) =
        scroll_position_query.get_mut(drag.entity)
    else {
        return;
    };

    // Disable Event propagation
    drag.propagate(false);

    let logical_distance = drag.distance / (comp_node.inverse_scale_factor * ui_scale.0);
    let logical_target_position = state.initial_dragz_pos + logical_distance;

    if state.drag_last_offset == Some(logical_target_position) {
        return;
    }
    state.drag_last_offset = Some(logical_target_position);

    apply_position(
        drag.entity,
        logical_target_position,
        &mut node,
        comp_node,
        layout_config,
        &children,
        &mut global_transform
    );
}

fn window_on_drag_end(
    mut drag: On<Pointer<DragEnd>>,
    mut scroll_position_query: Query<&mut FloatingWindowInteractionState>
) {
    if let Ok(mut state) = scroll_position_query.get_mut(drag.entity) {
        state.currerntly_drag = false;

        // Disable Event propagation
        drag.propagate(false);
    }
}

fn window_resize_drag(
    mut drag: On<Pointer<Drag>>,
    mut position_query: Query<&WindowResizeDragDirection>,
    parents_query: Query<&ChildOf>,
    mut windows_query: Query<(
        &mut Node,
        &FloatingWindow,
        &FloatingWindowInteractionState,
        &ComputedNode,
        &ComputedUiRenderTargetInfo,
    )>,
    ui_scale: Res<UiScale>,
) {
    // Get drag WindowResizeDragDirection from trigger entity
    let Ok(drag_direction) = position_query.get_mut(drag.entity) else {
        return;
    };

    // Disable drag propogation
    drag.propagate(false);

    // Get parent window Entity
    let Some(window_entity) = parents_query
        .iter_ancestors(drag.entity)
        .find(|ancestor| windows_query.contains(*ancestor))
    else {
        return;
    };

    let Ok((
        mut window_node,
        floating_window,
        floating_window_inter_state,
        window_comp_node,
        window_comp_target_info,
    )) = windows_query.get_mut(window_entity)
    else {
        return;
    };

    // Distance in physical coordinates
    let logical_distance = drag.distance / (window_comp_node.inverse_scale_factor * ui_scale.0);

    // Retrieve only necessary dimensions
    let drag_direction = drag_direction.0;
    let delta = drag_direction.as_vec2().abs() * logical_distance;

    let mut size_change = Vec2::ZERO;
    let mut left_top_change = Vec2::ZERO;

    if drag_direction.x < 0 {
        left_top_change.x = 1.;
        size_change.x += -delta.x;
    } else if drag_direction.x > 0 {
        size_change.x += delta.x;
    }

    if drag_direction.y < 0 {
        left_top_change.y = 1.;
        size_change.y += -delta.y;
    } else if drag_direction.y > 0 {
        size_change.y += delta.y;
    }

    if size_change != Vec2::ZERO {
        let width = floating_window_inter_state.initial_resize_size.x;
        let height = floating_window_inter_state.initial_resize_size.y;

        // TODO: Set real min width in floating window configuration
        let mut final_width = width + size_change.x;
        let mut final_height = height + size_change.y;

        final_width = final_width
            .min(
                resolve_x(floating_window.max_width, window_comp_target_info)
                .unwrap_or(window_comp_target_info.physical_size().x as f32),
            )
            .max(resolve_x(floating_window.min_width, window_comp_target_info).unwrap_or(50.));

        final_height = final_height
            .min(
                resolve_x(floating_window.max_height, window_comp_target_info)
                .unwrap_or(window_comp_target_info.physical_size().y as f32),
            )
            .max(resolve_x(floating_window.min_height, window_comp_target_info).unwrap_or(50.));

        window_node.min_width = px(final_width * window_comp_node.inverse_scale_factor);
        window_node.min_height = px(final_height * window_comp_node.inverse_scale_factor);
        window_node.width = px(final_width * window_comp_node.inverse_scale_factor);
        window_node.height = px(final_height * window_comp_node.inverse_scale_factor);

        if left_top_change != Vec2::ZERO {
            let mut left = floating_window_inter_state.initial_resize_offset.x;
            let mut top = floating_window_inter_state.initial_resize_offset.y;

            left += (width - final_width) * left_top_change.x;
            top += (height - final_height) * left_top_change.y;

            window_node.left = px(left * window_comp_node.inverse_scale_factor);
            window_node.top = px(top * window_comp_node.inverse_scale_factor);
        }
    }
}

fn window_resize_drag_end(
    drag_end: On<Pointer<DragEnd>>,
    mut target_query: Query<(), With<WindowResizeDragDirection>>,
    parents_query: Query<&ChildOf>,
    mut commands: Commands,
    mut windows_query: Query<&mut FloatingWindowInteractionState, With<FloatingWindow>>,
    mut active_state: ResMut<FloatingWindowResizeState>,
) {
    let Ok(()) = target_query.get_mut(drag_end.entity) else {
        return;
    };
    commands.entity(drag_end.entity).remove::<Pressed>();

    // Get parent window Entity
    let Some(window_entity) = parents_query
        .iter_ancestors(drag_end.entity)
        .find(|ancestor| windows_query.contains(*ancestor))
    else {
        return;
    };

    let Ok(mut window_interaction_state) = windows_query.get_mut(window_entity)
    else {
        return;
    };

    window_interaction_state.currently_resize = false;

    active_state.dragging = None;
}

fn window_resize_drag_start(
    mut drag_start: On<Pointer<DragStart>>,
    mut target_query: Query<(), With<WindowResizeDragDirection>>,
    mut windows_query: Query<(
        &Node,
        &mut FloatingWindowInteractionState,
        &ComputedNode,
        &ComputedUiRenderTargetInfo,
    ),
    With<FloatingWindow>,
    >,
    parents_query: Query<&ChildOf>,
    mut commands: Commands,
    mut active_state: ResMut<FloatingWindowResizeState>,
) {
    let Ok(()) = target_query.get_mut(drag_start.entity) else {
        return;
    };
    // Avoid window dragging
    drag_start.propagate(false);

    commands.entity(drag_start.entity).insert(Pressed);

    active_state.dragging = Some(drag_start.entity);

    let Some(window_entity) = parents_query
        .iter_ancestors(drag_start.entity)
        .find(|ancestor| windows_query.contains(*ancestor))
    else {
        return;
    };

    let Ok((node, mut window_interaction_state, window_comp_node, window_comp_target_info)) =
        windows_query.get_mut(window_entity)
    else {
        return;
    };

    window_interaction_state.currently_resize = true;
    window_interaction_state.initial_resize_size = window_comp_node.size;
    window_interaction_state.initial_resize_offset = Vec2::new(
        resolve_x(node.left, window_comp_target_info).unwrap_or(0.),
        resolve_y(node.top, window_comp_target_info).unwrap_or(0.),
    );
}

/// Usefull healper functiion to correctly update global transformations
/// and top, left element position in [`bevy_ui::Node`]
/// for the whole subtree rooted at `current` entity.
pub fn apply_position(
    entity: Entity,
    mut final_position: Vec2,
    node: &mut Node,
    comp_node: &ComputedNode,
    layout_config: Option<&LayoutConfig>,
    children: &Query<&Children>,
    global_transform: &mut Query<&mut UiGlobalTransform>,
) {
    let mut offset = final_position - comp_node.size * 0.5;

    // This is needed to avoid 1px broken layouts where something doesn't align up correctly
    if layout_config.map(|lc| lc.use_rounding).unwrap_or(true) {
        let offset_rounded = offset.round();
        final_position += offset_rounded - offset; // Get final position in correct place
        offset =  offset_rounded;
    }

    let offset_px = offset * comp_node.inverse_scale_factor;
    node.left = px(offset_px.x);
    node.top = px(offset_px.y);

    let Ok(current) = global_transform.get(entity) else {
        return;
    };

    // Logic to avoid 1 frame delay
    // Global transform update is done immediatelly
    let delta = final_position - current.translation;

    update_global_transforms(entity, delta, children, global_transform);
}

/// Useful helper function to correctly update global transformations
/// for the whole subtree rooted at `current` entity.
fn update_global_transforms(
    current: Entity,
    delta: Vec2,
    children: &Query<&Children>,
    query: &mut Query<&mut UiGlobalTransform>,
) {
    if let Ok(mut global) = query.get_mut(current) {
        let mut transformation = **global;
        transformation.translation += delta;
        *global = transformation.into();
    }

    let Ok(current_children) = children.get(current) else {
        return;
    };

    for &child in current_children {
        update_global_transforms(child, delta, children, query);
    }
}

pub fn close_button() -> impl Bundle
{
    (
        Node::default(),
        Children::spawn(SpawnWith(|parent: &mut ChildSpawner| {
            parent.spawn((
                Node {
                    flex_grow: 0.,
                    aspect_ratio: Some(1.),
                    padding: px(4.).into(),
                    ..default()
                },
                Button,
                Hovered::default(),
                children![Text("X".into())],
            ))
            .observe(|trigger: On<Pointer<Click>>, mut commands: Commands| {
                commands.entity(trigger.entity).trigger(|entity| CloseWindowEvent { entity });
            });
        }))
    )
}

pub fn minimize_button() -> impl Bundle
{
    (
        Node::default(),
        Children::spawn(SpawnWith(|parent: &mut ChildSpawner| {
            parent.spawn((
                Node {
                    flex_grow: 0.,
                    aspect_ratio: Some(1.),
                    padding: px(4.).into(),
                    ..default()
                },
                Button,
                Hovered::default(),
                children![Text("_".into())],
            ))
                .observe(|trigger: On<Pointer<Click>>, mut commands: Commands| {
                    commands.entity(trigger.entity).trigger(|entity| MinimizeWindowEvent { entity });
                });
        }))
    )
}

pub fn maximize_button() -> impl Bundle
{
    (
        Node::default(),
        Children::spawn(SpawnWith(|parent: &mut ChildSpawner| {
            parent.spawn((
                Node {
                    flex_grow: 0.,
                    aspect_ratio: Some(1.),
                    padding: px(4.).into(),
                    ..default()
                },
                Button,
                Hovered::default(),
                children![Text("O".into())],
            ))
                .observe(|trigger: On<Pointer<Click>>, mut commands: Commands| {
                    commands.entity(trigger.entity).trigger(|entity| MaximizeWindowEvent { entity });
                });
        }))
    )
}

pub fn floating_window_root(title: String, contents_bundle: impl Bundle) -> impl Bundle {
    (
        Node {
            flex_direction: FlexDirection::Column,
            min_height: Val::Px(30.),
            //overflow: Overflow { x: OverflowAxis::Hidden, y: OverflowAxis::Hidden },
            ..default()
        },
        FloatingWindow {
            initial_width: Val::Vw(40.),
            initial_height: Val::Vh(30.),
            min_width: Val::Px(200.),
            min_height: Val::Px(30.),
            ..default()
        },
        BackgroundColor::from(CRIMSON),
        Children::spawn( SpawnWith(|parent: &mut ChildSpawner| {
            parent.spawn((resizable_borders(5., BackgroundColor::from(DARK_GREEN)), WindowResize));
            parent.spawn(title_bar(title));
            parent.spawn((contents_bundle, Node {flex_grow: 1., flex_direction: FlexDirection::Column, overflow: Overflow { x: OverflowAxis::Hidden, y: OverflowAxis::Hidden }, ..default() }))
                .observe(block_pointer_events);
        })),
        bevy::ui_widgets::observe(|trigger: On<CloseWindowEvent>, mut commands: Commands| {
            commands.entity(trigger.entity).despawn();
        }),
        bevy::ui_widgets::observe(
            |trigger: On<MinimizeWindowEvent>,
            mut window_query: Query<&mut Node>,
            | {
                if let Ok(mut window_node) = window_query.get_mut(trigger.entity) {
                    window_node.height = Val::Px(30.);
                    window_node.min_height = Val::Px(30.);
                }
        }),
        bevy::ui_widgets::observe(
            |trigger: On<MaximizeWindowEvent>,
            mut window_query: Query<&mut Node>,
            | {
                if let Ok(mut window_node) = window_query.get_mut(trigger.entity) {
                    window_node.height = Val::Vh(100.);
                    window_node.width = Val::Vw(100.);
                    window_node.min_height = Val::Px(30.);
                    window_node.min_width = Val::Px(200.);
                }
            }
        )
    )
}

pub fn title_bar(title: String) -> impl Bundle {
    (
        Node {
            width: Val::Auto,
            height: Val::Px(30.),
            min_width: Val::Percent(100.),
            justify_content: JustifyContent::End,
            ..default()
        },
        WindowTitalBar,
        BackgroundColor::from(DARK_CYAN),
        Children::spawn(SpawnWith(|parent: &mut ChildSpawner| {
            parent.spawn(
                (
                    Node {
                        width: Val::Percent(100.),
                        justify_self: JustifySelf::Center,
                        justify_content: JustifyContent::Center,
                        align_self: AlignSelf::Center,
                        ..default()
                    },
                    Text(title)
                )
            );
            parent.spawn((minimize_button(), BackgroundColor::from(DARK_VIOLET)));
            parent.spawn((maximize_button(), BackgroundColor::from(DARK_VIOLET)));
            parent.spawn((close_button(), BackgroundColor::from(DARK_VIOLET)));
        }))
    )
}
