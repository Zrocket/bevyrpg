use bevy::color::palettes::css::{BLUE, GREEN};
use jonmo::prelude::*;

use super::*;

pub struct StatusUIPlugin;
impl Plugin for StatusUIPlugin {
    fn build(&self, app: &mut App) {
       app
           .add_systems(OnEnter(GameState::Gameplay), (jonmo_draw_status_ui));
    }
}

pub fn draw_status_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    ui_assets: Res<DA_UiAssets>,
    health_query: Query<(&Health, &MaxHealth), With<Player>>,
    mana_query: Query<(&Mana, &MaxMana), With<Player>>,
) {
    trace!("draw_status_ui");
    /*let health_ui_icons: [Handle<Image>; 3] = [
        asset_server.load("HP/Style_1.png"),
        asset_server.load("HP/Style_2.png"),
        asset_server.load("HP/Style_3.png"),
    ];*/
    /*let health_ui_icons: [Handle<Image>; 3] = [
        ui_assets.health_1.clone_weak(),
        ui_assets.health_2.clone_weak(),
        ui_assets.health_3.clone_weak(),
    ];*/
    if let Ok((_mana, _max_mana)) = mana_query.single()
        && let Ok((_health, _max_health)) = health_query.single() {
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
                        //image: health_ui_icons[0].clone().into(),
                        image: ui_assets.health_1.clone_weak().into(),
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

pub fn jonmo_draw_status_ui(
    world: &mut World,
    health_query: &mut QueryState<(&Health, &MaxHealth), With<Player>>,
    mana_query: &mut QueryState<(&Mana, &MaxMana), With<Player>>,
) {
    trace!("draw_status_ui");
    /*let health_ui_icons: [Handle<Image>; 3] = [
        asset_server.load("HP/Style_1.png"),
        asset_server.load("HP/Style_2.png"),
        asset_server.load("HP/Style_3.png"),
    ];*/
    /*let health_ui_icons: [Handle<Image>; 3] = [
        ui_assets.health_1.clone_weak(),
        ui_assets.health_2.clone_weak(),
        ui_assets.health_3.clone_weak(),
    ];*/
    let asset_server = world.resource::<AssetServer>();
    let ui_assets = world.resource::<DA_UiAssets>();
    if mana_query.single(world).is_ok()
        && health_query.single(world).is_ok() {
            let status_bar_node = JonmoBuilder::from((
                    Node {
                        width: Val::Percent(100.),
                        height: Val::Percent(10.),
                        justify_self: JustifySelf::Start,
                        ..default()
                    },
                    BackgroundColor(GREEN.into()),
                    Visibility::Visible,
                    UiStatus,
            ));

            let player_health_node = JonmoBuilder::from((
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
                        //image: health_ui_icons[0].clone().into(),
                        image: ui_assets.health_1.clone_weak().into(),
                        ..default()
                    },
            ));
            let player_mana_node = JonmoBuilder::from((
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
            ));

            status_bar_node.child(player_health_node)
                .child(player_mana_node)
                .spawn(world);
    }
}
