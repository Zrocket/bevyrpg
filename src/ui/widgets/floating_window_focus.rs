use bevy::{app::Plugin, ecs::{component::Component, entity::{self, Entity}, event::EntityEvent, hierarchy::ChildOf, observer::On, query::With, system::{Commands, Query}}, picking::events::{Pointer, Press}, platform::collections::HashSet};

/// If contents of entity are interacted with, focus should be kept
/// for given focus parent entity and its parents recursively
///
/// Component should be attached to root entity
#[derive(Component)]
pub struct FocusParernt(pub Entity);

/// Add component to Ui tree root entity
///
/// This component will track if Ui should be closed
/// in situations where something else is pressed on screen.
///
/// [`FocusShouldClose`] will be inserted if UI should close
#[derive(Component)]
pub struct FocusDetectShouldClose;

/// Informs that something else was focused and view should close
#[derive(Component)]
pub struct FocusShouldClose;

/// Entity event signalizes that current floating anchored ui entity tree
/// hierarchy marked with [`FocusDetectShouldClose`] should be closed
#[derive(EntityEvent)]
pub struct FocusCloseCurrentTree {
    /// Source entity
    #[entity_event]
    pub entity: Entity,
}

pub struct FloatingUiFocusPlugin;
impl Plugin for FloatingUiFocusPlugin {
    fn build(&self, app: &mut bevy::app::App) {
       app
           .add_observer(update_should_close)
           .add_observer(should_close_current_tree_observer); 
    }
}

impl FocusCloseCurrentTree {
    /// Create entity event for entity which will cause
    /// focus close detection to trigger for all ancestor elements
    pub fn new(entity: Entity) -> Self {
        Self { entity }
    }
}

fn should_close_current_tree_observer(
    event: On<FocusCloseCurrentTree>,
    should_close: Query<(), With<FocusDetectShouldClose>>,
    focus_parents: Query<&FocusParernt>,
    child_of: Query<&ChildOf>,
    mut commands: Commands,
) {
    let mut current_entity = Some(event.entity);

    while let Some(entity) = current_entity.take() {
        let root_entity = child_of.root_ancestor(entity);

        if should_close.contains(root_entity) {
            commands.entity(root_entity).insert(FocusShouldClose);
        }

        if let Ok(focus_parent) = focus_parents.get(root_entity) {
            current_entity = Some(focus_parent.0);
        }
    }
}

fn update_should_close(
    pointer: On<Pointer<Press>>,
    should_close: Query<Entity, With<FocusDetectShouldClose>>,
    focus_parents: Query<&FocusParernt>,
    child_of: Query<&ChildOf>,
    mut commands: Commands,
) {
    if pointer.original_event_target() != pointer.entity {
        return;
    }

    // No elements to close
    if should_close.is_empty() {
        return;
    }

    let mut keep_open = HashSet::new();

    let mut current_entity = Some(pointer.entity);
    while let Some(entity) = current_entity.take() {
        let root_entity = child_of.root_ancestor(entity);

        if should_close.contains(root_entity) {
            keep_open.insert(root_entity);
        }

        if let Ok(focus_parent) = focus_parents.get(root_entity) {
            current_entity = Some(focus_parent.0);
        }
    }

    for entity in should_close.iter() {
        if keep_open.contains(&entity) {
            continue;
        }
        commands.entity(entity).insert(FocusShouldClose);
    }
}
