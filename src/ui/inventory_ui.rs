use bevy::{app::{HierarchyPropagatePlugin, Propagate}, color::palettes::css::{DARK_BLUE, DARK_GRAY, DARK_GREEN, DARK_KHAKI, DARK_RED}, ecs::{lifecycle::HookContext, system::SystemId, world::DeferredWorld}};

use crate::widgets::{anchored::{Anchor, AnchorDirection, AnchorOption, AnchorTarget, DropdownMenu}, floating_window_focus::{FocusDetectShouldClose, FocusParernt}, floating_window_ordering::UiZOrderLayer, floating_windows::floating_window_root, tooltip::{TooltipChild, TooltipParent, TooltipSource}};

use super::*;

#[derive(Component)]
#[relationship_target(relationship = ChildMenu, linked_spawn)]
pub struct ParentMenu(Vec<Entity>);

#[derive(Component)]
#[relationship(relationship_target = ParentMenu)]
pub struct ChildMenu(pub Entity);

#[derive(Component, Reflect)]
#[require(
    Node {
        flex_grow: 1.,
        //flex_direction: FlexDirection::Column,
        display: Display::Grid,
        //grid_auto_flow: GridAutoFlow::Column,
        grid_template_columns: vec![
            GridTrack::auto(),
            GridTrack::auto(),
            GridTrack::auto(),
            GridTrack::auto(),
            GridTrack::auto(),
        ],
        //aspect_ratio: Some(1.0),
        padding: UiRect::all(px(24)),
        row_gap: px(12),
        column_gap: px(12),
        overflow: Overflow { x: OverflowAxis::Hidden, y: OverflowAxis::Hidden },
        ..default()
    },
    BackgroundColor::from(DARK_GRAY),
)]
#[component(on_add = on_inventory_ui_add)]
pub struct UiInventory;

#[derive(Component, Reflect)]
#[require(
    Node {
        height: Val::Px(80.),
        width: Val::Px(80.),
        ..default()
    },
    BackgroundColor::from(DARK_GREEN),
    TooltipSource,
)]
#[component(on_add = on_inventory_item_ui_add)]
pub struct UiInventoryItem;

#[derive(EntityEvent)]
pub struct DisplayInventoryEvent {
    pub entity: Entity,
}

#[derive(Component)]
pub struct RightClickMenuItems(pub Vec<SystemId>);

#[derive(Component, Reflect, Debug, Clone, Copy, PartialEq, Eq)]
#[reflect(Component)]
pub struct Owner {
    item_owner: Entity,
    inv_owner: Entity,
}

#[derive(Component, Debug)]
pub struct InvRef(pub Entity);

fn on_inventory_ui_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    world.commands()
        .entity(context.entity)
        .observe(transfer_item_observer)
        .observe(refresh_window_observer);
}

fn on_inventory_item_ui_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    world.commands()
        .entity(context.entity)
        .observe(inventory_tooltip_observer)
        .observe(inventory_tooltip_unhover_observer)
        .observe(inventory_item_drowpdown_observer);
}

pub struct InventoryUIPlugin;
impl Plugin for InventoryUIPlugin {
    fn build(&self, app: &mut App) {
       app
           .add_plugins(HierarchyPropagatePlugin::<Owner, (), ChildMenu>::new(Update))
           .register_type::<Owner>();
    }
}

pub fn display_inventory_event_observer(
    trigger: On<DisplayInventoryEvent>,
    mut commands: Commands,
    name_query: Query<&Name>,
    item_query: Query<&ItemDetails>,
    inventory: Query<&Inventory>,
    menu_state: Res<State<UiState>>,
    mut menu_state_setter: ResMut<NextState<UiState>>,
) {
    trace!("OBSERVER: display_inventory_event_observer");

    if *menu_state == UiState::Inventory {
        return;
    }

    menu_state_setter.set(UiState::Inventory);

    let Ok(name) = name_query.get(trigger.entity) else {
        return;
    };
    let mut item_vec = vec![];

    if let Ok(inventory_handle) = inventory.get(trigger.entity) {
        for item in inventory_handle.iter() {
            if let Ok(item_name) = item_query.get(item) {
                trace!("Pushing item: {:?}, item_name: {:?}, to item_vec", item, item_name.name);
                item_vec.push((item_name.clone(), item.clone(), trigger.entity.clone()));
            }
        }
    }

    commands.spawn((
        DespawnOnExit(UiState::Inventory),
        floating_window_root(
            format!("{} Inventory", name),
        (
            UiInventory,
            Children::spawn(SpawnWith(|parent: &mut ChildSpawner| {
                for (item, entity, inv) in item_vec {
                    parent.spawn(
                (
                            UiInventoryItem,
                            Text(item.name),
                            Owner { item_owner: entity, inv_owner: inv },
                            Propagate( Owner { item_owner: entity, inv_owner: inv }),
                        ));
                }
            })),
            InvRef(trigger.entity),
        ),
        ),
    ));
}

