use std::marker::PhantomData;

use bevy::{color::palettes::css::DARK_GRAY, ecs::{component::Component, entity::Entity, event::EntityEvent, observer::On, query::With, system::{Commands, Query}}, prelude::Plugin, reflect::enum_debug, ui::BackgroundColor};

use crate::{Player, widgets::floating_windows::floating_window_root};


#[derive(EntityEvent)]
pub struct DisplayEquiptEvent {
    pub entity: Entity,
}

pub struct EquiptUiPlugin;
impl Plugin for EquiptUiPlugin {
    fn build(&self, app: &mut bevy::app::App) {
       app; 
    }
}

pub fn spawn_equipt_ui(
    mut commands: Commands,
    entity: Query<Entity, With<Player>>,
) {
    if let Ok(entity) = entity.single() {
        commands.entity(entity).trigger(|entity| DisplayEquiptEvent { entity });
    }
}

pub fn display_equipt_event_observer(
    trigger: On<DisplayEquiptEvent>,
    mut commands: Commands,
) {
    commands.spawn((
            floating_window_root("EQUIPT TEST".to_string(),
                (
                )),
    ));
}
