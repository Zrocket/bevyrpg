use bevy::{color::palettes::css::CRIMSON, prelude::*};

use crate::UnInspectMessage;

#[derive(Component, Reflect)]
#[require(
    Node {
        position_type: PositionType::Absolute,
        width: Val::Percent(30.),
        height: Val::Percent(30.),
        left: Val::Percent(50.),
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Center,
        align_self: AlignSelf::Center,
        flex_wrap: FlexWrap::Wrap,
        ..default()
    },
    BackgroundColor(CRIMSON.into()),
)]
pub struct UiInspect;

pub struct InspectUiPlugin;
impl Plugin for InspectUiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, despawn_inspect_ui);
    }
}

pub fn draw_misc_inspect_ui(
    mut commands: Commands,
) {
    trace!("SYSTEM: draw_inspect_ui");
    commands.spawn((
            UiInspect,
    ));
}

pub fn despawn_inspect_ui(
    mut commands: Commands,
    mut uninspect_messages: MessageReader<UnInspectMessage>,
    ui_inspect_query: Query<Entity, With<UiInspect>>,
) {
    for _message in uninspect_messages.read() {
        for entity in ui_inspect_query.iter() {
            commands.entity(entity).despawn();
        }
    }
}
