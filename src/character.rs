use bevy::{prelude::*, reflect};

use crate::*;
use items::Item;

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
    pub archery: i32,
    pub blade: i32,
    pub bludgeon: i32,
    pub harm: i32,
    pub heal: i32,
    pub heavy_armor: i32,
    pub light_armor: i32,
    pub lock_picking: i32,
    pub sneak: i32,
    pub technology: i32,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Character {
    pub speed: f32,
    pub mana: i32,
    pub max_mana: i32,
    pub health: i32,
    pub max_health: i32,
    pub level: i32,
    pub experience: i32,
    pub mind: i32,
    pub matter: i32,
    pub maneuver: i32,
    pub luck: i32,
    pub skills: Skills,
    pub profession: Profession,
    pub corruption: i32,
}

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Character>()
            .add_event::<DamageEvent>()
            .add_event::<LevelUpEvent>()
            .add_event::<ExperienceEvent>()
            .add_event::<AttackEvent>()
            .add_systems(Update, damage_character.run_if(in_state(GameState::Gameplay)))
            .add_systems(Update, get_experience.run_if(in_state(GameState::Gameplay)))
            .add_systems(Update, level_up.run_if(in_state(GameState::Gameplay)))
            .add_systems(Update, attack_character.run_if(in_state(GameState::Gameplay)));
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
                ammount: attacker.matter as i32,
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
        if event.ammount > target.health {
            target.health = 0;
        } else {
            target.health -= event.ammount;
        }
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
        character.experience += event.ammount;
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
