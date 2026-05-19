use std::time::Duration;

use avian3d::prelude::*;
use bevy::{platform::collections::HashMap, prelude::*};

use crate::{asset_management::{AssetLoadState, GameAssets}, game_schedule::GameSchedule, game_state::GameState, trigger::TriggerEvent};
pub struct MapPlugin;
impl Plugin for MapPlugin {
  fn build(&self, app: &mut App) {
    app
      .init_resource::<AnimationGraphHandles>()
      .add_systems(OnEnter(AssetLoadState::Loaded), spawn_map)
      .add_systems(OnEnter(GameState::Initialize), (init_collision_hulls, init_dynamic_collision_hulls, init_moving_blocks))
      .add_systems(FixedUpdate, move_blocks.in_set(GameSchedule::MoveEntities ))
      .add_observer(init_animation)
      ;
  }
}


#[derive(Component, Default, Reflect, Debug)]
#[reflect(Component, Default)]
#[type_path = "api"]
struct ColliderMesh;

#[derive(Component, Default, Reflect, Debug)]
#[reflect(Component, Default)]
#[type_path = "api"]
struct Animated{
  action:String,
}

#[derive(Component, Default, Reflect, Debug)]
#[reflect(Component, Default)]
#[type_path = "api"]
struct DynamicCollisionHull{
  leave_mesh:bool,
}


#[derive(Component, Default, Reflect, Debug)]
#[reflect(Component, Default)]
#[type_path = "api"]
struct CollisionHull{
  leave_mesh:bool,
}



#[derive(Resource, FromWorld)]
struct AnimationGraphHandles(HashMap<String,GraphDetails>);

struct GraphDetails{
  idx:AnimationNodeIndex,
  handle:Handle<AnimationGraph>,
}


#[derive(Component, Default, Reflect, Debug)]
#[reflect(Component, Default)]
#[type_path = "api"]
struct MovingBlockSettings{
  //where this moves to
  displacement:Vec3,
  //time to make the move
  movement_time:f32,
  //leave the child collider mesh visible
  leave_collider:bool,
  //initial direction of movement
  direction:MovementDirection,
  //progress through first movement
  init_state:f32,
  //if the movement should continue and flkip after reaching the end
  oscilate:bool,
  //time to spend at each end
  pause_time:f32,
}


#[derive(Component, Default)]
struct MovingBlock{
  start_transform:Transform,
  end_transform:Transform,
  time:Timer,
  pause_timer:Timer,
  direction:MovementDirection,
  oscilator:bool,
}


fn spawn_map(
  mut commands: Commands,
  game_assets: Res<GameAssets>,
  gltf_assets: Res<Assets<Gltf>>,
  mut next_state: ResMut<NextState<GameState>>,
  mut animation_handles:ResMut<AnimationGraphHandles>,
  mut animation_graphs: ResMut<Assets<AnimationGraph>>,
)->Result<()> {
  let map = gltf_assets.get(&game_assets.map_model).ok_or("Couldn't get map")?;
  let root = commands.spawn( 
    SceneRoot(map.scenes[0].clone())
  ).id();

  for  animation in  map.named_animations.iter(){
    let (graph,idx) = AnimationGraph::from_clip(animation.1.clone());
    //info!("action name: '{}' anim: {:?}  graph: {:?}", animation.0, animation.1, graph);
    let handle = animation_graphs.add(graph);
    animation_handles.0.insert(animation.0.to_string(), GraphDetails { idx, handle });
  }

  info!("Map spawned! {}", root);
  //start initialization
  next_state.set(GameState::Initialize);
  Ok(())
} 

#[derive(PartialEq, Eq, Default, Reflect, Debug, Clone, Copy)]
enum MovementDirection{
  Forward,
  #[default]
  Backward,
}



fn init_moving_blocks(
  mut query: Query<(Entity, &MovingBlockSettings, &Transform)>, 
  child_query: Query<&Children>,
  collider_query:Query<(&Mesh3d, Entity), With<ColliderMesh>>,
  mut commands:Commands,
  meshes: ResMut<Assets<Mesh>>,
){
 for (entity, settings, transform) in query.iter_mut() {
    info!("moving block found: {:?}", entity);
    let end_transform = transform.with_translation(transform.translation + settings.displacement);
    let mut time = Timer::from_seconds(settings.movement_time, TimerMode::Once);
    time.set_elapsed(Duration::from_secs_f32(settings.movement_time * settings.init_state));
    commands.entity(entity)
    .insert(
      MovingBlock{
        start_transform: *transform, 
        end_transform,
        time,
        direction:settings.direction.clone(),
        oscilator:settings.oscilate,
        pause_timer: Timer::from_seconds(settings.pause_time, TimerMode::Once),
      }
    )
    .observe(trigger_movement);

    for child in child_query.iter_descendants(entity){
      if let Ok((collider_mesh, collider_entity)) = collider_query.get(child){
        info!("found collider for: {:?}", entity);
        if let Some(mesh) = meshes.get(collider_mesh){
          if let Some(collider) = Collider::convex_hull_from_mesh(mesh){
            info!("moving block setup complete: {:?}", entity);
            commands.entity(entity)
            .insert((
              collider,
              RigidBody::Kinematic,
            ));
            if !settings.leave_collider{
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
    if block.time.is_finished(){ 
      if block.oscilator{
        block.pause_timer.tick(time.delta());
        if block.pause_timer.is_finished(){
          info!("reversing moveing block");
          block.direction = match block.direction {
            MovementDirection::Forward => MovementDirection::Backward,
            MovementDirection::Backward => MovementDirection::Forward,
          };
          block.time.reset();
          block.pause_timer.reset();
        }
      }
      continue; 
    }
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
    //info!("Collision hull found: {:?}", hull_entity);
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

fn init_dynamic_collision_hulls(
  mut query: Query<(&mut Visibility, Entity, &DynamicCollisionHull), With<Mesh3d>>, 
  mut commands: Commands,
) {
  for (mut visiblity,hull_entity, collision_hull) in query.iter_mut() {
    //info!("Collision hull found: {:?}", hull_entity);
    commands.entity(hull_entity)
      .insert((
        ColliderConstructor::ConvexHullFromMesh,
        RigidBody::Kinematic,
      ));

    if !collision_hull.leave_mesh{
      *visiblity = Visibility::Hidden;
    }
  }
}

fn init_animation(
  event:On<Add, AnimationPlayer>,
  mut query:Query<(&mut AnimationPlayer, &Animated)>,
  animation_handles:Res<AnimationGraphHandles>,
  mut commands:Commands,
){
  info!("Animaton player found {}", event.entity);
  if let Ok((mut player, animated)) = query.get_mut(event.entity){
    info!("Attempting to start animation clip {}", animated.action );
    if let Some(anim_deets) = animation_handles.0.get(&animated.action){
      info!("starting anim {}", animated.action);
      commands.entity(event.entity).insert(AnimationGraphHandle(anim_deets.handle.clone()));
      player.play(anim_deets.idx).repeat();

    };
  }; 
}

/*
fn init_animation(
  event:On<Add, AnimationPlayer>,
  query:Query<&AnimationPlayer>,
  game_assets: Res<GameAssets>,
  gltf_assets: Res<Assets<Gltf>>,
){

  if let Some(map) = gltf_assets.get(&game_assets.map_model){
    if let Ok(player) = query.get(event.entity){
      for &anim in map.animations.iter(){
      anim.
      }
    }; 
  };
}
   */
