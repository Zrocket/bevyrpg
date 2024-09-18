use super::*;

#[derive(Component)]
pub struct DialogBox;

pub fn create_dialog_ui(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    dialog: &YarnNode
    ) -> Entity {
    debug!("Creating dialog UiNode");
    commands
        .spawn(NodeBundle {
            background_color: BackgroundColor::from(Color::BLACK),
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Percent(80.),
                height: Val::Percent(80.),
                left: Val::Percent(10.),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_self: AlignSelf::Center,
                flex_wrap: FlexWrap::Wrap,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(NodeBundle {
                style: Style {
                    height: Val::Percent(100.),
                    flex_direction: FlexDirection::Column,
                    align_self: AlignSelf::Stretch,
                    overflow: Overflow::clip_y(),
                    ..default()
                },
                background_color: Color::CRIMSON.into(),
                ..default()
            })
            .with_children(|parent| {
                parent.spawn(
                    TextBundle {
                        text: Text::from_section(
                          &dialog.0,
                          TextStyle {
                              font: asset_server.load("FiraSans-Bold.ttf"),
                              font_size: 50.0,
                              color: Color::WHITE,
                          }
                        ),
                        style: Style {
                            ..default()
                        },
                        z_index: ZIndex::Global(10),
                        ..default()
                        }
                    );
                });
            }
        )
        //.insert(UiEntity)
        .insert(DialogBox)
        .id()
}
