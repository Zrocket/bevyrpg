use bevy::{ecs::query::QueryFilter, prelude::*};
use bevy_seedling::{pool::SamplerPool, prelude::{MainBus, MusicPool, SoundEffectsBus, Volume, VolumeNode}};

use crate::{MenuState, PerceptualVolumeConverter, widgets::{self, ui_root}};

#[derive(Component, Reflect)]
pub struct UiSoundSettings;

#[derive(Component, Reflect)]
#[reflect(Component)]
struct GlobalVolumeLabel;

#[derive(Component, Reflect)]
#[reflect(Component)]
struct MusicVolumeLabel;

#[derive(Component, Reflect)]
#[reflect(Component)]
struct SfxVolumeLabel;

#[derive(Resource, Reflect, Debug)]
struct VolumeTicks(usize);

impl VolumeTicks {
    fn increment(&mut self) {
        self.0 = Self::MAX_TICK_COUNT.min(self.0 + 1);
    }

    fn decrement(&mut self) {
        self.0 = self.0.saturating_sub(1);
    }

    fn fraction(&self) -> f32 {
        self.0 as f32 / Self::MAX_TICK_COUNT as f32
    }

    fn label(&self) -> String {
        let filled = "█".repeat(self.0);
        let empty = " ".repeat(VolumeTicks::MAX_TICK_COUNT - self.0);
        filled + &empty + "|"
    }

    // How many ticks the volume slider supports
    const MAX_TICK_COUNT: usize = 20;
}

impl From<VolumeTicks> for Volume {
    fn from(value: VolumeTicks) -> Self {
        PerceptualVolumeConverter::default().to_volume(value.fraction())
    }
}

impl From<Volume> for VolumeTicks {
    fn from(value: Volume) -> Self {
        VolumeTicks(
            (PerceptualVolumeConverter::default().to_perceptual(value)
             * Self::MAX_TICK_COUNT as f32)
            .round() as usize,
        )
    }
}

pub struct SoundSettingsMenuUiPlugin;
impl Plugin for SoundSettingsMenuUiPlugin {
    fn build(&self, app: &mut bevy::app::App) {
       app
           .register_type::<UiSoundSettings>()
           .add_systems(OnEnter(MenuState::SoundSettings), spawn_sound_settings_menu)
           .add_systems(Update, (
                   update_volume_label::<With<GlobalVolumeLabel>, With<MainBus>>,
                   update_volume_label::<With<MusicVolumeLabel>, With<SamplerPool<MusicPool>>>,
                   update_volume_label::<With<SfxVolumeLabel>, With<SoundEffectsBus>>,
           ).run_if(in_state(MenuState::SoundSettings)));
    }
}

fn spawn_sound_settings_menu(
    mut commands: Commands,
) {
    commands.spawn((
            ui_root("Sound Settings"),
            DespawnOnExit(MenuState::SoundSettings),
            GlobalZIndex(2),
            UiSoundSettings,
            children![
            (
                widgets::label("Master Volume"),
                Node {
                    justify_self: JustifySelf::End,
                    ..default()
                },
            ),
            widgets::plus_minus_bar(
                GlobalVolumeLabel,
                lower_volume::<With<MainBus>>,
                raise_volume::<With<MainBus>>,
            ),
            (
                widgets::label("Music Volume"),
                Node {
                    justify_self: JustifySelf::End,
                    ..default()
                },
            ),
            widgets::plus_minus_bar(
                MusicVolumeLabel,
                lower_volume::<With<SamplerPool<MusicPool>>>,
                raise_volume::<With<SamplerPool<MusicPool>>>,
            ),
            (
                widgets::label("Sound Volume"),
                Node {
                    justify_self: JustifySelf::End,
                    ..default()
                },
            ),
            widgets::plus_minus_bar(
                SfxVolumeLabel,
                lower_volume::<With<SoundEffectsBus>>,
                raise_volume::<With<SoundEffectsBus>>,
            ),
                widgets::button("Back", back_to_settings_menu),
            ],
    ));
}

fn back_to_settings_menu(
    _: On<Pointer<Click>>,
    mut pause_menu_state: ResMut<NextState<MenuState>>,
) {
    pause_menu_state.set(MenuState::Settings);
}

fn lower_volume<F: QueryFilter>(_on: On<Pointer<Click>>, mut volume: Single<&mut VolumeNode, F>) {
    let mut ticks = VolumeTicks::from(volume.volume);
    ticks.decrement();
    volume.volume = ticks.into();
    println!("LOWER {:?}", volume.volume);
}

fn raise_volume<F: QueryFilter>(_on: On<Pointer<Click>>, mut volume: Single<&mut VolumeNode, F>) {
    let mut ticks = VolumeTicks::from(volume.volume);
    ticks.increment();
    volume.volume = ticks.into();
    println!("RAISE {:?}", volume.volume);
}

fn update_volume_label<F1, F2>(mut label: Single<&mut Text, F1>, master: Single<&VolumeNode, F2>)
where
    F1: QueryFilter,
    F2: QueryFilter,
{
    let ticks = VolumeTicks::from(master.volume);
    label.0 = ticks.label();
}
