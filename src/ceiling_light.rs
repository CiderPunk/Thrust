use avian3d::prelude::*;
use bevy::{asset::LoadState, gltf::GltfMesh, light::NotShadowCaster, math::VectorSpace, prelude::*};

use crate::{asset_management::{AssetLoadState, GameAssets}, game_state::GameState, get_gltf_primative, health::Health};
pub struct CeilingLightPlugin;

impl Plugin for CeilingLightPlugin{
  fn build(&self, app: &mut App) {
    app
      .insert_resource(CeilingLightResources{
        ..default()
      })
      .add_systems(OnEnter(AssetLoadState::Loaded), init_light_resources)
      .add_systems(OnEnter(GameState::Initialize), spawn_ceiling_lights);
  }
}


#[derive(Component, Default, Reflect, Debug)]
#[reflect(Component, Default)]
#[type_path = "api"]
struct CeilingLightSpawn;



#[derive(Resource, Default)]
struct CeilingLightResources{
  light_material:Handle<StandardMaterial>,
  frame_material:Handle<StandardMaterial>,
  frame:Handle<Mesh>,
  light:Handle<Mesh>,
  collider:Option<Collider>,
}


fn init_light_resources(
  mut light_resources:ResMut<CeilingLightResources>,
  game_assets: Res<GameAssets>,
  gltf_assets: Res<Assets<Gltf>>,
  gltf_meshes: Res<Assets<GltfMesh>>,
  mut meshes: ResMut<Assets<Mesh>>,
) -> Result<()> {
  info!("Init ceiling light resources");
  let models = gltf_assets.get(&game_assets.models).ok_or("Couldn't get models")?;
  let frame = get_gltf_primative!(gltf_meshes, models,"ceiling-frame" );
  let light = get_gltf_primative!(gltf_meshes, models,"ceiling-light" );
  let collision = get_gltf_primative!(gltf_meshes, models,"ceiling-collision" );
  light_resources.frame_material = frame.material.clone().ok_or("no flame material")?;
  light_resources.light_material = light.material.clone().ok_or("no flame material")?;
  light_resources.frame = frame.mesh.clone();
  light_resources.light = light.mesh.clone();
  let collision_mesh =  meshes.get(&collision.mesh).clone().ok_or("Couldn't get collision mesh")?;
  light_resources.collider = Some(Collider::convex_hull_from_mesh(collision_mesh).ok_or("couldn't create collider from mesh")?);
  Ok(())
}


fn spawn_ceiling_lights(
  resources:Res<CeilingLightResources>,
  mut commands:Commands,
  query:Query<&Transform, With<CeilingLightSpawn>>,
){
  for transform in query{
    info!("Spawning ceiling light");
    commands.spawn((
      Mesh3d(resources.frame.clone()),
      MeshMaterial3d(resources.frame_material.clone()),
      transform.clone().with_scale(Vec3::splat(1.)),
      NotShadowCaster,
      resources.collider.clone().unwrap(),
      RigidBody::Static,
      Health{ health:100.,},
      children![
        (
          NotShadowCaster,
          Mesh3d(resources.light.clone()),
          MeshMaterial3d(resources.light_material.clone()),
          Transform::from_translation(Vec3::ZERO)
        )
      ]
    ));


  }
}