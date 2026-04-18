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

#[derive(Reflect, Debug, Default, Copy, Clone)]
pub enum SensorTriggerState{
  #[default]
  All,
  OnEnter,
  OnExit,
}

#[derive(Component, Default, Reflect, Debug)]
#[reflect(Component, Default)]
#[type_path = "api"]
pub struct PlayerSensor{
  trigger_on:SensorTriggerState
}

fn init_sensors(
  mut query: Query<(&mut Visibility, Entity, &PlayerSensor), With<Mesh3d>>, 
  mut commands: Commands,
) {
  info!("initializing sensors {}", query.count());
  
  for (mut visiblity,entity, sensor_config) in query.iter_mut() {
    info!("Sensor found: {:?}", entity);


    let sensor = commands.entity(entity)
      .insert((
        ColliderConstructor::ConvexHullFromMesh, 
        Sensor, 
        CollisionEventsEnabled,
        CollisionLayers::new([GameLayer::Sensor], [GameLayer::Player]),
      )).id();
      match sensor_config.trigger_on{
        SensorTriggerState::All => { 
          commands.entity(sensor)
            .observe(on_player_entered)
            .observe(on_player_exited);
          },
        SensorTriggerState::OnEnter =>  { commands.entity(sensor).observe(on_player_entered);},
        SensorTriggerState::OnExit =>  { commands.entity(sensor).observe(on_player_exited);}
      }

    *visiblity = Visibility::Hidden;
  }
}

fn on_player_entered(
  event:On<CollisionStart>,
  mut commands:Commands,
){

  info!("Player entered sensor {}", event.collider1);
  commands.trigger(TriggerEvent{ entity: event.collider1, state:true });

}


fn on_player_exited(
  event:On<CollisionEnd>,
  mut commands:Commands,
){
  info!("Player exited sensor {}", event.collider1);
  commands.trigger(TriggerEvent{ entity: event.collider1, state:false });
}

