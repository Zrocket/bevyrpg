use bevy::prelude::*;

use crate::PauseMenuState;

use super::widgets;

#[derive(Component, Reflect)]
pub struct UiCredits;

pub struct CreditsMenuUiPlugin;
impl Plugin for CreditsMenuUiPlugin {
    fn build(&self, app: &mut App) {
       app
           .register_type::<UiCredits>()
           .add_systems(OnEnter(PauseMenuState::Credits), spawn_credits_menu);
    }
}

fn spawn_credits_menu(
    mut commands: Commands,
) {
    println!("QQQQQQQQQQQQQQQQQQQQQQQQQQ");
    commands.spawn((
            widgets::ui_root("Credits Menu"),
            DespawnOnExit(PauseMenuState::Credits),
            GlobalZIndex(2),
            UiCredits,
            children![
                widgets::label("Red Pawn"),
            ]
    ));
}
