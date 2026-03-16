use bevy::prelude::*;

use crate::game_schedule::GameSchedule;

pub struct HealthPlugin;

impl Plugin for HealthPlugin{
  fn build(&self, app: &mut App) {
    app
      .add_observer(on_damage)
      .add_systems(Update, remove_dead.in_set(GameSchedule::DespawnEntites));
  } 
}


fn remove_dead(
  query:Query<Entity, With<Dead>>,
  mut commands:Commands,
){
  for entity in query{
    commands.entity(entity).despawn();
  }
}


#[derive(Component, Default)]
pub struct Hurtable;


#[derive(Component)]
#[require(Hurtable)]
pub struct Health{
  pub health:f32,
}


#[derive(EntityEvent)]
#[entity_event(auto_propagate, propagate = &'static ChildOf)]
pub struct DamageEvent {
  #[event_target]
  pub target: Entity,
  pub value:f32,
}


#[derive(Component)]
pub struct Dead;


fn on_damage(
  event:On<DamageEvent>, 
  mut query:Query<&mut Health>,
  mut commands:Commands,
){
  info!("Damage entity {}", event.target);
  //info!("Entity {} took damage {}", event.entity, event.value);
  if let Ok(mut health) = query.get_mut(event.target){
    health.health -= event.value;
    info!("Entity {} took damage {}, health now {}", event.target, event.value, health.health);
    if health.health <= 0.{
      commands.entity(event.target).insert(Dead);
          info!("Entity {} dead", event.target);
    }
    
  };
}
