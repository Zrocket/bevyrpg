use super::*;

pub fn draw_crosshair(mut commands: Commands, asset_server: Res<AssetServer>) {
    trace!("draw_crosshair");
    debug!("Creating crosshair UiNode");
    debug!("Loading crossair asset");
    let crosshair: Handle<Image> = asset_server.load("new_crosshairs/dot.png");
    debug!("Crosshair asset loaded");
    debug!("Spawning UiNode");
    commands.spawn((ImageNode {
        image: crosshair.clone().into(),
        ..default()
    },));
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
        //.insert(UiEntity)
        .insert(UiCrosshair);
}
