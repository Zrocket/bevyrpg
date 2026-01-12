use bevy::prelude::*;
use super::*;

#[derive(Component, Reflect)]
pub struct UiCrosshair;

#[derive(Debug, Clone, Deref, Component, Reflect)]
pub struct Cooldown(Timer);

pub struct CrosshairPlugin;
impl Plugin for CrosshairPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Gameplay), draw_crosshair)
            .add_systems(Update, cooldown_tick)
            .add_observer(cooldown_observer);
    }
}

pub fn draw_crosshair(mut commands: Commands, asset_server: Res<AssetServer>) {
    trace!("draw_crosshair");
    let crosshair: Handle<Image> = asset_server.load("new_crosshairs/dot.png");
    commands
        .spawn((
            ImageNode {
                image: crosshair.clone(),
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
