use bevy::prelude::*;

use crate::{GameState, Player, SpawnPlayerMessage};

use super::widgets;

#[derive(Component)]
pub struct UiGameOver;

pub struct GameOverUiPlugin;
impl Plugin for GameOverUiPlugin {
    fn build(&self, app: &mut App) {
       app
           .add_systems(OnEnter(GameState::GameOver), spawn_gameover_menu);
    }
}

fn spawn_gameover_menu(
    mut commands: Commands,
) {
    commands.spawn((
            widgets::ui_root("GameOver Menu"),
            DespawnOnExit(GameState::GameOver),
            GlobalZIndex(2),
            children![
                widgets::label("GameOver!"),
                widgets::button("Respawn", respawn)
            ]
    ));
}

fn respawn(
    _: On<Pointer<Click>>,
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
    mut spawn_player_message_writer: MessageWriter<SpawnPlayerMessage>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if let Ok(_player) = player_query.single() {
        game_state.set(GameState::Gameplay);
        spawn_player_message_writer.write(SpawnPlayerMessage);
    }
}
