use bevy::{gltf::GltfMesh, prelude::*};

use crate::{asset_management::{AssetLoadState, GameAssets}, get_gltf_primative};

pub struct WreckagePlugin;

impl Plugin for WreckagePlugin{
  fn build(&self, app: &mut App) {
    app.insert_resource(WreckResources::default())
      .add_systems(OnEnter(AssetLoadState::Loaded), init_wreck_reosurces);
  }
}

#[derive(Resource, Default)]
pub struct WreckResources{
  pub wreck_material: Handle<StandardMaterial>,
}


fn init_wreck_reosurces(
  mut wreck_resources: ResMut<WreckResources>,
  game_assets: Res<GameAssets>,
  gltf_assets: Res<Assets<Gltf>>,
  gltf_meshes: Res<Assets<GltfMesh>>,
 // mut meshes: ResMut<Assets<Mesh>>,
) -> Result<()> {
  let models = gltf_assets.get(&game_assets.models).ok_or("Couldn't get models")?;
  let wreck_cube = get_gltf_primative!(gltf_meshes, models,"wreck-cube" );
  wreck_resources.wreck_material = wreck_cube.material.clone().ok_or("Missing wreck material")?;
  Ok(())
}