use super::*;

pub fn draw_crosshair(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    ) {
    debug!("Creating crosshair UiNode");
    debug!("Loading crossair asset");
    let crosshair: Handle<Image> = asset_server.load("new_crosshairs/dot.png");
    debug!("Crosshair asset loaded");
    debug!("Spawning UiNode");
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
    //.insert(UiEntity)
    .insert(UiCrosshair);
}
