use bevy::{ecs::{lifecycle::HookContext, world::DeferredWorld}, prelude::*};

use super::widgets;

use crate::MenuState;

#[derive(Component, Reflect)]
pub struct UiSaveGame;

#[derive(Component, Reflect)]
#[component(on_add = on_ui_save_list_item_add)]
pub struct UiSaveListItem;

fn on_ui_save_list_item_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    world.commands()
        .entity(context.entity)
        .insert(widgets::button("Save Game", save_game));
}

#[derive(Component, Reflect)]
#[require(
    Node {
        ..default()
    }
)]
pub struct UiSaveList;

fn on_save_list_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    world
        .commands()
        .entity(context.entity);
}

pub struct SaveGameMenuUiPlugin;
impl Plugin for SaveGameMenuUiPlugin {
    fn build(&self, app: &mut bevy::app::App) {
       app
           .register_type::<UiSaveGame>()
           .add_systems(OnEnter(MenuState::SaveGame), spawn_save_game_menu);
    }
}

fn spawn_save_game_menu(
    mut commands: Commands,
) {
    commands.spawn((
            widgets::ui_root("Save Game"),
            DespawnOnExit(MenuState::SaveGame),
            GlobalZIndex(2),
            UiSaveGame,
            children![
                widgets::button("Save Game", save_game),
            ]
    ));
}

fn exit_save_game_menu(
    _: On<Pointer<Click>>,
) {
}

fn save_game(
    _: On<Pointer<Click>>,
    mut commands: Commands,
    mut save_game_message_writer: MessageWriter<crate::SaveGameMessage>,
    mut change_level_message_writer: MessageWriter<crate::ChangeLevelMessage>,
    mut menu_state_setter: ResMut<NextState<crate::MenuState>>,
    start_menu_camera: Query<Entity, With<crate::StartMenuCamera>>,
) {
    menu_state_setter.set(crate::MenuState::Off);
    change_level_message_writer.write(crate::ChangeLevelMessage("levels/World.glb".into()));
    //save_game_message_writer.write(crate::SaveGameMessage);
    //commands.insert_resource(crate::PendingSaveLoad);
}

fn new_save(
    _: On<Pointer<Click>>,
) {
}
