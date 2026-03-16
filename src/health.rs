use bevy::prelude::*;

pub struct HealthPlugin;

impl Plugin for HealthPlugin{
  fn build(&self, app: &mut App) {
    app.add_observer(on_damage);
  } 
}

#[derive(Component)]
pub struct Health{
  pub health:f32,
}

#[derive(EntityEvent)]
#[entity_event(auto_propagate, propagate = &'static ChildOf)]
pub struct Damage {
  #[event_target]
  pub entity: Entity,
  pub value:f32,
}

fn on_damage(
  event:On<Damage>,
  mut query:Query<&mut Health>,
  mut commands:Commands,
){
  info!("Damage");
  //info!("Entity {} took damage {}", event.entity, event.value);
  if let Ok(mut health) = query.get_mut(event.entity){
    health.health -= event.value;
    info!("Entity {} took damage {}, health now {}", event.entity, event.value, health.health);
  };
}
