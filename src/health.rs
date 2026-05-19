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
  query:Query<(&mut Dead,Entity)>,
  mut commands:Commands,
  time:Res<Time>,
){
  for (mut dead,entity) in query{
    dead.timer.tick(time.delta());
    if dead.timer.is_finished(){
      commands.entity(entity).despawn();
    }
  }
}


#[derive(Component, Default)]
pub struct Hurtable;



#[derive(Component, Default, Reflect, Debug)]
#[reflect(Component, Default)]
#[require(Hurtable)]
#[type_path = "api"]
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


#[derive(Component, Default)]
pub struct Dead{
  pub timer:Timer,
}

impl Dead{
  pub fn new(time_to_live:f32)->Self{
    Self{ timer:Timer::from_seconds(time_to_live, TimerMode::Once)}
  }
}

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
      commands.entity(event.target).insert(Dead::new(0.));
        info!("Entity {} dead", event.target);
    }
    
  };
}
