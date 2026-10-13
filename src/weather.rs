use bevy::prelude::*;

pub enum Weather {
    Sunny,
    Cloudy,
    Rainy,
    Stormy,
    Windy,
    Foggy,
}

#[derive(Resource)]
pub struct WeatherTimer(pub Timer);

pub struct WeatherPlugin;
impl Plugin for WeatherPlugin {
    fn build(&self, app: &mut App) {
       app; 
    }
}

fn change_weather() {
}