#[derive(EntityEvent)]
#[entity_event(propagate, auto_propagate)]
pub struct RefreshInventory {
    pub entity: Entity,
}

fn refresh_window_observer(
    trigger: On<RefreshInventory>,
    mut commands: Commands,
    mut children_query: Query<(Entity, Option<&mut Children>, &mut InvRef)>,
    item_query: Query<&ItemDetails>,
    inventory: Query<&Inventory>,
) {
    trace!("OBSERVER: refresh_window_observer");
    if let Ok((parent_entity, children, invref)) = children_query.get_mut(trigger.entity) {
        trace!("Got children: {:?}, invref: {:?}", children, invref);

        if let Some(children) = children {
            for child in children.iter() {
                trace!("Despawning child: {:?}", child);
                commands.entity(child).despawn();
            }
        }

        let mut item_vec = vec![];

        if let Ok(inventory_handle) = inventory.get(invref.0) {
            for item in inventory_handle.iter() {
                if let Ok(item_name) = item_query.get(item) {
                    trace!("Pushing item: {:?}, item_name: {:?}, to item_vec", item, item_name.name);
                    item_vec.push((item_name.clone(), item.clone(), invref.0.clone()));
                }
            }
        }

        for (item, entity, inv) in item_vec {
            let child = commands.spawn((
                    UiInventoryItem,
                    Text(item.name),
                    Owner { item_owner: entity, inv_owner: inv },
                    Propagate( Owner { item_owner: entity, inv_owner: inv }),
                )).id();
            commands.entity(parent_entity)
                .add_child(child);
        }
    }
}

fn transfer_item_observer(
    trigger: On<Pointer<DragDrop>>,
    mut commands: Commands,
    owner_query: Query<&Owner>,
    invref_query: Query<&InvRef>,
    childof_query: Query<&ChildOf>,
) {
    trace!("OBSERVER: transfer_item_observer");
    if trigger.event().button == PointerButton::Secondary {
        return;
    }
    if let Ok(invref) = invref_query.get(trigger.entity)
    && let Ok(item) = owner_query.get(trigger.dropped)
    && let Ok(childof) = childof_query.get(trigger.dropped) {
        trace!("Removing item: {:?}, from inventory: {:?}", item.item_owner, item.inv_owner);
        commands.entity(item.inv_owner).trigger(|entity| RemoveFromInventoryEvent { entity, item: item.item_owner });
        trace!("Adding item: {:?}, to inventory: {:?}", item.item_owner, invref.0);
        commands.entity(invref.0).trigger(|entity| AddToInventoryEvent { entity, item: item.item_owner });
        trace!("Refreshing new inventory window: {:?}", trigger.entity);
        commands.entity(trigger.entity).trigger(|entity| RefreshInventory { entity });
        trace!("Refreshing old inventory window: {:?}", childof.0);
        commands.entity(childof.0).trigger(|entity| RefreshInventory { entity });
    }
}

fn inventory_tooltip_observer(
    trigger: On<Pointer<Over>>,
    mut commands: Commands,
    item_query: Query<&ItemDetails>,
    owner_query: Query<&Owner>,
) {
    trace!("OBSERVER: inventory_tooltip_observer");
    if let Ok(owner) = owner_query.get(trigger.entity)
    && let Ok(item) = item_query.get(owner.item_owner) {
        commands.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Px(100.),
                    height: Val::Px(100.),
                    ..default()
                },
                Text::new(item.description.0.to_string()),
                BackgroundColor::from(DARK_RED),
                AnchorTarget::Cursor,
                UiZOrderLayer::Tooltip,
                FocusParernt(trigger.entity),
                Propagate(Pickable {
                    should_block_lower: false,
                    is_hoverable: false,
                }),
                Pickable {
                    should_block_lower: false,
                    is_hoverable: false,
                },
                TooltipChild(trigger.entity),
        ));
    }
}

