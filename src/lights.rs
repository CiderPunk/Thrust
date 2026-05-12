use bevy::prelude::*;

use crate::game_state::GameState;


pub struct LightsPlugin;
impl Plugin for LightsPlugin{
  fn build(&self, app: &mut App) {
    app.add_systems(OnEnter(GameState::Initialize), init_point_lights);
  }
}

#[derive(Component, Default, Reflect, Debug)]
#[reflect(Component, Default)]
#[type_path = "api"]
struct LightSettings{
  range:f32,
}

fn init_point_lights(
  mut query:Query<(&LightSettings, &mut PointLight)>
){
  for (settings, mut light) in query.iter_mut(){
    light.range = settings.range;
  }
}