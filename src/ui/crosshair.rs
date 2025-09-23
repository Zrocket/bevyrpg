use bevy::prelude::*;
use super::*;

#[derive(Debug, Clone, Deref, Component, Reflect)]
pub struct Cooldown(Timer);

pub struct CrosshairPlugin;
impl Plugin for CrosshairPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Gameplay), (
                jonmo_draw_crosshair,
            ))
            .add_systems(Update, cooldown_tick)
            .add_observer(cooldown_observer);
    }
}

pub fn _draw_crosshair(mut commands: Commands, asset_server: Res<AssetServer>) {
    trace!("draw_crosshair");
    let crosshair: Handle<Image> = asset_server.load("new_crosshairs/dot.png");
    /*commands.spawn((ImageNode {
        image: crosshair.clone().into(),
        ..default()
    },));*/
    commands
        .spawn((
            ImageNode {
                image: crosshair.clone().into(),
                ..default()
            },
            Node {
                position_type: PositionType::Absolute,
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                align_self: AlignSelf::Center,
                justify_self: JustifySelf::Center,
                left: Val::Vw(45.0),
                ..default()
            },
        ))
        .insert(UiCrosshair);
}

pub fn jonmo_draw_crosshair(
    world: &mut World,
) {
    let asset_server = world.resource::<AssetServer>();
    trace!("draw_crosshair");
    let crosshair: Handle<Image> = asset_server.load("new_crosshairs/dot.png");
    let cooldown_timer = LazyEntity::new();
    JonmoBuilder::from((
        Node {
            position_type: PositionType::Absolute,
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            align_self: AlignSelf::Center,
            justify_self: JustifySelf::Center,
            ..default()
        },
    ))
    .insert(Cooldown(Timer::from_seconds(1.0, TimerMode::Once)))
    .entity_sync(cooldown_timer.clone())
    .child(
        JonmoBuilder::from((
            Node {
                align_self: AlignSelf::Center,
                ..default()
            },
            UiCrosshair,
        ))
        .component_signal(
            SignalBuilder::from_component_lazy(cooldown_timer.clone())
            .map_in(|cooldown: Cooldown| cooldown.0.remaining_secs())
            .dedupe()
            .map_in(move |percent: f32| {
                ImageNode {
                    image: crosshair.clone(),
                    color: Color::Srgba(Srgba { red: 1. - percent, green: 1. - percent, blue: 1. - percent, alpha: 1. }),
                    ..default()
                }
            })
            .map_in(Some)
        )
    )
    .spawn(world);
}

pub fn cooldown_observer(
    trigger: Trigger<ShootEvent>,
    time: Res<Time>,
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
