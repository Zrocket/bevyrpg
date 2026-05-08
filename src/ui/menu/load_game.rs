use bevy::prelude::*;

use super::widgets;

use crate::{MenuState, SCENE_FILE_PATH, level::ChangeLevelMessage};

#[derive(Component, Reflect)]
pub struct UiLoadGame;

pub struct LoadGameMenuUiPlugin;
impl Plugin for LoadGameMenuUiPlugin {
    fn build(&self, app: &mut bevy::app::App) {
       app
           .register_type::<UiLoadGame>()
           .add_systems(OnEnter(MenuState::LoadGame), spawn_load_game_menu);
    }
}

fn spawn_load_game_menu(
    mut commands: Commands,
) {
    commands.spawn((
            widgets::ui_root("Load Game"),
            DespawnOnExit(MenuState::LoadGame),
            GlobalZIndex(2),
            UiLoadGame,
            children![
                widgets::button("Load Save", load_save),
            ]
    ));
}

fn exit_load_game_menu(
    _: On<Pointer<Click>>,
) {
}

fn load_save(
    _: On<Pointer<Click>>,
    mut commands: Commands,
    mut load_game_message_writer: MessageWriter<crate::LoadGameMessage>,
    mut change_level_message_writer: MessageWriter<crate::ChangeLevelMessage>,
    mut menu_state_setter: ResMut<NextState<crate::MenuState>>,
    start_menu_camera: Query<Entity, With<crate::StartMenuCamera>>,
) {
    menu_state_setter.set(crate::MenuState::Off);
    change_level_message_writer.write(crate::ChangeLevelMessage("levels/World.glb".into()));
    //load_game_message_writer.write(crate::LoadGameMessage);
    commands.insert_resource(crate::PendingSaveLoad);
}
