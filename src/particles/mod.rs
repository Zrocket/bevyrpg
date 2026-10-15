use avian3d::prelude::*;
use bevy::{ecs::{lifecycle::HookContext, world::DeferredWorld}, prelude::*};

mod bubble_particles;
mod dust_particles;
mod fire_particles;
mod frost_particles;
mod rain_particles;
mod spark_particles;
mod steam_particles;
mod trash_particles;

use bubble_particles::*;
use dust_particles::*;
use fire_particles::*;
use frost_particles::*;
use rain_particles::*;
use spark_particles::*;
use steam_particles::*;
use trash_particles::*;

pub struct ParticlePlugin;
impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
       app
           .add_plugins((
                   BubbleParticlePlugin,
                   DustParticlePlugin,
                   FireParticlePlugin,
                   FrostParticlePlugin,
                   SparkParticlePlugin,
                   RainParticlePlugin,
                   SteamParticlePlugin,
                   TrashParticlePlugin,
           ));
    }
}
