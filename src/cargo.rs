use avian3d::prelude::*;
use bevy::{gltf::GltfMesh, prelude::*};

use crate::{asset_management::{AssetLoadState, GameAssets}, game_state::GameState,  get_gltf_primative, physics::GameLayer};


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
  MetalCrate,
}

#[derive(Component, Default, Reflect, Debug)]
#[reflect(Component, Default)]
#[type_path = "api"]
struct CargoStart(CargoType);

#[derive(Component)]
struct CargoItem;

#[derive(Resource, Default)]
struct CargoResources{
  crate_collider: Option<Collider>,
  band_mesh: Handle<Mesh>,
  crystal_mesh:Handle<Mesh>,
  ring_material: Handle<StandardMaterial>,
  crate_mesh:Handle<Mesh>,
  crate_material:Handle<StandardMaterial>,
}




fn init_cargo_reosurces(
 mut cargo_resources: ResMut<CargoResources>,
  game_assets: Res<GameAssets>,
  gltf_assets: Res<Assets<Gltf>>,
  gltf_meshes: Res<Assets<GltfMesh>>,
  meshes: Res<Assets<Mesh>>,
) -> Result<()>{
  let models = gltf_assets.get(&game_assets.models).ok_or("Couldn't get models")?;

  let band_primative = get_gltf_primative!(gltf_meshes, models, "cargo-band" );
  let crystal_primative = get_gltf_primative!(gltf_meshes, models, "cargo-crystal" );
  let crate_primative = get_gltf_primative!(gltf_meshes, models, "crate-metal" );

  cargo_resources.band_mesh = band_primative.mesh.clone();
  cargo_resources.ring_material = band_primative.material.clone().ok_or("No ring material")?;
  cargo_resources.crystal_mesh = crystal_primative.mesh.clone();
  cargo_resources.crate_material = crate_primative.material.clone().ok_or("no crate material")?;
  cargo_resources.crate_mesh = crate_primative.mesh.clone();
  cargo_resources.crate_collider =  Some(Collider::cuboid(2.,2.,2.));
  Ok(())
}

fn spawn_cargo(
  query: Query<&Transform, With<CargoStart>>,
  mut commands: Commands,
  cargo_resources: Res<CargoResources>,
){

  for transform in query{
    info!("Spawning cargo at {}", transform.translation);
    commands.spawn((
      CargoItem,
      Mesh3d(cargo_resources.crate_mesh.clone()),
      MeshMaterial3d(cargo_resources.crate_material.clone()),
      CollisionLayers::new([GameLayer::Cargo, GameLayer::Default], [GameLayer::Default]),
      Transform::from_translation(transform.translation).with_scale(Vec3::splat(1.6)),
      RigidBody::Dynamic,
      TransformInterpolation,
      LockedAxes::new().lock_rotation_y().lock_rotation_x().lock_translation_z(),
      cargo_resources.crate_collider.clone().unwrap(),
    ));

  }
}