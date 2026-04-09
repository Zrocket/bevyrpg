use bevy::prelude::*;

use crate::{MenuState,  widgets::{self, ui_root}};

#[derive(Component, Reflect)]
pub struct UiVideoSettings;

#[derive(Resource, Reflect, Debug, Deref, DerefMut)]
struct CameraSensitivitySetting(Vec2);

impl Default for CameraSensitivitySetting {
    fn default() -> Self {
        Self(Vec2::ONE)
    }
}

#[derive(Resource, Reflect, Debug, Default)]
struct VsyncSettings(bool);

#[derive(Component, Reflect)]
#[reflect(Component)]
struct CameraSensitivityLabel;

fn lower_camera_sensitivity(
    _trigger: On<Pointer<Click>>,
    mut sensitivity: ResMut<CameraSensitivitySetting>
) {
    sensitivity.0 -= 0.1;
    const MIN_SENSITIVITY: f32 = 0.1;
    sensitivity.x = sensitivity.x.max(MIN_SENSITIVITY);
    sensitivity.y = sensitivity.y.max(MIN_SENSITIVITY);
}

fn raise_camera_sensitivity(
    _trigger: On<Pointer<Click>>,
    mut sensitivity: ResMut<CameraSensitivitySetting>
) {
    sensitivity.0 += 0.1;
    const MAX_SENSITIVITY: f32 = 20.0;
    sensitivity.x = sensitivity.x.min(MAX_SENSITIVITY);
    sensitivity.y = sensitivity.y.min(MAX_SENSITIVITY);
}

fn update_camera_sensitivity_label(
    mut commands: Commands,
    cam: Single<(Entity, )>,
    sensitivity: Res<CameraSensitivitySetting>,
) {
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct VsyncLabel;

fn enable_vsync(
    _trigger: On<Pointer<Click>>,
    mut setting: ResMut<VsyncSettings>,
) {
    setting.0 = true;
}

fn disable_vsync(
    _trigger: On<Pointer<Click>>,
    mut setting: ResMut<VsyncSettings>,
) {
    setting.0 = false;
}

fn update_vsync(
    mut window: Single<&mut Window>,
    setting: Res<VsyncSettings>,
) {
    window.present_mode = if setting.0 {
        bevy::window::PresentMode::AutoVsync
    } else {
        bevy::window::PresentMode::Mailbox
    };
}

fn update_vsync_label(
    mut label: Single<&mut Text, With<VsyncLabel>>,
    setting: Res<VsyncSettings>,
) {
    label.0 = if setting.0 {
        "On".into()
    } else {
        "Off".into()
    };
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct CameraFovLabel;

#[derive(Component, Reflect)]
#[reflect(Component)]
struct QualityLabel;

#[derive(Resource, Debug, PartialEq, Eq, Clone, Copy, Reflect, Default, PartialOrd, Ord)]
pub enum QualitySetting {
    Low,
    #[default]
    Medium,
    High,
    Ultra,
}

impl QualitySetting {
    pub fn next(&self) -> Self {
        match self {
            QualitySetting::Low => QualitySetting::Medium,
            QualitySetting::Medium => QualitySetting::High,
            QualitySetting::High => QualitySetting::Ultra,
            QualitySetting::Ultra => QualitySetting::Low,
        }
    }
}

fn change_quality(
    _trigger: On<Pointer<Click>>,
    mut label: Single<&mut Text, With<QualityLabel>>,
    mut settings: ResMut<QualitySetting>,
) {
    *settings = settings.next();
    label.0 = format!("{:?}", *settings);
}

fn on_quality_label_add(
    mut label: Single<&mut Text, With<QualityLabel>>,
    settings: ResMut<QualitySetting>,
) {
    label.0 = format!("{:?}", *settings);
}

pub struct VideoSettingsMenuUiPlugin;
impl Plugin for VideoSettingsMenuUiPlugin {
    fn build(&self, app: &mut bevy::app::App) {
       app
           .register_type::<UiVideoSettings>()
           .init_resource::<VsyncSettings>()
           .init_resource::<CameraSensitivitySetting>()
           .add_systems(OnEnter(MenuState::VideoSettings), spawn_video_settings_menu)
           .add_systems(
               Update,
               (
                   update_vsync.run_if(resource_exists_and_changed::<VsyncSettings>),
                   update_vsync_label
               ).run_if(in_state(MenuState::VideoSettings)),
            );
    }
}

fn spawn_video_settings_menu(
    mut commands: Commands,
) {
    commands.spawn((
            ui_root("Video Settings"),
            DespawnOnExit(MenuState::VideoSettings),
            GlobalZIndex(2),
            UiVideoSettings,
            children![
            (
                widgets::label("Camera Sensetivity"),
                Node {
                    justify_self: JustifySelf::End,
                    ..default()
                },
            ),
            (
                widgets::label("Vsync"),
                Node {
                    justify_self: JustifySelf::End,
                    ..default()
                },
            ),
                widgets::plus_minus_bar(VsyncLabel, disable_vsync, enable_vsync),
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
