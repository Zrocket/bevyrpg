use bevy::{color::palettes::css::DARK_GREEN, ecs::{entity::Entity, event::EntityEvent, hierarchy::{ChildSpawner, Children}, name::Name, observer::On, query::With, relationship::RelationshipTarget, spawn::{SpawnRelated, SpawnWith}, system::{Commands, Query}}, prelude::{Node, Plugin, Text}, ui::{BackgroundColor, Overflow}, utils::default};

use crate::{Equiptment, Player, widgets::floating_windows::floating_window_root};


#[derive(EntityEvent)]
pub struct DisplayEquipEvent {
    pub entity: Entity,
}

pub struct EquipUiPlugin;
impl Plugin for EquipUiPlugin {
    fn build(&self, app: &mut bevy::app::App) {
       app;
    }
}

pub fn spawn_equip_ui(
    mut commands: Commands,
    entity: Query<Entity, With<Player>>,
) {
    if let Ok(entity) = entity.single() {
        commands.entity(entity).trigger(|entity| DisplayEquipEvent { entity });
    }
}

pub fn display_equip_event_observer(
    trigger: On<DisplayEquipEvent>,
    mut commands: Commands,
    equiptment_query: Query<&Equiptment>,
    name_query: Query<&Name>,
) {
    let mut equip_vec = vec![];
    if let Ok(equiptment) = equiptment_query.get(trigger.entity) {
        for item in equiptment.iter() {
            if let Ok(item_name) = name_query.get(item) {
                equip_vec.push((item_name.clone(), item.clone()));
            }
        }
    }
    commands.spawn((
            floating_window_root("Equiptment".to_string(),
                (
                    Node {
                        flex_grow: 1.,
                        flex_direction: bevy::ui::FlexDirection::Column,
                        overflow: Overflow { x: bevy::ui::OverflowAxis::Hidden, y: bevy::ui::OverflowAxis::Hidden },
                        ..default()
                    },
                    Children::spawn(SpawnWith(|parent: &mut ChildSpawner| {
                        for (name, entity) in equip_vec {
                            parent.spawn((
                                    Node {
                                        ..default()
                                    },
                                    Text(name.to_string()),
                                    BackgroundColor::from(DARK_GREEN),
                            ));
                        }
                    })),
                ),
                ),
    ));
}
