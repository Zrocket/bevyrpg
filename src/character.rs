use bevy::{prelude::*, reflect};

use crate::*;

pub struct DamageEvent {
    pub target: Entity,
    pub ammount: i64,
}
pub struct LevelUpEvent(pub Entity);
pub struct ExperienceEvent {
    pub target: Entity,
    pub ammount: i64,
}
pub struct DeathEvent(pub Entity);
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
    pub archery: usize,
    pub blade: usize,
    pub bludgeon: usize,
    pub harm: usize,
    pub heal: usize,
    pub heavy_armor: usize,
    pub light_armor: usize,
    pub lock_picking: usize,
    pub sneak: usize,
    pub technology: usize,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Character {
    pub speed: f32,
    pub mana: usize,
    pub max_mana: usize,
    pub health: i64,
    pub max_health: i64,
    pub level: usize,
    pub experience: usize,
    pub mind: usize,
    pub matter: usize,
    pub maneuver: usize,
    pub luck: usize,
    pub skills: Skills,
    pub profession: Profession,
    pub corruption: usize,
}

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Character>()
            .add_event::<DamageEvent>()
            .add_event::<LevelUpEvent>()
            .add_event::<ExperienceEvent>()
            .add_event::<AttackEvent>()
            .add_system(damage_character.in_set(OnUpdate(GameState::Gameplay)))
            .add_system(get_experience.in_set(OnUpdate(GameState::Gameplay)))
            .add_system(level_up.in_set(OnUpdate(GameState::Gameplay)))
            .add_system(attack_character.in_set(OnUpdate(GameState::Gameplay)));
    }
}

fn attack_character(
    mut attack_events: EventReader<AttackEvent>,
    mut damage_event_writer: EventWriter<DamageEvent>,
    charactes: Query<&mut Character>,
) {
    for event in attack_events.iter() {
        let attacker = charactes
            .get(event.attacker)
            .expect("Attacking a non-character entity!");
        let defender = charactes.get(event.defender).unwrap();
        if attacker.matter > defender.maneuver {
            damage_event_writer.send(DamageEvent {
                target: event.defender,
                ammount: attacker.matter as i64,
            });
        }
    }
}

fn damage_character(
    mut characters: Query<&mut Character>,
    mut damage_events: EventReader<DamageEvent>,
) {
    for event in damage_events.iter() {
        let mut target = characters
            .get_mut(event.target)
            .expect("Damaging a non-character entity!");
        target.health -= event.ammount;
    }
}

fn get_experience(
    mut characters: Query<&mut Character>,
    mut experience_events: EventReader<ExperienceEvent>,
    mut level_up_writer: EventWriter<LevelUpEvent>,
) {
    for event in experience_events.iter() {
        let mut character = characters
            .get_mut(event.target)
            .expect("Trying to give experience to a non-character entity!");
        character.experience += event.ammount as usize;
        if character.experience >= 100 {
            level_up_writer.send(LevelUpEvent(event.target));
        }
    }
}

fn level_up(mut characters: Query<&mut Character>, mut level_up_events: EventReader<LevelUpEvent>) {
    for event in level_up_events.iter() {
        let mut character = characters
            .get_mut(event.0)
            .expect("Trying to level up a non-character entity!");
        character.level += 1;
    }
}
