use bevy::{app::Propagate, color::palettes::css::{DARK_GREEN, DARK_RED}, ecs::{lifecycle::HookContext, world::DeferredWorld}, prelude::*};

use crate::{AddToInventoryEvent, ApplicatorAttachment, ApplicatorSubstance, AttachedToRover, Attachment, InvRef, Owner, RemoveFromInventoryEvent, Rover, RoverAttachment, SwitchRoveerAttachmentEvent, SwitchRoveerAttachmentEvent2, palette::{BRONZE, RUST_BROWN, VANILLA_CUSTARD}, widgets::floating_windows::floating_window_root};

#[derive(Component, Reflect)]
#[require(
    Node {
        flex_grow: 1.,
        flex_direction: FlexDirection::Row,
        overflow: Overflow { x: OverflowAxis::Hidden, y: OverflowAxis::Hidden },
        ..default()
    },
    BackgroundColor::from(Srgba::hex(BRONZE).unwrap()),
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
    BackgroundColor::from(Srgba::hex(RUST_BROWN).unwrap()),
)]
#[component(on_add = on_ui_rover_add)]
pub struct UiRover;

fn on_ui_rover_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    world.commands()
        .entity(context.entity)
        .observe(rover_on_attachment_drop);
}

fn rover_on_attachment_drop(
    trigger: On<Pointer<DragDrop>>,
    mut commands: Commands,
    attachment_query: Query<Entity, With<RoverAttachment>>,
    rover_query: Query<Entity, With<Rover>>,
    owner_query: Query<&Owner>,
) {
    let Ok(owner) = owner_query.get(trigger.dropped) else {
        error!("rover_on_attachment_drop: Failed to query Owner for {}", trigger.dropped);
        return;
    };
    let Ok(attachment) = attachment_query.get(owner.item_owner) else {
        error!("rover_on_attachment_drop: Dropped entity {} not a RoverAttachment", trigger.dropped);
        return;
    };
    let Ok(rover) = rover_query.single() else {
        error!("rover_on_attachment_drop: Failed to query Rover");
        return;
    };
    commands.entity(owner.inv_owner).trigger(|entity| RemoveFromInventoryEvent { entity, item: owner.item_owner });
    commands.entity(rover).trigger(|entity| SwitchRoveerAttachmentEvent2 { entity, attachment});
}

#[derive(Component, Reflect)]
#[require(
    Node {
        flex_direction: FlexDirection::Column,
        overflow: Overflow { x: OverflowAxis::Hidden, y: OverflowAxis::Hidden },
        ..default()
    },
    BackgroundColor::from(Srgba::hex(VANILLA_CUSTARD).unwrap()),
)]
#[component(on_add = on_ui_attachment_add)]
pub struct UiAttachment;

fn on_ui_attachment_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    world.commands()
        .entity(context.entity);
}

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

#[derive(Component)]
#[require(
    Node {
        align_self: AlignSelf::Center,
        padding: UiRect::all(px(24)),
        ..default()
    },
    //Text("SUBSTANCE".into()),
    BackgroundColor::from(DARK_GREEN),
)]
#[component(on_add = on_ui_substance_attachment)]
pub struct UiSubstanceAttachment;

fn on_ui_substance_attachment(
    mut world: DeferredWorld,
    context: HookContext,
) {
    world.commands()
        .entity(context.entity)
        .observe(switch_substance_attachment);
}

fn switch_substance_attachment(
    trigger: On<Pointer<DragDrop>>,
    mut commands: Commands,
    substance_query: Query<(Entity, &ApplicatorSubstance)>,
    mut applicator_attachment_query: Query<&mut ApplicatorAttachment>,
    rover_query: Query<Entity, With<Rover>>,
    owner_query: Query<&Owner>,
    invref_query: Query<&InvRef>,
) {
    if trigger.button == PointerButton::Secondary {
        return;
    }
    let Ok(mut applicator_attachment) = applicator_attachment_query.single_mut() else {
        error!("switch_substance_attachment: Failed to query ApplicatorAttachment");
        return;
    };
    let Ok(mut rover) = rover_query.single() else {
        error!("switch_substance_attachment: Failed to query Rover");
        return;
    };
    let Ok(owner) = owner_query.get(trigger.dropped) else {
        error!("switch_substance_attachment: Failed to query Owner for {}", trigger.dropped);
        return;
    };
    let Ok((substance_entity, substance)) = substance_query.get(owner.item_owner) else {
        error!("switch_substance_attachment: Not an ApplicatorSubstance: {}", trigger.dropped);
        return;
    };

    if applicator_attachment.0.is_some() {
        commands.entity(rover).trigger(|entity| AddToInventoryEvent { entity, item: applicator_attachment.0.unwrap()});
    }
    commands.entity(owner.inv_owner).trigger(|entity| RemoveFromInventoryEvent { entity, item: owner.item_owner});
    applicator_attachment.0 = Some(substance_entity);
}

