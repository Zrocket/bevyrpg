use bevy::prelude::*;
use bevy_enhanced_input::{action::InputAction, prelude::{Fire, Start}};
use bevy_seedling::{SeedlingPlugin, pool::SamplerPool, prelude::PoolLabel, sample::SamplePlayer};
use rand::Rng;

use crate::{BackwardAction, ForwardAction, JumpAction, LeftAction, Player, RightAction, RunAction};

#[derive(Debug, Reflect, Resource)]
pub struct VolumeSettings {
    master: f32,
    music: f32,
    sound: f32,
}

#[derive(PoolLabel, Debug, Clone, PartialEq, Eq, Hash)]
struct MovementPool;

#[derive(PoolLabel, Debug, Clone, PartialEq, Eq, Hash)]
struct JumpPool;

#[derive(PoolLabel, Debug, Clone, PartialEq, Eq, Hash)]
struct RunPool;

pub struct AudioPlugin;
impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
       app
           .register_type::<VolumeSettings>()
           .add_plugins(SeedlingPlugin::default())
           .add_systems(Startup, init_audio)
           .add_observer(walk_audio::<ForwardAction>)
           .add_observer(walk_audio::<BackwardAction>)
           .add_observer(walk_audio::<LeftAction>)
           .add_observer(walk_audio::<RightAction>)
           .add_observer(jump_audio)
           .add_observer(dash_audio);
    }
}

fn init_audio(
    mut commands: Commands,
) {
    commands.insert_resource(VolumeSettings {
        master: 100.,
        music: 100.,
        sound: 100.,
    });
    commands.spawn(SamplerPool(MovementPool));
    commands.spawn(SamplerPool(JumpPool));
    commands.spawn(SamplerPool(RunPool));
}

fn walk_audio<T: InputAction>(
    _trigger: On<Fire<T>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    pool_query: Query<&SamplePlayer, With<MovementPool>>,
) {
    if let Ok(_) = pool_query.single() {
        return;
    }
    let mut rng = rand::rng();
    let file = format!("audio/footsteps/tile/{}.ogg", rng.random_range(0..8));
    commands.spawn(
        (
            MovementPool,
            SamplePlayer::new(asset_server.load(file)),
        ));
}

fn jump_audio(
    _trigger: On<Fire<JumpAction>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    pool_query: Query<&SamplePlayer, With<JumpPool>>,
) {
    if let Ok(_) = pool_query.single() {
        return;
    }
    commands.spawn((
            JumpPool,
            SamplePlayer::new(asset_server.load("audio/jump/jumppp11.ogg")),
    ));
}

fn dash_audio(
    _trigger: On<Start<RunAction>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
            RunPool,
            SamplePlayer::new(asset_server.load("audio/dash/steam hisses - Marker 1.wav")),
    ));
}
