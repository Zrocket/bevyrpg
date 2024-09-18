use bevy::prelude::*;

#[derive(Component, Clone, Reflect)]
#[reflect(Component)]
pub struct BaseDmg(i32);
#[derive(Component, Clone, Reflect)]
#[reflect(Component)]
pub struct ManaCost(i32);
#[derive(Component, Clone, Reflect)]
#[reflect(Component)]
pub struct CastTime(i32);

#[derive(Component, Clone, Reflect)]
#[reflect(Component)]
pub enum Element {
    Fire,
    Water,
    Earth,
    Air,
    Electric,
    Aether,
    Void,
}

#[derive(Bundle)]
pub struct Spell {
    name: Name,
    element: Element,
    base_dmg: BaseDmg,
    mana_cost: ManaCost,
    cast_time: CastTime,
}

pub struct MagicPlugin;

impl Plugin for MagicPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<BaseDmg>()
            .register_type::<ManaCost>()
            .register_type::<CastTime>();
    }
}
