use bevy::prelude::*;

use crate::{MenuState, MetaState};

use super::widgets;

#[derive(Component, Reflect)]
pub struct UiCredits;

pub struct CreditsMenuUiPlugin;
impl Plugin for CreditsMenuUiPlugin {
    fn build(&self, app: &mut App) {
       app
           .register_type::<UiCredits>()
           .add_systems(OnEnter(MenuState::Credits), spawn_credits_menu);
    }
}

fn spawn_credits_menu(
    mut commands: Commands,
) {
    commands.spawn((
            widgets::ui_root("Credits Menu"),
            DespawnOnExit(MenuState::Credits),
            GlobalZIndex(2),
            UiCredits,
            children![
                widgets::label("Red Pawn"),
                widgets::button("Back", exit_credits_menu)
            ]
    ));
}

fn exit_credits_menu(
    _: On<Pointer<Click>>,
    meta_state: Res<State<MetaState>>,
    mut pause_menu_state: ResMut<NextState<MenuState>>,
) {
    match meta_state.get() {
        MetaState::Splash => {},
        MetaState::MainMenu => pause_menu_state.set(MenuState::Off),
        MetaState::Gameplay => pause_menu_state.set(MenuState::MainMenu),
    }
}
