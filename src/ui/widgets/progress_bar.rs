use std::ops::Mul;

use bevy::prelude::*;

#[derive(Component)]
pub struct ProgressBar {
    pub value: f32,
    pub output: Val,
}

pub struct ProgressBarWidgetPlugin;
impl Plugin for ProgressBarWidgetPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, update_progress_bars);
    }
}

fn update_progress_bars(
    mut widget_query: Query<(&ProgressBar, &mut Node)>,
) {
    for (bar, mut node) in widget_query.iter_mut() {
        node.width = bar.output.mul(bar.value);
    }
}
