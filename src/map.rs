use avian3d::prelude::*;
use bevy::{ecs::system::command::insert_batch, prelude::*};

use crate::{asset_management::{AssetLoadState, GameAssets}, game_state::GameState};
pub struct MapPlugin;
impl Plugin for MapPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(OnEnter(AssetLoadState::Loaded), spawn_map)
      .add_systems(OnEnter(GameState::Initialize), init_map);
  }
}

fn spawn_map(
  mut commands: Commands,
  game_assets: Res<GameAssets>,
  gltf_assets: Res<Assets<Gltf>>,
  mut next_state: ResMut<NextState<GameState>>,
)->Result<()> {
  let map = gltf_assets.get(&game_assets.map_model).ok_or("Couldn't get map")?;
  commands.spawn( 
    SceneRoot(map.scenes[0].clone())
  );
  // Placeholder for map spawning logic
  info!("Map spawned!");
  //start initialization
  next_state.set(GameState::Initialize);
  Ok(())
} 

#[derive(Component, Default, Reflect, Debug)]
#[reflect(Component, Default)]
#[type_path = "api"]
struct CollisionHull;

fn init_map(
  mut query: Query<(&mut Visibility, Entity), (With<CollisionHull>, With<Mesh3d>)>, 
  mut commands: Commands,
) {
  for (mut visiblity,hull_entity) in query.iter_mut() {
    info!("Collision hull found: {:?}", hull_entity);
    commands.entity(hull_entity)
      .insert(ColliderConstructor::TrimeshFromMesh)
      .insert(RigidBody::Static);
    *visiblity = Visibility::Hidden;
  }

}