fn inventory_tooltip_unhover_observer(
    trigger: On<Pointer<Out>>,
    mut commands: Commands
) {
    trace!("OBSERVER: inventory_tooltip_unhover_observer");
    commands.entity(trigger.entity).despawn_related::<TooltipParent>();
}

fn inventory_item_drowpdown_observer(
    trigger: On<Pointer<Click>>,
    mut commands: Commands,
    dropdown_query: Query<Entity, With<DropdownMenu>>,
) {
    trace!("OBSERVER: inventory_item_drowpdown_observer");
    for entity in dropdown_query.iter() {
        commands.entity(entity).despawn();
    }
    if trigger.event().button == PointerButton::Secondary {
        commands.spawn((
                ChildMenu(trigger.entity),
                DropdownMenu,
                Node {
                    position_type: PositionType::Absolute,
                    flex_direction: FlexDirection::Column,
                    width: Val::Px(100.),
                    height: Val::Px(100.),
                    ..default()
                },
                UiZOrderLayer::Dropdown,
                AnchorTarget::Entity(trigger.entity),
                FocusParernt(trigger.entity),
                FocusDetectShouldClose,
                BackgroundColor::from(DARK_BLUE),
                AnchorOption {
                    anchor: AnchorDirection {
                        x: Anchor::Middle,
                        y: Anchor::Start,
                    },
                    target_anchor: AnchorDirection {
                        x: Anchor::Middle,
                        y: Anchor::End,
                    },
                    ..default()
                },
                Children::spawn(SpawnWith(|parent: &mut ChildSpawner| {
                    parent.spawn((
                            Node {
                                ..default()
                            },
                            Text::new("Drop"),
                            BackgroundColor::from(DARK_KHAKI),
                    ))
                    .observe(drop_item_button_observer);
                    parent.spawn((
                            Node {
                                ..default()
                            },
                            Text::new("Use"),
                            BackgroundColor::from(DARK_KHAKI),
                    ))
                    .observe(use_item_button_observer);
                }))
            ));
    }
}

#[allow(clippy::complexity)]
fn drop_item_button_observer(
    trigger: On<Pointer<Click>>,
    mut commands: Commands,
    parent_query: Query<&ChildOf>,
    childmenu_query:  Query<&ChildMenu>,
    owner_query: Query<&Owner>,
    inv_query: Query<Entity, With<Inventory>>,
    transform_query: Query<&Transform>,
    shelf_query: Query<&Shelf<Transform>>,
) {
    trace!("OBSERVER: drop_item_button_observer");
    if let Ok(parent) = parent_query.get(trigger.entity)
    && let Ok(owner) = owner_query.get(parent.0)
    && let Ok(actor) = inv_query.get(owner.inv_owner)
    && let Ok(actor_transform) = transform_query.get(owner.inv_owner)
    && let Ok(childmenu) = childmenu_query.get(parent.0)
    && let Ok(item_parent) = parent_query.get(owner.item_owner)
    && let Ok(parent_shelf) = shelf_query.get(item_parent.0)
    && let Ok(item_shelf) = shelf_query.get(owner.item_owner) {
        let mut parent_transform = *parent_shelf.0;
        parent_transform.translation = actor_transform.translation;
        commands.entity(item_parent.0)
            .insert(parent_transform);
        commands.entity(owner.item_owner)
            .insert(*item_shelf.0);
        commands.entity(actor).trigger(|entity| RemoveFromInventoryEvent { entity, item: owner.item_owner});
        commands.entity(childmenu.0).trigger(|entity| RefreshInventory { entity });
    }
}

fn use_item_button_observer(
    trigger: On<Pointer<Click>>,
    mut commands: Commands,
    parent_query: Query<&ChildOf>,
    owner_query: Query<&Owner>,
    inv_query: Query<Entity, With<Inventory>>,
) {
    trace!("OBSERVER: use_item_button_observer");
    if let Ok(parent) = parent_query.get(trigger.entity)
    && let Ok(owner) = owner_query.get(parent.0)
    && let Ok(actor) = inv_query.get(owner.inv_owner) {
        commands.entity(owner.item_owner).trigger(|entity| UseEvent { entity, actor });
    }
}
