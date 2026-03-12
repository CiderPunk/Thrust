use bevy::{light::NotShadowCaster, prelude::*};
use crate::{game_state::GameState, shaders::ShaderMaterials};

pub struct StaticLightsPlugin;

#[derive(Component, Default, Reflect, Debug)]
#[reflect(Component, Default)]
#[type_path = "api"]
struct StaticSpotLight{
  throw_distance:f32,
}


#[derive(Component, Default, Reflect, Debug)]
#[reflect(Component, Default)]
#[type_path = "api"]
struct LightRaysMaterial;


impl Plugin for StaticLightsPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(OnEnter(GameState::Initialize), (init_static_lights, init_ray_material));
  }
  
}

fn init_ray_material(
  query:Query<(Entity, &Name), With<LightRaysMaterial>>,
  materials:Res<ShaderMaterials>,
  mut commands: Commands,
){
  for (entity, name) in query{
    commands
      .entity(entity)
      .remove::<MeshMaterial3d<StandardMaterial>>()
      .insert((
        NotShadowCaster,
        MeshMaterial3d(materials.rays.clone())
      ));
    info!("ray material added {}",name);
  }
}


fn init_static_lights(
  mut query:Query<(&mut SpotLight, &StaticSpotLight)>,
){
  for (mut light, marker) in query.iter_mut(){
    light.range = marker.throw_distance;
    info!("light initialized");
  }
}

