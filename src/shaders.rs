use bevy::{prelude::*, render::render_resource::AsBindGroup};
use bevy_asset_loader::prelude::*;
use crate::asset_management::AssetLoadState;

pub struct ShaderPlugin;
impl Plugin for ShaderPlugin{
    fn build(&self, app: &mut App) {
      app
        .configure_loading_state(LoadingStateConfig::new(AssetLoadState::Startup)
          .load_collection::<GameShaders>());
    }
}


#[derive(AssetCollection, Resource)]
struct GameShaders{
  #[asset(path="shaders/spotlight_ray_material.wgsl")]
  bounds_shader: Handle<SpotLightRayShader>

}


#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct SpotLightRayShader {
  #[uniform(0)]
  color1: LinearRgba,
  #[uniform(1)]
  color2: LinearRgba,
  alpha_mode: AlphaMode,
}