use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{asset_management::AssetLoadState, game_physics::GameLayer, game_state::GameState, trigger::TriggerEvent};

pub struct SensorPlugin;

impl Plugin for SensorPlugin{
  fn build(&self, app: &mut App) {
    app
      .add_systems(OnEnter(GameState::Initialize), init_sensors);
  }
}

#[derive(Component, Default, Reflect, Debug)]
#[reflect(Component, Default)]
#[type_path = "api"]
pub struct PlayerSensor{
}

fn init_sensors(
  mut query: Query<(&mut Visibility, Entity, &PlayerSensor), With<Mesh3d>>, 
  mut commands: Commands,
) {
  info!("initializing sensors {}", query.count());
  
  for (mut visiblity,entity, _) in query.iter_mut() {
    info!("Sensor found: {:?}", entity);
    commands.entity(entity)
      .insert((
        ColliderConstructor::ConvexHullFromMesh, 
        Sensor, 
        CollisionEventsEnabled,
        CollisionLayers::new([GameLayer::Sensor], [GameLayer::Player]),
      ))
      .observe(on_player_entered)
      .observe(on_player_exited);
    *visiblity = Visibility::Hidden;
  }
}

fn on_player_entered(
  event:On<CollisionStart>,
  mut commands:Commands,
){
  if let Some(trigger_entity) = event.body1 {
    info!("Player entered sensor {}", trigger_entity);
    commands.trigger(TriggerEvent{ entity: trigger_entity, state:true });
  }
}


fn on_player_exited(
  event:On<CollisionEnd>,
  mut commands:Commands,
){
  if let Some(trigger_entity) = event.body1 {
    
    commands.trigger(TriggerEvent{ entity: trigger_entity, state:false });
  }
  info!("Player exited sensor");
  //trigger
}

