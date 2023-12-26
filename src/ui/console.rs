use super::*;

pub fn create_console_ui(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) -> Entity {
    commands.spawn(
            NodeBundle {
                background_color: BackgroundColor::from(Color::BLACK),
                style: Style {
                    position_type: PositionType::Absolute,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Start,
                    width: Val::Percent(100.),
                    height: Val::Percent(50.),
                    ..default()
                },
                z_index: ZIndex::Global(20),
                ..default()
            })
    .with_children(|parent| {
        parent.spawn(
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    align_self: AlignSelf::Stretch,
                    overflow: Overflow::clip_y(),
                    ..default()
                },
                ..default()
            }
                    );
    })
    .insert(UiEntity)
    .id()
}
