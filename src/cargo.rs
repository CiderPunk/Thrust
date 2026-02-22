use avian3d::prelude::Collider;
use bevy::{gltf::GltfMesh, prelude::*};

use crate::{asset_management::{AssetLoadState, GameAssets}, game_state::GameState};


pub struct CargoPlugin;
impl Plugin for CargoPlugin{
  fn build(&self, app: &mut App) {
    app
    .init_resource::<CargoResources>()
    .add_systems(OnEnter(AssetLoadState::Loaded), init_cargo_reosurces)
    .add_systems(OnEnter(GameState::Initialize), spawn_cargo);
 }
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash, Reflect, Debug)]
pub enum CargoType{  
  #[default]
  Sphere,
}



#[derive(Component, Default, Reflect, Debug)]
#[reflect(Component, Default)]
#[type_path = "api"]
struct CargoStart(CargoType);



#[derive(Resource, Default)]
struct CargoResources{
  collider: Option<Collider>,
  band_mesh: Handle<Mesh>,
  crystal_mesh:Handle<Mesh>,
  ring_material: Handle<StandardMaterial>,
}




fn init_cargo_reosurces(
 mut cargo_resources: ResMut<CargoResources>,
  game_assets: Res<GameAssets>,
  gltf_assets: Res<Assets<Gltf>>,
  gltf_meshes: Res<Assets<GltfMesh>>,
  meshes: Res<Assets<Mesh>>,
) -> Result<()>{
  let models = gltf_assets.get(&game_assets.models).ok_or("Couldn't get models")?;
  let band_primative = &gltf_meshes.get( 
      models.named_meshes.get("cargo-band")
      .ok_or("Couldn't get cargo band mesh")?,)
      .ok_or("Couldn't get cargo band mesh data")?
    .primitives[0];


  let crystal_primative = &gltf_meshes.get(
    models.named_meshes.get("cargo-crystal")
    .ok_or("Couldn't get cargo crystal mesh")?,)
      .ok_or("Couldn't get cargo crystal mesh data")?
      .primitives[0];

  cargo_resources.band_mesh = band_primative.mesh.clone();
  cargo_resources.ring_material = band_primative.material.clone().ok_or("No ring material")?;
  cargo_resources.crystal_mesh = crystal_primative.mesh.clone();
  Ok(())
}


fn spawn_cargo(
  query: Query<&GlobalTransform, With<CargoStart>>,
  mut commands: Commands,
  cargo_resources: Res<CargoResources>,
){}