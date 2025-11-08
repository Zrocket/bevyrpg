use bevy::prelude::*;

#[derive(Debug, Reflect, Resource)]
pub struct VolumeSettings {
    master: f32,
    music: f32,
    sound: f32,
}

pub struct AudioPlugin;
impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
       app
           .register_type::<VolumeSettings>(); 
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
}
