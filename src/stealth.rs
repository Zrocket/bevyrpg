use bevy::prelude::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Stealth(i32);
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct FeildOfView(f32);
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct LineOfSight;
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct CanSee(Vec<Entity>);

#[derive(Bundle)]
pub struct Sight {
    pub field_of_view: FeildOfView,
    pub line_of_sight: LineOfSight,
    pub can_see: CanSee,
}

pub struct StealthPlugin;

impl Plugin for StealthPlugin {
    fn build(&self, _app: &mut App) {}
}

fn _calc_line_of_sight(_commands: Commands, _query: Query<&LineOfSight>) {
    todo!()
}

fn _calc_can_see(_commands: Commands, _query: Query<(&FeildOfView, &LineOfSight, &CanSee)>) {
    todo!()
}
