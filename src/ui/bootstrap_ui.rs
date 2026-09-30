use bevy::{color::palettes::css::DARK_TURQUOISE, prelude::*};
use iyes_progress::ProgressTracker;

use crate::{BootStrap, widgets::progress_bar::ProgressBar};

#[derive(Component)]
#[require(
    Pickable {
        should_block_lower: true,
        ..default()
    },
)]
pub struct UiBootstrap;

#[derive(Component)]
#[require(
    Pickable {
        should_block_lower: true,
        ..default()
    },
    Text("PROGRESS".into()),
    BackgroundColor::from(DARK_TURQUOISE),
    ProgressBar {
        value: 0.,
        output: Val::Percent(100.),
    },
)]
pub struct UiBootstrapLoadingBar;

pub struct BootstrapUiPlugin;
impl Plugin for BootstrapUiPlugin {
    fn build(&self, app: &mut App) {
       app
           .add_systems(OnEnter(BootStrap::Preload), spawn_bootstrap_menu)
           .add_systems(Update, update_bar.run_if(in_state(BootStrap::Preload).or_else(in_state(BootStrap::Loading).or_else(in_state(BootStrap::Postload)))));
    }
}

fn spawn_bootstrap_menu(
    mut commands: Commands,
) {
    commands.spawn((
            crate::widgets::ui_root("BootStrap Screen"),
            DespawnOnExit(BootStrap::Postload),
            GlobalZIndex(5),
            UiBootstrap,
            children![
                crate::widgets::label("Loading"),
                UiBootstrapLoadingBar,
            ]
    ));
}

fn update_bar(
    mut commands: Commands,
    mut ui_query: Query<&mut ProgressBar, With<UiBootstrapLoadingBar>>,
    progress: Res<ProgressTracker<BootStrap>>,
) {
    let progress = progress.get_global_progress();
    if let Ok(mut ui) = ui_query.single_mut() {
        ui.value = progress.done as f32 / progress.total as f32;
    } else {
        error!("update_bar: Failed to query ProgressBar for UiBootstrapLoadingBar");
    }
}
