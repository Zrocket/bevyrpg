use bevy::prelude::*;
use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt};

pub struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<MetaState>()
            .register_type::<GameLoadingState>()
            .register_type::<BootStrap>()
            .register_type::<GameState>()
            .register_type::<UiState>()
            .register_type::<MenuState>()
            .init_state::<BootStrap>()
            .init_state::<MenuState>()
            .init_state::<UiState>()
            .add_sub_state::<GameState>()
            .add_sub_state::<MetaState>()
            .add_loading_state(
                LoadingState::new(BootStrap::Preload)
                    .continue_to_state(BootStrap::Loading)
                    .on_failure_continue_to_state(BootStrap::Loading)
            )
            .add_loading_state(
                LoadingState::new(BootStrap::Loading)
                    .continue_to_state(BootStrap::Postload)
                    .on_failure_continue_to_state(BootStrap::Postload)
            )
            .add_loading_state(
                LoadingState::new(BootStrap::Postload)
                    .continue_to_state(BootStrap::Finished)
                    .on_failure_continue_to_state(BootStrap::Finished)
            );
    }
}

#[derive(Clone, Hash, Debug, Eq, PartialEq, Default, States, Reflect)]
pub enum BootStrap {
    #[default]
    Preload,
    Loading,
    Postload,
    Finished,
}

#[derive(Clone, Hash, Debug, Eq, PartialEq, Default, SubStates, Reflect)]
#[source(BootStrap = BootStrap::Finished)]
pub enum MetaState {
    Splash,
    #[default]
    MainMenu,
    Gameplay,
}

#[derive(Clone, Hash, Debug, Eq, PartialEq, Default, States, Reflect)]
pub enum GameLoadingState {
    #[default]
    Preload,
    Loading,
    Postload,
    Finished,
}

#[derive(Clone, Hash, Debug, Eq, PartialEq, Default, SubStates, Reflect)]
#[source(MetaState = MetaState::Gameplay)]
pub enum GameState {
    #[default]
    Gameplay,
    Paused,
    GameOver,
    Loading
}

#[derive(Clone, Hash, Debug, Eq, PartialEq, Default, States, Reflect)]
pub enum UiState {
    #[default]
    Off,
    Inventory,
    Stats,
    QuestLog,
    Equiptment,
}

#[derive(Clone, Hash, Debug, Eq, PartialEq, Default, States, Reflect)]
pub enum MenuState {
    #[default]
    Off,
    Settings,
    ControllerSettings,
    Credits,
    GameplaySettings,
    SoundSettings,
    VideoSettings,
    MainMenu,
}
