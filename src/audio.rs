use bevy::prelude::*;
use bevy_enhanced_input::{action::InputAction, prelude::{Fire, Start}};
use bevy_seedling::{SeedlingPlugins, pool::SamplerPool, prelude::{PoolLabel, Volume}, sample::SamplePlayer};
use bevy_tnua::TnuaController;
use rand::random_range;

use crate::{FlashlightAction, JumpAction, MovementAction, Player, PlayerControlScheme, RunAction};

/// Constructor for taking a user-presented control value and converting it to a volume.
#[derive(Debug, Clone, Copy)]
pub struct PerceptualVolumeConverter {
    /// When the perceptual control value is below this value, the mapping will be linear between:
    /// - 0 perceptual = 0 volume
    /// - [`Self::pivot_pos`] perceptual = [`Self::pivot_volume`] volume
    ///
    /// When above this value, the mapping will be exponential between:
    /// - [`Self::pivot_pos`] perceptual = [`Self::pivot_volume`] volume
    /// - 1.0 perceptual = 0 dB
    pub pivot_pos: f32,
    /// The volume to use at [`Self::pivot_pos`]
    pub pivot_volume: Volume,
}

impl Default for PerceptualVolumeConverter {
    fn default() -> Self {
        Self {
            pivot_pos: 0.01,
            pivot_volume: Volume::Decibels(-50.0),
        }
    }
}

impl PerceptualVolumeConverter {
    /// Converts a user-presented control value in  \[0.0, 1.0\] to a [`Volume`].
    pub fn to_volume(self, perceptual: f32) -> Volume {
        if perceptual < self.pivot_pos {
            let min = 0.0_f32;
            let max = self.pivot_volume.linear();
            let t = perceptual / self.pivot_pos;
            Volume::Linear(min.lerp(max, t))
        } else {
            let min = self.pivot_volume.decibels();
            let max = 0.0;
            let t = (perceptual - self.pivot_pos) /  (1.0 - self.pivot_pos);
            Volume::Decibels(min.lerp(max, t))
        }
    }

    /// Converts a [`Volume`] into a  user-presented control value in [0.0, 1.0].
    pub fn to_perceptual(self, volume: Volume) -> f32 {
        if volume.linear() <= self.pivot_volume.linear() {
            let vol = volume.linear();
            let pivot = self.pivot_volume.linear();
            let t = vol / pivot;
            t * self.pivot_pos
        } else {
            let vol = volume.decibels();
            let pivot = self.pivot_volume.decibels();
            let t = (vol - pivot) / (0.0 - pivot);
            self.pivot_pos + t * (1.0 - self.pivot_pos)
        }
    }
}

#[derive(Debug, Reflect, Resource)]
pub struct VolumeSettings {
    master: f32,
    music: f32,
    sound: f32,
}

#[derive(PoolLabel, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MovementPool;

#[derive(PoolLabel, Debug, Clone, PartialEq, Eq, Hash)]
struct JumpPool;

#[derive(PoolLabel, Debug, Clone, PartialEq, Eq, Hash)]
struct RunPool;

pub struct AudioPlugin;
impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
       app
           .register_type::<VolumeSettings>()
           .add_plugins(SeedlingPlugins)
           .add_systems(Startup, init_audio)
           .add_observer(walk_audio::<MovementAction>)
           .add_observer(jump_audio)
           .add_observer(dash_audio)
           .add_observer(flashlight_audio);
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
    tnua_query: Query<&TnuaController<PlayerControlScheme>, With<Player>>,
) {
    if pool_query.single().is_ok() {
        return;
    } else if let Ok(tnua_controller) = tnua_query.single()
    && tnua_controller.is_airborne().unwrap() {
        return;
    }

    let file = format!("audio/footsteps/tile/{}.ogg", random_range(0..8));
    commands.spawn(
        (
            MovementPool,
            SamplePlayer::new(asset_server.load(file)),
        ));
}

fn jump_audio(
    _trigger: On<Start<JumpAction>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    pool_query: Query<&SamplePlayer, With<JumpPool>>,
    tnua_query: Query<&TnuaController<PlayerControlScheme>, With<Player>>,
) {
    if pool_query.single().is_ok() {
        return;
    } else if let Ok(tnua_controller) = tnua_query.single()
    && tnua_controller.is_airborne().unwrap() {
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

fn flashlight_audio(
    _trigger: On<Start<FlashlightAction>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
            SamplePlayer::new(asset_server.load("audio/clicks/click.1.ogg")),
    ));
}

pub fn ui_hover(
    _trigger: On<Pointer<Over>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
            SamplePlayer::new(asset_server.load("audio/clicks/click.1.ogg")),
    ));
}

pub fn ui_click(
    _trigger: On<Pointer<Click>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
            SamplePlayer::new(asset_server.load("audio/clicks/click.1.ogg")),
    ));
}