pub struct RoverUiPlugin;
impl Plugin for RoverUiPlugin {
    fn build(&self, app: &mut App) {
       app
           .add_systems(Update, sync_rover_ui);
    }
}

#[allow(clippy::complexity)]
pub fn display_rover_ui(
    trigger: On<crate::DisplayInventoryEvent>,
    mut commands: Commands,
    name_query: Query<&Name>,
    item_query: Query<&crate::ItemDetails>,
    inventory: Query<&crate::Inventory>,
    attached_query: Query<(&Name, Option<&ApplicatorAttachment>), With<AttachedToRover>>,
) {
    let mut substance: Option<Entity> = None;
    let mut substance_name: String = "None".to_string();
    let Ok(_name) = name_query.get(trigger.entity) else {
        return;
    };
    let Ok((attachment_name, applicator_attachment)) = attached_query.single() else {
        return;
    };
    if applicator_attachment.is_some() {
        let applicator_attachment = applicator_attachment.unwrap();
        substance = applicator_attachment.0.clone();
        if substance.is_some() {
            if let Ok(tmp) = name_query.get(substance.unwrap()) {
                substance_name = tmp.clone().to_string();
            }
        }
    }
    let mut item_vec = vec![];

    if let Ok(inventory_handle) = inventory.get(trigger.entity) {
        for item in inventory_handle.iter() {
            if let Ok(item_name) = item_query.get(item) {
                trace!("Pushing item: {:?}, item_name: {:?}, to item_vec", item, item_name.name);
                item_vec.push((item_name.clone(), item.clone(), trigger.entity.clone()));
            }
        }
    }

    let inv_ref = trigger.entity.clone();
    let inv_ref2 = trigger.entity.clone();

    let attachment_name = attachment_name.to_string().clone();

    if applicator_attachment.is_some() {
        println!("AEIOUAEIOUAEOIU");
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
                                                            Text::new(attachment_name),
                                                            Children::spawn(SpawnWith(move |parent: &mut ChildSpawner| {
                                                                parent.spawn((
                                                                        UiSubstanceAttachment,
                                                                        Text::new(substance_name),
                                                                ));
                                                            })),
                                                    ));
                                                })),
                                        ));
                                    })),
                            ));
                        })),
                )),
        ));
    } else {
        println!("QWERTYQWERYQWERTY");
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
                                                            Text::new(attachment_name),
                                                    ));
                                                })),
                                        ));
                                    })),
                            ));
                        })),
                )),
        ));
    }
}

fn sync_rover_ui(
    changed_substance_query: Query<&ApplicatorAttachment, Changed<ApplicatorAttachment>>,
    mut ui_substance_text_query: Query<&mut Text, With<UiSubstanceAttachment>>,
    name_query: Query<&Name>,
) {
    let Ok(applicator_attachment) = changed_substance_query.single() else {
        trace!("sync_rover_ui: Failed to query changed ApplicatorAttachment");
        return;
    };
    let Ok(mut ui_substance_text) = ui_substance_text_query.single_mut() else {
        trace!("sync_rover_ui: Failed to query changed Text for UiSubstanceAttachment");
        return;
    };

    if applicator_attachment.0.is_some() {
        let tmp = applicator_attachment.0.unwrap();
        let Ok(name) = name_query.get(tmp) else {
            error!("sync_rover_ui: Failed to query Substance Name");
            return;
        };
        *ui_substance_text = Text::new(name.to_string());
    } else {
        *ui_substance_text = Text::new("None");
    }
}
