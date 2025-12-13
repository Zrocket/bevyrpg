use std::default;

use bevy::{app::Plugin, ecs::{component::Component, observer::On, query::With, resource::Resource, system::{Query, Res, ResMut}}, picking::events::{Out, Over, Pointer}, time::Time};

/// Marks entity as tooltip source. When hovered.
///
/// It will count towards timers for deploying tooltip.
#[derive(Component)]
pub struct TooltipSource;

/// Global settings when tooltips should be displayed
#[derive(Resource)]
pub struct TooltipGlobalSettings {
    /// Delay in seconds until tolltip is shown when ui element with tooltip is hovered
    pub tooltip_delay: f32,

    /// When [`Self::tooltip_delay`] has elapsed. If pointer changes ui element with
    /// tooltip, show tooltip after pointer changed delay.
    ///
    /// Useful to avoid flickering when moving mouse.
    pub pointer_changed_delay: f32,

    /// If [`Self::tooltip_delay`] has elapsed. Tooltips will be shown immediately.
    /// Reset delay will reset tooltip delay, when no tooltip source is hovered for reset delay
    /// amount of seconds.
    pub reset_delay: f32,
}

/// Stores if tooltips should be shown
#[derive(Resource, Default)]
pub struct TooltipGlobalState {
    state: TooltipGlobalStateInner,
}

#[derive(Default)]
enum TooltipGlobalStateInner {
    #[default]
    Nothing,
    Waiting {
        since: f64,
    },
    PointerWaiting {
        since: f64,
    },
    Tooltip,
    Reset {
        since: f64,
    },
}

pub struct TooltipPlugin;
impl Plugin for TooltipPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app;
    }
}

impl TooltipGlobalState {
    pub fn show_tooltip(&self) -> bool {
        matches!(self.state, TooltipGlobalStateInner::Tooltip)
    }
}

fn on_source(
    pointer: On<Pointer<Over>>,
    query: Query<(), With<TooltipSource>>,
    mut res: ResMut<TooltipGlobalState>,
    time: Res<Time>,
) {
    if query.contains(pointer.entity) {
        match &res.state {
            TooltipGlobalStateInner::Nothing => {
                res.state = TooltipGlobalStateInner::Waiting {
                    since: time.elapsed_secs_f64(),
                };
            }
            TooltipGlobalStateInner::Waiting { since: _ }
            | TooltipGlobalStateInner::PointerWaiting { since: _ } => {}
            TooltipGlobalStateInner::Tooltip => {
                res.state = TooltipGlobalStateInner::Tooltip;
            }
            TooltipGlobalStateInner::Reset { since: _ } => {
                res.state = TooltipGlobalStateInner::PointerWaiting {
                    since: time.elapsed_secs_f64(),
                };
            }
        }
    }
}

fn out_source(
    pointer: On<Pointer<Out>>,
    query: Query<(), With<TooltipSource>>,
    mut res: ResMut<TooltipGlobalState>,
    time: Res<Time>,
) {
    if query.contains(pointer.entity) {
        res.state = match res.state {
            TooltipGlobalStateInner::Nothing | TooltipGlobalStateInner::Waiting { since: _ } => {
                TooltipGlobalStateInner::Nothing
            }
            TooltipGlobalStateInner::PointerWaiting { since: _ }
            | TooltipGlobalStateInner::Tooltip => TooltipGlobalStateInner::Reset {
                since: time.elapsed_secs_f64(),
            },
            TooltipGlobalStateInner::Reset { since } => TooltipGlobalStateInner::Reset { since },
        };
    }
}

fn update_tooltip_global_state(
    mut res: ResMut<TooltipGlobalState>,
    time: Res<Time>,
    global: Res<TooltipGlobalSettings>,
) {
    match &res.state {
        TooltipGlobalStateInner::Nothing => {}
        TooltipGlobalStateInner::Waiting { since } => {
            if (time.elapsed_secs_f64() - since) > global.tooltip_delay as f64 {
                res.state = TooltipGlobalStateInner::Tooltip;
            }
        }
        TooltipGlobalStateInner::PointerWaiting { since } => {
            if (time.elapsed_secs_f64() - since) > global.pointer_changed_delay as f64 {
                res.state = TooltipGlobalStateInner::Tooltip;
            }
        }
        TooltipGlobalStateInner::Tooltip => {}
        TooltipGlobalStateInner::Reset { since } => {
            if (time.elapsed_secs_f64() - since) > global.reset_delay as f64 {
                res.state = TooltipGlobalStateInner::Nothing;
            }
        }
    }
}
