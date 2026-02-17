use bevy::prelude::*;
use crate::game_state::GameState;

pub struct StaticLightsPlugin;

#[derive(Component, Default, Reflect, Debug)]
#[reflect(Component, Default)]
#[type_path = "api"]
struct StaticSpotLight{
  throw_distance:f32,
}

impl Plugin for StaticLightsPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(OnEnter(GameState::Initialize), init_static_lights);
  }
}
/*
fn init_static_lights(
  query:Query<&StaticSpotLight>,
){
  for (marker) in query.iter(){
    info!("light initialized {:}", marker.throw_distance);
  }
}
 */

fn init_static_lights(
  mut query:Query<(&mut SpotLight, &StaticSpotLight)>,
){
  for (mut light, marker) in query.iter_mut(){
    light.range = marker.throw_distance;
    info!("light initialized");
  }
}

