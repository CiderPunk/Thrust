use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{asset_management::{AssetLoadState, GameAssets}, game_state::GameState};
pub struct MapPlugin;
impl Plugin for MapPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(OnEnter(AssetLoadState::Loaded), spawn_map)
      .add_systems(OnEnter(GameState::Initialize), (init_collision_hulls, init_moving_blocks));
  }
}

#[derive(Component, Default, Reflect, Debug)]
#[reflect(Component, Default)]
#[type_path = "api"]
struct CollisionHull{
  leave_mesh:bool,
}


#[derive(Component, Default, Reflect, Debug)]
#[reflect(Component, Default)]
#[type_path = "api"]
struct MovingBlock{
  displacement:Vec3,
  timer:Option<Timer>,
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


fn init_moving_blocks(
  mut query: Query<(Entity, &MovingBlock), With<Mesh3d>>, 
  mut commands:Commands,
){
 for (entity, moving_block) in query.iter_mut() {
    info!("moving block found: {:?}", entity);
    commands.entity(entity)
      .insert(ColliderConstructor::TrimeshFromMesh)
      .insert(RigidBody::Kinematic);


  }


}



fn init_collision_hulls(
  mut query: Query<(&mut Visibility, Entity, &CollisionHull), With<Mesh3d>>, 
  mut commands: Commands,
) {
  for (mut visiblity,hull_entity, collision_hull) in query.iter_mut() {
    info!("Collision hull found: {:?}", hull_entity);
    commands.entity(hull_entity)
      .insert(ColliderConstructor::TrimeshFromMesh)
      .insert(RigidBody::Static);

    if !collision_hull.leave_mesh{
      *visiblity = Visibility::Hidden;
    }
  }
}





