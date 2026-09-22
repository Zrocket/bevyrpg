use bevy::{color::palettes::css::BLUE, ecs::{lifecycle::HookContext, world::DeferredWorld}, prelude::*};

use crate::{Cartridge, ComputerCctvIcon, ComputerUiNode, PlugSocketEvent, furniture::computer::computer_input::{icon_drag_observer, icon_out, icon_over}, widgets::floating_windows::floating_computer_rover_window_root};

#[derive(Component)]
#[require(
    Node {
        position_type: PositionType::Absolute,
        width: Val::Percent(11.),
        height: Val::Percent(15.),
        align_items: AlignItems::Center,
        border_radius: BorderRadius::all(Val::Px(10.)),
        left: Val::Px(20.),
        top: Val::Px(200.),
        flex_direction: FlexDirection::Column,
        overflow: Overflow { x: OverflowAxis::Hidden, y: OverflowAxis::Hidden },
        ..default()
    },
    BackgroundColor(BLUE.into()),
    crate::IconClickTimer(Timer::from_seconds(1.0, TimerMode::Once)),
)]
#[component(on_add = on_computer_cart_icon_add)]
pub struct ComputerCartIcon(pub Entity);

fn on_computer_cart_icon_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    let Some(tmp) = world.get_mut::<ComputerCartIcon>(context.entity) else { return };
    let tmp = tmp.0;

    let Some(cart) = world.get_mut::<Cartridge>(tmp) else { return };
    let title = cart.title.clone();

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
        //Text::new("CART"),
        Text::new(title),
    )).id();

    world.commands()
        .entity(context.entity)
        .observe(icon_drag_observer)
        .observe(icon_over)
        .observe(icon_out)
        .observe(cart_icon_double_click_observer)
        .add_child(icon_node)
        .add_child(text_node);
}

pub(crate) fn cart_icon_double_click_observer(
    trigger: On<Pointer<Click>>,
    mut timer_query: Query<&mut crate::IconClickTimer>,
    mut commands: Commands,
    computer_ui_query: Query<Entity, With<ComputerUiNode>>,
) {
    if let Ok(mut timer) = timer_query.get_mut(trigger.entity)
    && let Ok(computer_ui) = computer_ui_query.single() {
        if timer.0.is_finished() {
            timer.0.reset();
        } else {
            let window = commands.spawn((
                    crate::ComputerNode,
                    floating_computer_rover_window_root("CART".to_string(), (
                        Node {
                            width: Val::Auto,
                            height: px(300),
                            border: UiRect::all(px(5)),
                            overflow: Overflow { x: OverflowAxis::Hidden, y: OverflowAxis::Hidden },
                            flex_direction: FlexDirection::ColumnReverse,
                            ..default()
                        },
                        BorderColor::all(Color::WHITE),
                        Children::spawn(SpawnWith(|root_parent: &mut ChildSpawner| {
                        })),
                    )),
            )).id();

            commands.entity(computer_ui).add_child(window);
        }
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
#[component(on_add = on_inserted_cart_add, on_remove = on_inserted_cart_remove)]
pub struct InsertedCart(pub Entity);

fn on_inserted_cart_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    let Some(tmp) = world.get_mut::<InsertedCart>(context.entity) else { return };
    let inserted_cart = tmp.0;

    let Some(mut computer_ui_query) = world.try_query_filtered::<Entity, With<ComputerUiNode>>() else { return };
    let mut query = world.query(&mut computer_ui_query);
    let Ok(computer_ui_entity) = query.single_mut() else { return };

    world.commands()
        .entity(computer_ui_entity)
        .with_child(ComputerCartIcon(inserted_cart));
}

fn on_inserted_cart_remove(
    mut world: DeferredWorld,
    context: HookContext,
) {
    let Some(mut computer_ui_query) = world.try_query_filtered::<Entity, With<ComputerUiNode>>() else { return };
    let mut query = world.query(&mut computer_ui_query);
    if let Ok(computer_ui_entity) = query.single_mut() {
        world.commands()
            .entity(computer_ui_entity)
            .remove::<ComputerCartIcon>();
    }
}

fn cart_slot_plug_socket_event_observer(
    trigger: On<PlugSocketEvent>,
    mut commands: Commands,
    cart_query: Query<&Cartridge>,
) {
    commands.entity(trigger.entity)
        .insert(InsertedCart(trigger.plug));
}

#[derive(Component, Reflect)]
#[reflect(Component)]
#[component(on_add = on_cart_slot_add)]
pub struct CartSlot;

fn on_cart_slot_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    println!("CARTSLOT ADDED");
    world.commands()
        .entity(context.entity)
        .observe(cart_slot_plug_socket_event_observer);
}

pub struct CartSlotPlugin;
impl Plugin for CartSlotPlugin {
    fn build(&self, app: &mut App) {
       app
           .register_type::<CartSlot>();
    }
}
