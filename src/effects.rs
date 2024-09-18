use bevy::prelude::*;

#[derive(Default, Clone, Component, Reflect, StandardDist)]
#[reflect(Component)]
pub struct Poison(pub i32);
#[derive(Default, Clone, Component, Reflect, StandardDist)]
#[reflect(Component)]
pub struct Fire(pub i32);
#[derive(Default, Clone, Component, Reflect, StandardDist)]
#[reflect(Component)]
pub struct Frost(pub i32);

#[derive(Default, Clone, Component, Reflect, StandardDist)]
#[reflect(Component)]
pub struct IsPoisoned(pub i32);
#[derive(Default, Clone, Component, Reflect, StandardDist)]
#[reflect(Component)]
pub struct IsOnFire(pub i32);
#[derive(Default, Clone, Component, Reflect, StandardDist)]
#[reflect(Component)]
pub struct IsFrosted(pub i32);

#[derive(Default, Clone, Component, Reflect, StandardDist)]
#[reflect(Component)]
pub struct PoisonResistance(pub i32);
#[derive(Default, Clone, Component, Reflect, StandardDist)]
#[reflect(Component)]
pub struct FireResistance(pub i32);
#[derive(Default, Clone, Component, Reflect, StandardDist)]
#[reflect(Component)]
pub struct FrostResisance(pub i32);

pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
}

pub fn calc_effect<T: Component, U: Component>(
    query: Query<AnyOf<(T, U)>>,
    ) {
    todo!()
}
