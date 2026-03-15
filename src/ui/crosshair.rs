use bevy::{ecs::{lifecycle::HookContext, world::DeferredWorld}, prelude::*, window::WindowResized};
use super::*;

#[derive(Component, Reflect)]
#[require(
    Node {
        position_type: PositionType::Absolute,
        display: Display::Flex,
        flex_direction: FlexDirection::Column,
        align_self: AlignSelf::Center,
        justify_self: JustifySelf::Center,
        left: Val::Vw(45.0),
        ..default()
    },
)]
#[component(on_add = on_crosshair_add)]
pub struct UiCrosshair;

#[derive(Debug, Clone, Deref, Component, Reflect)]
pub struct Cooldown(Timer);

fn on_crosshair_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    let asset_server = world.resource::<AssetServer>();

    let image: Handle<Image> = asset_server.load("new_crosshairs/dot.png");

    world.commands()
        .entity(context.entity)
        .insert(ImageNode {
            image,
            ..default()
        });
}

pub struct CrosshairPlugin;
impl Plugin for CrosshairPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Gameplay), draw_crosshair)
            .add_systems(Update, cooldown_tick)
            .add_systems(Update, on_window_resize)
            .add_observer(cooldown_observer);
    }
}

pub fn draw_crosshair(mut commands: Commands) {
    trace!("draw_crosshair");
    commands
        .spawn((
            UiCrosshair,
        ));
}

pub fn cooldown_observer(
    _trigger: On<ShootEvent>,
    _time: Res<Time>,
    mut cooldown_query: Query<&mut Cooldown>,
) {
    if let Ok(mut cooldown) = cooldown_query.single_mut() {
        cooldown.0.reset();
    }
}

pub fn cooldown_tick(
    mut cooldown_query: Query<&mut Cooldown>,
    time: Res<Time>,
) {
    if let Ok(mut cooldown) = cooldown_query.single_mut() {
        cooldown.0.tick(time.delta());
    }
}

fn on_window_resize(
    mut commands: Commands,
    crosshair_query: Query<Entity, With<UiCrosshair>>,
    resize_reader: MessageReader<WindowResized>,
) {
    if let Ok(crosshair_entity) = crosshair_query.single()
    && !resize_reader.is_empty() {
        commands.entity(crosshair_entity).despawn();
        commands.spawn(UiCrosshair);
    }
}
