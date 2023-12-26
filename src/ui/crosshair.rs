use super::*;

#[derive(Component)]
pub struct Crosshair;

pub fn create_crosshair(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    ) -> Entity {
    let crosshair: Handle<Image> = asset_server.load("new_crosshairs/dot.png");
    commands
        .spawn(ImageBundle {
            image: crosshair.clone().into(),
            style: Style {
                position_type: PositionType::Absolute,
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                align_self: AlignSelf::Center,
                justify_self: JustifySelf::Center,
                left: Val::Vw(45.0),
                ..default()
            },
            ..default()
        })
    .insert(UiEntity)
    .insert(Crosshair)
    .id()
}
