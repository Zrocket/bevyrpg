use bevy::{ecs::{lifecycle::HookContext, world::DeferredWorld}, prelude::*};

use crate::{add_to_inventory_observer, analyzer_ui::{display_analyzer_ui}, container_interaction_observer};

#[derive(Component, Reflect, Clone, PartialEq, Eq, Debug)]
#[reflect(Component)]
pub struct AnalyzerTimer(pub Timer);

fn update_analyzer_timer(
    mut timer_query: Query<&mut AnalyzerTimer>,
    time: Res<Time>,
) {
    if let Ok(mut timer) = timer_query.single_mut() {
        timer.0.tick(time.delta());
    }
}

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

#[derive(Component)]
pub struct ActiveSample(pub Entity);

#[derive(EntityEvent)]
pub struct AnalyzeSampleCancel(pub Entity);

#[derive(EntityEvent)]
pub struct AnalyzeSamplePause(pub Entity);

#[derive(EntityEvent)]
pub struct AnalyzeSampleEvent(pub Entity);

pub struct AnalyzerPlugin;
impl Plugin for AnalyzerPlugin {
    fn build(&self, app: &mut App) {
       app
           .add_systems(Update, update_analyzer_timer);
    }
}

fn analyze_sample_event_observer(
    trigger: On<AnalyzeSampleEvent>,
    mut commands: Commands,
    sample_query: Query<&mut crate::SampleItem>,
    analyzer_query: Query<Entity, With<Analyzer>>,
) {
    //if let Ok(sample) = sample_query.get(trigger.0)
    println!("ZZZZZZZZZ");
    if let Ok(analyzer) = analyzer_query.single() {
        println!("ASDFASDF");
        commands.entity(analyzer).insert(AnalyzerTimer(Timer::from_seconds(60., TimerMode::Once)));
    }
}
