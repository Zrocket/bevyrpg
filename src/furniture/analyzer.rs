use bevy::{ecs::{lifecycle::HookContext, world::DeferredWorld}, prelude::*};

use crate::{add_to_inventory_observer, analyzer_ui::{display_analyzer_ui}, container_interaction_observer};

#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(
    crate::Interactable,
    Name::new("Analyzer"),
)]
#[component(on_add = on_analyzer_add)]
pub struct Analyzer {
    pub sample: Option<Entity>,
    active: bool,
    progress: f32,
}

#[derive(Component)]
pub struct ActiveSample;

fn on_analyzer_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    world.commands()
        .entity(context.entity)
        .observe(add_to_inventory_observer::<Analyzer>)
        .observe(analyze_sample_event_observer)
        .observe(container_interaction_observer)
        //.observe(display_inventory_event_observer);
        .observe(display_analyzer_ui);
}

#[derive(EntityEvent)]
pub struct AnalyzeSampleCancel(pub Entity);

#[derive(EntityEvent)]
pub struct AnalyzeSamplePause(pub Entity);

#[derive(EntityEvent)]
pub struct AnalyzeSampleEvent(pub Entity);

pub struct AnalyzerPlugin;
impl Plugin for AnalyzerPlugin {
    fn build(&self, app: &mut App) {
       app;
    }
}

fn analyze_sample_event_observer(
    _trigger: On<AnalyzeSampleEvent>,
    sample_query: Query<&mut crate::SampleItem>,
) {
}
