use super::*;

#[derive(Component)]
pub struct StatusBarUi;

pub fn create_ui(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    player: &Character,
) -> Entity {
    commands
        .spawn(
            NodeBundle {
            visibility: Visibility::Visible,
            style: Style {
                flex_direction: FlexDirection::ColumnReverse,
                width: Val::Percent(100.),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(
                NodeBundle {
                background_color: BackgroundColor::from(Color::GREEN),
                visibility: Visibility::Visible,
                style: Style {
                    height: Val::Vh(10.),
                    width: Val::Percent(100.),
                    ..default()
                },
                ..default()
                }
            )
            .with_children(|parent| {
                create_health_ui(asset_server, player, parent);
            })
            .with_children(|parent| {
                create_mana_ui(asset_server, player, parent);
            });
        })
        .insert(UiEntity)
        .insert(StatusBarUi)
        .id()
}

fn create_health_ui(
    asset_server: &Res<AssetServer>,
    player: &Character,
    parent: &mut ChildBuilder,
) {
    let ui_icons: [Handle<Image>; 3] = [
        asset_server.load("HP/Style_1.png"),
        asset_server.load("HP/Style_2.png"),
        asset_server.load("HP/Style_3.png"),
    ];
        parent.spawn(
            TextBundle {
                text: Text::from_section(
                  "Player Health",
                  TextStyle {
                      font: asset_server.load("FiraSans-Bold.ttf"),
                      font_size: 50.0,
                      color: Color::WHITE,
                  }
                ),
                style: Style { 
                    width: Val::Percent(30.),
                    height: Val::Percent(100.),
                    ..default()
                },
                z_index: ZIndex::Global(10),
                ..default()
            })
            .with_children(|parent| {
                parent.spawn(
                        ImageBundle {
                            image: ui_icons[0].clone().into(),
                            visibility: Visibility::Visible,
                            style: Style {
                                width: Val::Percent((player.health as f32 / player.max_health as f32) * 100.),
                                height: Val::Percent(100.),
                                ..default()
                            },
                            z_index: ZIndex::Global(9),
                            ..default()
                        }
                    );
            });
}

fn create_mana_ui(
    asset_server: &Res<AssetServer>,
    player: &Character,
    parent: &mut ChildBuilder,
) {

    parent.spawn(
        TextBundle {
            text: Text::from_section(
              "Player Mana",
              TextStyle {
                  font: asset_server.load("FiraSans-Bold.ttf"),
                  font_size: 50.0,
                  color: Color::WHITE,
              }
            ),
            style: Style { 
                width: Val::Percent(30.),
                height: Val::Percent(100.),
                ..default()
            },
            z_index: ZIndex::Global(10),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(
                NodeBundle {
                visibility: Visibility::Visible,
                background_color: BackgroundColor::from(Color::BLUE),
                style: Style {
                    width: Val::Percent((player.mana as f32 / player.max_mana as f32) * 100.),
                    height: Val::Percent(100.),
                    ..default()
                },
                z_index: ZIndex::Global(9),
                ..default()
                }
            );
        });

}
