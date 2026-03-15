use bevy::prelude::*;

use super::widgets;

use crate::{GameLoadingState};

#[derive(Component, Reflect)]
pub struct UiLoading;

pub struct LoadingMenuUiPlugin;
impl Plugin for LoadingMenuUiPlugin {
    fn build(&self, app: &mut bevy::app::App) {
       app
           .register_type::<UiLoading>()
           .add_systems(OnEnter(GameLoadingState::Loading), spawn_loading_menu);
    }
}

fn spawn_loading_menu(
    mut commands: Commands,
) {
    commands.spawn((
            widgets::ui_root("Loading Menu"),
            DespawnOnExit(GameLoadingState::Loading),
            GlobalZIndex(2),
            UiLoading,
            children![
                widgets::label("Loading"),
            ]
    ));
}
