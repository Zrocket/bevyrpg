use bevy::prelude::*;

use crate::*;

#[derive(Default, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct Health(pub i32);
#[derive(Default, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct Level(pub i32);
#[derive(Default, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct Mana(pub i32);
#[derive(Default, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct Experience(pub i32);
#[derive(Default, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct Speed(pub i32);
#[derive(Default, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct Corruption(pub i32);
#[derive(Default, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct Matter(pub i32);
#[derive(Default, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct Mind(pub i32);
#[derive(Default, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct Luck(pub i32);
#[derive(Default, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct Maneuver(pub i32);
#[derive(Default, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct MaxMana(pub i32);
#[derive(Default, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct MaxHealth(pub i32);

#[derive(Bundle)]
pub struct CharacterBundle {
    pub corruption: Corruption,
    pub experience: Experience,
    pub speed: Speed,
    pub health: Health,
    pub level: Level,
    pub luck: Luck,
    pub mana: Mana,
    pub maneuver: Maneuver,
    pub max_mana: MaxMana,
    pub max_health: MaxHealth,
    pub matter: Matter,
    pub mind: Mind,
    pub profession: Profession,
    pub skills: Skills,
}

impl Default for CharacterBundle {
    fn default() -> Self {
        CharacterBundle {
            corruption: Corruption(0),
            experience: Experience(100),
            speed: Speed(100),
            health: Health(100),
            level: Level(1),
            luck: Luck(100),
            mana: Mana(100),
            maneuver: Maneuver(100),
            max_mana: MaxMana(100),
            max_health: MaxHealth(100),
            matter: Matter(100),
            mind: Mind(100),
            profession: Profession::default(),
            skills: Skills::default(),
        }
    }
}

#[derive(Event)]
pub struct DamageEvent {
    pub target: Entity,
    pub ammount: i32,
}
#[derive(Event)]
pub struct LevelUpEvent(pub Entity);

#[derive(Event)]
pub struct ExperienceEvent {
    pub target: Entity,
    pub ammount: i32,
}
#[derive(Event)]
pub struct DeathEvent(pub Entity);

#[derive(Event)]
pub struct AttackEvent {
    pub attacker: Entity,
    pub defender: Entity,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub enum Profession {
    #[default]
    MagicUser,
    Martial,
    TreasureHunter,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Skills {
    pub guns: i32,
    pub blade: i32,
    pub heal: i32,
    pub armor: i32,
    pub lock_picking: i32,
    pub sneak: i32,
    pub technology: i32,
}

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DamageEvent>()
            .add_event::<LevelUpEvent>()
            .add_event::<ExperienceEvent>()
            .add_event::<AttackEvent>()
            .add_systems(
                Update,
                damage_event_handler.run_if(in_state(GameState::Gameplay)),
            )
            .add_systems(
                Update,
                experience_event_handler.run_if(in_state(GameState::Gameplay)),
            )
            .add_systems(
                Update,
                level_up_event_handler.run_if(in_state(GameState::Gameplay)),
            )
            .add_systems(
                Update,
                attack_event_handler.run_if(in_state(GameState::Gameplay)),
            )
            .register_type::<Health>()
            .register_type::<MaxHealth>()
            .register_type::<Level>()
            .register_type::<Luck>()
            .register_type::<Mana>()
            .register_type::<MaxMana>()
            .register_type::<Maneuver>()
            .register_type::<Matter>()
            .register_type::<Mind>()
            .register_type::<Speed>();
    }
}

fn attack_event_handler(
    mut attack_events: EventReader<AttackEvent>,
    mut damage_event_writer: EventWriter<DamageEvent>,
    actors: Query<AnyOf<(&Matter, &Maneuver)>>,
) {
    trace!("attack_event_handler");
    for event in attack_events.read() {
        if let Ok(attacker) = actors.get(event.attacker) {
            if let Ok(defender) = actors.get(event.defender) {
                if let Some(matter) = attacker.0 {
                    if let Some(maneuver) = defender.1 {
                        if matter.0 > maneuver.0 {
                            damage_event_writer.send(DamageEvent {
                                target: event.defender,
                                ammount: matter.0,
                            });
                        }
                    }
                }
            }
        }
    }
}

fn damage_event_handler(
    mut commands: Commands,
    mut health_query: Query<&mut Health>,
    mut damage_events: EventReader<DamageEvent>,
) {
    for event in damage_events.read() {
        if let Ok(mut health) = health_query.get_mut(event.target) {
            if event.ammount > health.0 {
                health.0 = 0;
                info!("TARGET IS DEAD!!!");
                commands.entity(event.target).despawn_recursive();
            } else {
                health.0 -= event.ammount;
            }
        }
    }
}

fn experience_event_handler(
    mut experience_query: Query<&mut Experience>,
    mut events: EventReader<ExperienceEvent>,
    mut level_up_writer: EventWriter<LevelUpEvent>,
) {
    for event in events.read() {
        if let Ok(mut experience) = experience_query.get_mut(event.target) {
            info!("Giving {} experience to {:?}", event.ammount, event.target);
            experience.0 += event.ammount;
            if experience.0 >= 100 {
                level_up_writer.send(LevelUpEvent(event.target));
            }
        }
    }
}

fn level_up_event_handler(
    mut level_query: Query<&mut Level>,
    mut level_up_events: EventReader<LevelUpEvent>,
) {
    for event in level_up_events.read() {
        if let Ok(mut level) = level_query.get_mut(event.0) {
            level.0 += 1;
            info!("Entity {:?} leveled up!", event.0);
        }
    }
}
