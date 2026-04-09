use bevy::{ecs::{lifecycle::HookContext, world::DeferredWorld}, prelude::*};

use crate::*;

#[derive(Component, Default)]
pub struct GodMode;

#[derive(Default, Clone, Component, Reflect)]
#[reflect(Component)]
#[component(on_add = on_health_add)]
pub struct Health(pub i32);

#[derive(Default, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct MaxHealth(pub i32);

#[derive(Default, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct Mana(pub i32);

#[derive(Default, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct MaxMana(pub i32);

#[derive(Default, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct Experience(pub i32);

#[derive(Default, Clone, Component, Reflect)]
#[reflect(Component)]
#[component(on_add = on_level_add)]
pub struct Level(pub i32);

#[derive(Default, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct Hunger(pub i32);

#[derive(Default, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct Thirst(pub i32);

#[derive(Clone, Component, Reflect)]
#[reflect(Component)]
pub struct Sleep {
    pub value: i32,
    timer: Timer,
}

impl Default for Sleep {
    fn default() -> Self {
        Self {
            value: 100,
            timer: Timer::from_seconds(5.0, TimerMode::Repeating),
        }
    }
}

fn drain_sleep(
    mut sleep_query: Query<&mut Sleep>,
    time: Res<Time>,
) {
    for mut sleep in sleep_query.iter_mut() {
        sleep.timer.tick(time.delta());
        if sleep.timer.is_finished() {
            sleep.value -= 10;
        }
    }
}

#[derive(Bundle)]
pub struct CharacterBundle {
    pub experience: Experience,
    pub health: Health,
    pub level: Level,
    pub mana: Mana,
    pub max_mana: MaxMana,
    pub max_health: MaxHealth,
    pub ammo_pouch: AmmoPouch,
    pub hunger: Hunger,
    pub thirst: Thirst,
}

impl Default for CharacterBundle {
    fn default() -> Self {
        CharacterBundle {
            experience: Experience(0),
            health: Health(100),
            level: Level(1),
            mana: Mana(100),
            max_mana: MaxMana(100),
            max_health: MaxHealth(100),
            ammo_pouch: AmmoPouch(100),
            hunger: Hunger(0),
            thirst: Thirst(0),
        }
    }
}

#[derive(EntityEvent)]
pub struct DamageEvent {
    pub entity: Entity,
    pub ammount: i32,
}

#[derive(EntityEvent)]
pub struct ManaEvent {
    pub entity: Entity,
    pub ammount: i32,
}

#[derive(EntityEvent)]
pub struct HealEvent {
    pub entity: Entity,
    pub ammount: i32,
}

#[derive(EntityEvent)]
pub struct LevelUpEvent {
    pub entity: Entity,
}

#[derive(EntityEvent)]
pub struct ExperienceEvent {
    pub entity: Entity,
    pub ammout: i32,
}

#[derive(EntityEvent)]
pub struct DeathEvent{
    pub entity: Entity,
}

pub struct CharacterPlugin;
impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Health>()
            .register_type::<MaxHealth>()
            .register_type::<Mana>()
            .register_type::<MaxMana>()
            .register_type::<Experience>()
            .register_type::<Level>()
            .register_type::<Hunger>()
            .register_type::<Thirst>()
            .register_type::<Sleep>()
            .add_systems(Update, drain_sleep.run_if(in_state(GameState::Gameplay)));
    }
}

fn on_health_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    world.commands()
        .entity(context.entity)
        .observe(damage_observer)
        .observe(heal_observer)
        .observe(mana_event_observer);
}

fn on_level_add(
    mut world: DeferredWorld,
    context: HookContext,
) {
    world.commands()
        .entity(context.entity)
        .observe(experience_observer)
        .observe(level_up_observer);
}

fn damage_observer(
    trigger: On<DamageEvent>,
    mut commands: Commands,
    mut health_query: Query<&mut Health, Without<GodMode>>,
) {
    if let Ok(mut health) = health_query.get_mut(trigger.entity) {
        if health.0 <= trigger.ammount {
            health.0 = 0;
            info!("TARGET IS DEAD!!!");
            commands.entity(trigger.entity).trigger(|entity| DeathEvent { entity });
        } else {
            health.0 -= trigger.event().ammount;
        }
    }
}

fn mana_event_observer(
    trigger: On<ManaEvent>,
    mut mana_query: Query<&mut Mana>,
) {
    if let Ok(mut mana) = mana_query.get_mut(trigger.entity) {
        if mana.0 <= trigger.ammount {
            mana.0 = 0;
        } else {
            mana.0 -= trigger.event().ammount;
        }
    }
}

fn heal_observer(
    trigger: On<HealEvent>,
    mut health_query: Query<(&mut Health, &MaxHealth)>,
) {
    if let Ok((mut health, max_health)) = health_query.get_mut(trigger.entity) {
        health.0 += trigger.event().ammount;
        if health.0 >= max_health.0 {
            health.0 = max_health.0;
        }
    }
}

pub fn death_event_observer(
    trigger: On<DeathEvent>,
    mut commands: Commands,
) {
    commands.entity(trigger.entity).despawn();
}

fn experience_observer(
    trigger: On<ExperienceEvent>,
    mut commands: Commands,
    mut experience_query: Query<&mut Experience>,
) {
    if let Ok(mut experience) = experience_query.get_mut(trigger.entity) {
        info!("Giving {} experience to {:?}", trigger.event().ammout, trigger.entity);
        experience.0 += trigger.event().ammout;
        if experience.0 >= 100 {
            experience.0 -= 100;
            commands.entity(trigger.entity).trigger(|entity| LevelUpEvent { entity });
        }
    }
}

fn level_up_observer(
    trigger: On<LevelUpEvent>,
    mut level_query: Query<&mut Level>,
) {
    if let Ok(mut level) = level_query.get_mut(trigger.entity) {
        level.0 += 1;
        info!("Entity {:?} leveled up!", trigger.entity);
    }
}

fn sustinance_timer(
    query: Query<(&mut Hunger, &mut Thirst)>,
    time: Res<Time>
) {
}
