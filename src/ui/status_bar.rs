use bevy::color::palettes::css::{BLUE, GREEN};

use super::*;

pub fn draw_status_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    health_query: Query<(&Health, &MaxHealth), With<Player>>,
    mana_query: Query<(&Mana, &MaxMana), With<Player>>,
) {
    trace!("draw_status_ui");
    let health_ui_icons: [Handle<Image>; 3] = [
        asset_server.load("HP/Style_1.png"),
        asset_server.load("HP/Style_2.png"),
        asset_server.load("HP/Style_3.png"),
    ];
    if let Ok((_mana, _max_mana)) = mana_query.single() {
        if let Ok((_health, _max_health)) = health_query.single() {
            let status_bar_node = commands
                .spawn((
                    Node {
                        width: Val::Percent(100.),
                        height: Val::Percent(10.),
                        justify_self: JustifySelf::Start,
                        ..default()
                    },
                    BackgroundColor(GREEN.into()),
                    Visibility::Visible,
                    UiStatus,
                ))
                .id();

            let player_health_node = commands
                .spawn((
                    Node { ..default() },
                    Button,
                    Text("Player Health".to_string()),
                    TextColor(Color::WHITE),
                    TextFont {
                        font: asset_server.load("FiraSans-Bold.ttf"),
                        font_size: 50.0,
                        ..default()
                    },
                    ZIndex(10),
                    ImageNode {
                        image: health_ui_icons[0].clone().into(),
                        ..default()
                    },
                ))
                .id();
            let player_mana_node = commands
                .spawn((
                    Node { ..default() },
                    Button,
                    Text("Player Mana".to_string()),
                    TextColor(Color::WHITE),
                    TextFont {
                        font: asset_server.load("FiraSans-Bold.ttf"),
                        font_size: 50.0,
                        ..default()
                    },
                    BackgroundColor::from(BLUE),
                    ZIndex(10),
                ))
                .id();

            commands.entity(status_bar_node).add_child(player_health_node);
            commands.entity(status_bar_node).add_child(player_mana_node);
        }
    }
}
