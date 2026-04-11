use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{asset_management::AssetLoadState, game_physics::GameLayer, game_state::GameState};

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
    //*visiblity = Visibility::Hidden;
  }
}

fn on_player_entered(event:On<CollisionStart>){
  info!("Player entered sensor");
  //trigger
}
fn on_player_exited(event:On<CollisionEnd>){
  info!("Player exited sensor");
  //trigger
}

