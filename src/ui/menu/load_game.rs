use std::process::id;

use bevy::{ecs::{lifecycle::HookContext, world::DeferredWorld}, prelude::*};

use super::widgets;

use crate::{MenuState, MetaState, SCENE_FILE_PATH, level::ChangeLevelMessage};

#[derive(Component, Reflect)]
pub struct UiLoadGame;

#[derive(Component, Reflect)]
#[require(
    Node {
        ..default()
    },
)]
#[component(on_add = on_ui_load_slots_add)]
pub struct UiLoadSlots;

#[derive(Component, Reflect)]
pub struct SaveRef(pub Entity);

fn on_ui_load_slots_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    let Some(mut saves_query) = world.try_query::<(&crate::SaveFile)>() else { return };

    let mut save_names: Vec<String> = Vec::new();

    for save_file in saves_query.iter(&world) {
        save_names.push(save_file.0.clone());
    }
    for file in save_names.iter() {
        let bundle = world.commands().spawn(widgets::button(file, load_game)).id();
        world.commands()
            .entity(context.entity)
            .add_child(bundle);
    }

}

pub struct LoadGameMenuUiPlugin;
impl Plugin for LoadGameMenuUiPlugin {
    fn build(&self, app: &mut bevy::app::App) {
       app
           .register_type::<UiLoadGame>()
           .register_type::<SaveRef>()
           .add_systems(OnEnter(MenuState::LoadGame), spawn_load_game_menu);
    }
}

fn spawn_load_game_menu(
    mut commands: Commands,
    saves_query: Query<&crate::SaveFile>,
    saves_storage: Res<crate::SaveStorage>,
) {
    let mut save_names: Vec<String> = Vec::new();

    for save_file in saves_query.iter() {
        save_names.push(save_file.0.clone());
    }

    let mut menu = commands.spawn((
            widgets::ui_root("Load Game"),
            DespawnOnExit(MenuState::LoadGame),
            GlobalZIndex(2),
            UiLoadGame,
            children![
                widgets::button("Load Game", load_game),
                //UiLoadSlots,
                widgets::button("Back", exit_load_game_menu),
            ]
    )).id();

    for (save_index, save_entity) in saves_storage.0.iter().enumerate() {
        println!("{:?}", save_entity);
        let bundle = commands.spawn(
            widgets::button_with_component(
                format!("save: {save_index}"),
                load_game,
            SaveRef(*save_entity),
        )).id();
        commands.entity(menu)
            .add_child(bundle);
    }

    /*for file in save_names.iter() {
        let bundle = commands.spawn(widgets::button(file, load_game))
            .insert(SaveRef()).id();
        commands.entity(menu)
            .add_child(bundle);
    }*/
}

fn exit_load_game_menu(
    _: On<Pointer<Click>>,
    mut pause_menu_state: ResMut<NextState<MenuState>>,
    mut meta_state: Res<State<MetaState>>,
) {
    if meta_state.get() == &MetaState::MainMenu {
        pause_menu_state.set(MenuState::MainMenu);
    } else {
        pause_menu_state.set(MenuState::Settings);
    }
}

fn load_game(
    trigger: On<Pointer<Click>>,
    mut commands: Commands,
    save_ref_queury: Query<&SaveRef>,
    mut load_game_message_writer: MessageWriter<crate::LoadGameMessage>,
    mut pause_menu_state: ResMut<NextState<MenuState>>,
    mut change_level_message_writer: MessageWriter<crate::ChangeLevelMessage>,
) {
    if let Ok(save_ref) = save_ref_queury.get(trigger.entity) {
        change_level_message_writer.write(crate::ChangeLevelMessage("levels/World.glb".into()));
        commands.insert_resource(crate::PendingSaveLoad(save_ref.0));
    }
}
