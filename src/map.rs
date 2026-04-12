use std::time::Duration;

use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{asset_management::{AssetLoadState, GameAssets}, game_schedule::GameSchedule, game_state::GameState, trigger::TriggerEvent};
pub struct MapPlugin;
impl Plugin for MapPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(OnEnter(AssetLoadState::Loaded), spawn_map)
      .add_systems(OnEnter(GameState::Initialize), (init_collision_hulls, init_moving_blocks))
      .add_systems(FixedUpdate, move_blocks.in_set(GameSchedule::MoveEntities ));
  }
}


#[derive(Component, Default, Reflect, Debug)]
#[reflect(Component, Default)]
#[type_path = "api"]
struct ColliderMesh;



#[derive(Component, Default, Reflect, Debug)]
#[reflect(Component, Default)]
#[type_path = "api"]
struct CollisionHull{
  leave_mesh:bool,
}


#[derive(Component, Default, Reflect, Debug)]
#[reflect(Component, Default)]
#[type_path = "api"]
struct MovingBlockSettings{
  displacement:Vec3,
  movement_time:f32,
  leave_mesh:bool,
}


#[derive(Component, Default)]
struct MovingBlock{
  start_transform:Transform,
  end_transform:Transform,
  time:Timer,
  direction:MovementDirection,
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

#[derive(PartialEq, Eq, Default)]
enum MovementDirection{
  Forward,
  #[default]
  Backward,
}



fn init_moving_blocks(
  mut query: Query<(&mut Visibility, Entity, &MovingBlockSettings, &Transform)>, 
  child_query: Query<&Children>,
  collider_query:Query<(&Mesh3d, Entity), With<ColliderMesh>>,
  mut commands:Commands,
  meshes: ResMut<Assets<Mesh>>,
){
 for (mut visiblity, entity, settings, transform) in query.iter_mut() {
    info!("moving block found: {:?}", entity);
    for child in child_query.iter_descendants(entity){
      if let Ok((collider_mesh, collider_entity)) = collider_query.get(child){
        info!("found collider for: {:?}", entity);
        if let Some(mesh) = meshes.get(collider_mesh){
          if let Some(collider) = Collider::convex_hull_from_mesh(mesh){
            let end_transform = transform.with_translation(transform.translation + settings.displacement);
            info!("moving block setup complete: {:?}", entity);

            commands.entity(entity)
              .insert((
                collider,
                RigidBody::Kinematic,
                MovingBlock{
                  start_transform: *transform, 
                  end_transform,
                  time: Timer::from_seconds(settings.movement_time, TimerMode::Once),
                  direction:MovementDirection::Backward,
                }
              ))
              .observe(trigger_movement);
            if !settings.leave_mesh{
              commands.entity(collider_entity).despawn();
            }
          }


        }
      };
      continue;
    }



  
  }
}

fn trigger_movement(
  event:On<TriggerEvent>,
  mut query:Query<&mut MovingBlock>,
){
  if let Ok(mut block) = query.get_mut(event.entity){
    info!("block movement triggered");
    let new_direction = match event.state {
      true => MovementDirection::Forward,
      false => MovementDirection::Backward,
    };
    if new_direction != block.direction{
      //reverse the timer
      let remaining = block.time.remaining_secs();
      block.time.reset();
      block.time.set_elapsed(Duration::from_secs_f32(remaining));
      block.direction = new_direction;
    }
  };

}

fn move_blocks(
  mut query:Query<(&mut Transform, &mut MovingBlock)>,
  time:Res<Time>,
){
  for (mut transform, mut block) in query.iter_mut(){
    if block.time.is_finished(){ continue; }
    block.time.tick(time.delta());
    let fraction = block.time.fraction();
    let points = match block.direction{
      MovementDirection::Forward => (block.start_transform, block.end_transform),
      MovementDirection::Backward => (block.end_transform, block.start_transform),
    };
    //info!("moving block {}", fraction);
    transform.translation = points.0.translation.lerp(points.1.translation, fraction);
    transform.rotation = points.0.rotation.slerp(points.1.rotation, fraction);
    transform.scale = points.0.scale.lerp(points.1.scale, fraction);
  }
}

fn init_collision_hulls(
  mut query: Query<(&mut Visibility, Entity, &CollisionHull), With<Mesh3d>>, 
  mut commands: Commands,
) {
  for (mut visiblity,hull_entity, collision_hull) in query.iter_mut() {
    info!("Collision hull found: {:?}", hull_entity);
    commands.entity(hull_entity)
      .insert((
        ColliderConstructor::TrimeshFromMesh,
        RigidBody::Static,
      ));

    if !collision_hull.leave_mesh{
      *visiblity = Visibility::Hidden;
    }
  }
}





