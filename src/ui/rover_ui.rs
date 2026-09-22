use bevy::{app::Propagate, color::palettes::css::{BLUE, DARK_KHAKI, DARK_RED, DARK_SLATE_GRAY, DARK_TURQUOISE, DARK_VIOLET, LIGHT_PINK, PURPLE, SADDLE_BROWN}, ecs::{lifecycle::HookContext, world::DeferredWorld}, prelude::*};

use crate::{AttachedToRover, widgets::floating_windows::floating_window_root};

#[derive(Component, Reflect)]
#[require(
    Node {
        flex_grow: 1.,
        flex_direction: FlexDirection::Row,
        overflow: Overflow { x: OverflowAxis::Hidden, y: OverflowAxis::Hidden },
        ..default()
    },
    BackgroundColor::from(DARK_VIOLET),
)]
#[component(on_add = on_ui_rover_root_add)]
pub struct UiRoverRoot;

fn on_ui_rover_root_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    world.commands()
        .entity(context.entity);
}

#[derive(Component, Reflect)]
#[require(
    Node {
        flex_grow: 1.,
        flex_direction: FlexDirection::Column,
        overflow: Overflow { x: OverflowAxis::Hidden, y: OverflowAxis::Hidden },
        ..default()
    },
    BackgroundColor::from(DARK_SLATE_GRAY),
)]
#[component(on_add = on_ui_rover_add)]
pub struct UiRover;

fn on_ui_rover_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    world.commands()
        .entity(context.entity);
}

#[derive(Component, Reflect)]
#[require(
    Node {
        flex_direction: FlexDirection::Column,
        overflow: Overflow { x: OverflowAxis::Hidden, y: OverflowAxis::Hidden },
        ..default()
    },
    BackgroundColor::from(BLUE),
)]
#[component(on_add = on_ui_attachment_add)]
pub struct UiAttachment;

#[derive(Component, Reflect)]
#[require(
    Node {
        align_self: AlignSelf::Center,
        padding: UiRect::all(px(24)),
        ..default()
    },
    Text("ACTIVE".into()),
    BackgroundColor::from(DARK_RED),
)]
#[component(on_add = on_ui_attachment_icon_add)]
pub struct UiAttacmentIcon;

fn on_ui_attachment_icon_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    world.commands()
        .entity(context.entity);
}

fn on_ui_attachment_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    world.commands()
        .entity(context.entity);
}

pub struct RoverUiPlugin;
impl Plugin for RoverUiPlugin {
    fn build(&self, app: &mut App) {
       app;
    }
}

#[allow(clippy::complexity)]
pub fn display_rover_ui(
    trigger: On<crate::DisplayInventoryEvent>,
    mut commands: Commands,
    name_query: Query<&Name>,
    item_query: Query<&crate::ItemDetails>,
    inventory: Query<&crate::Inventory>,
    menu_state: Res<State<crate::UiState>>,
    mut menu_state_setter: ResMut<NextState<crate::UiState>>,
    attached_query: Query<&Name, With<AttachedToRover>>,
) {
    let Ok(name) = name_query.get(trigger.entity) else {
        return;
    };
    let Ok(attachment_name) = attached_query.single() else {
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

    let mut active_sample = String::from("ACTIVESAMPLE");

    let inv_ref = trigger.entity.clone();
    let inv_ref2 = trigger.entity.clone();

    let tmp = attachment_name.to_string().clone();

    commands.spawn((
            floating_window_root("Rover".into(), (
                    UiRoverRoot,
                    Children::spawn(SpawnWith(move |parent: &mut ChildSpawner| {
                        parent.spawn((
                                crate::UiInventory,
                                Children::spawn(SpawnWith(|parent: &mut ChildSpawner| {
                                    for (item, entity, inv) in item_vec {
                                        parent.spawn((
                                                crate::UiInventoryItem,
                                                Text(item.name),
                                                crate::Owner { item_owner: entity, inv_owner: inv },
                                                Propagate( crate::Owner { item_owner: entity, inv_owner: inv }),
                                        ));
                                    }
                                })),
                                crate::InvRef(inv_ref.clone()),
                        ));
                        parent.spawn((
                                UiRover,
                                Children::spawn(SpawnWith(move |parent: &mut ChildSpawner| {
                                    parent.spawn((
                                            UiAttachment,
                                            Children::spawn(SpawnWith(|parent: &mut ChildSpawner| {
                                                parent.spawn((
                                                        UiAttacmentIcon,
                                                        Text::new(tmp),
                                                ));
                                            })),
                                    ));
                                })),
                        ));
                    })),
            )),
    ));
}
