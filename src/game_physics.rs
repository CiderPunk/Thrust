use avian3d::prelude::*;
use bevy::prelude::*;

pub struct GamePhysicsPlugin;
impl Plugin for GamePhysicsPlugin{
  fn build(&self, app: &mut App) {
    app.add_observer(apply_impacts);
  }
}


fn apply_impacts(
  event:On<ImpactEvent>,
  mut query:Query<Forces>,
){
  if let Ok(mut forces) = query.get_mut(event.target){
    forces.apply_linear_impulse_at_point(event.force, event.location); 
  }
}


#[derive(EntityEvent)]
#[entity_event(auto_propagate, propagate = &'static ChildOf)]
pub struct ImpactEvent {
  #[event_target]
  pub target: Entity,
  pub location:Vec3,
  pub force:Vec3,
}



#[derive(PhysicsLayer, Default)]
pub enum GameLayer{
  #[default]
  Default,
  Player,
  Cargo,
  Bullets,
}


#[derive(Component)]
pub struct PhysicsBody;

