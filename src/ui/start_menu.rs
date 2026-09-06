use bevy::prelude::*;
use bevy_egui::PrimaryEguiContext;

use crate::{MenuState, MetaState, widgets};

#[derive(Component)]
#[require(
    Camera {
        ..default()
    },
    Camera3d::default(),
    //PrimaryEguiContext,
)]
pub struct StartMenuCamera;

#[derive(Component, Reflect)]
pub struct UiStartMenu;

pub struct StartMenuUiPlugin;
impl Plugin for StartMenuUiPlugin {
    fn build(&self, app: &mut App) {
       app
           .register_type::<UiStartMenu>()
           .add_systems(OnEnter(MetaState::MainMenu), spawn_start_menu_camera)
           .add_systems(OnEnter(MenuState::MainMenu), spawn_start_menu);
    }
}

fn spawn_start_menu_camera(
    mut commands: Commands,
) {
    commands.spawn((
        StartMenuCamera,
        DespawnOnExit(MetaState::MainMenu),
    ));
}

fn spawn_start_menu(
    mut commands: Commands,
) {
    commands.spawn((
        widgets::ui_root("Start Menu"),
        //DespawnOnExit(MetaState::MainMenu),
        DespawnOnExit(MenuState::MainMenu),
        GlobalZIndex(2),
        children![
            widgets::button("Start Game", start_game),
            widgets::button("Load Game", start_loading),
            widgets::button("Settings", start_settings),
            widgets::button("Credits", start_credits),
        ]
    ));
}

fn start_loading(
    _trigger: On<Pointer<Click>>,
    mut game_state: ResMut<NextState<MenuState>>,
) {
    game_state.set(MenuState::LoadGame);
}

fn start_credits(
    _trigger: On<Pointer<Click>>,
    mut game_state: ResMut<NextState<MenuState>>,
) {
    game_state.set(MenuState::Credits);
}

fn start_settings(
    _trigger: On<Pointer<Click>>,
    mut game_state: ResMut<NextState<MenuState>>,
) {
    game_state.set(MenuState::Settings);
}

fn start_game(
    _trigger: On<Pointer<Click>>,
    mut commands: Commands,
    start_menu_camera: Query<Entity, With<StartMenuCamera>>,
    mut game_state: ResMut<NextState<MetaState>>,
    mut change_level_message_writer: MessageWriter<crate::ChangeLevelMessage>,
) {
    //if let Ok(camera) = start_menu_camera.single() {
    //    commands.entity(camera).despawn();
    //}
    //game_state.set(MetaState::Gameplay);
    change_level_message_writer.write(crate::ChangeLevelMessage("levels/World.glb".into()));
}
