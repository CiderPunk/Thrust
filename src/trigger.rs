use core::slice;
use bevy::{platform::collections::HashMap, prelude::*};

use crate::game_state::GameState;

pub struct TriggerPlugin;
impl Plugin for TriggerPlugin{
  fn build(&self, app: &mut App) {
    app
      .add_systems(OnEnter(GameState::TriggerInitialize), init_triggers);


    app.add_observer(|event: On<TriggerEvent>| {
    info!("Global TriggerEvent caught for entity: {:?}", event.entity);
});
  }
}

#[derive(Component, Debug, Clone, PartialEq, Eq, Hash, Default, Copy, Reflect)]
pub enum TriggerRepeatType{
  #[default]
  Repeat,
  Once,
}

#[derive(EntityEvent)]
#[entity_event(auto_propagate, propagate = &'static TriggerSource)]
pub struct TriggerEvent{
  #[event_target]
  pub entity:Entity,
  pub state:bool,
}


#[derive(Component, Default, Reflect, Debug)]
#[reflect(Component, Default)]
#[type_path = "api"]
pub struct TriggerSender{
  targets:String,
  repeat:TriggerRepeatType,
}

#[derive(Component, Default, Reflect, Debug)]
#[reflect(Component, Default)]
#[type_path = "api"]
pub struct TriggerReceiver{
  name:String,
}

#[derive(Component, Debug, PartialEq, Eq)]
#[relationship(relationship_target = TriggerRecipients)]
pub struct TriggerSource(pub Entity);

#[derive(Component, Default, Debug, PartialEq, Eq, Clone)]
#[relationship_target(relationship = TriggerSource, linked_spawn)]
pub struct TriggerRecipients(Vec<Entity>);


#[derive(Component, Debug, PartialEq, Eq)]
#[relationship(relationship_target = TriggerSenders)]
pub struct TriggerDestination(pub Entity);

#[derive(Component, Default, Debug, PartialEq, Eq, Clone)]
#[relationship_target(relationship = TriggerDestination, linked_spawn)]
pub struct TriggerSenders(Vec<Entity>);


fn init_triggers(
  sender_query:Query<(Entity, &TriggerSender)>,
  receiver_query:Query<(Entity, &TriggerReceiver)>,
  mut commands:Commands,
){
  let mut receivers = HashMap::new();
  for (entity, trigger) in receiver_query{
    receivers.insert(trigger.name.clone(), entity);
  }

  for (entity, trigger) in sender_query{
    let targets:Vec<&str> = trigger
      .targets.split(",")
      .map(|s| s.trim())
      .filter(|s| !s.is_empty())
      .collect();

    for target in targets{
      if let Some(receiver) = receivers.get(target){
        info!("Trigger link created: {} {} ", target, entity);
        //spawn our trigger link
        let bridge = commands.spawn(( 
          trigger.repeat.clone(),
          TriggerSource(entity),
          TriggerDestination(*receiver),
        ))
        .observe(trigger_relay)
        .id();

      }
    }
  }
}


fn trigger_relay(
  event:On<TriggerEvent>,
  query:Query<(&TriggerDestination, &TriggerRepeatType)>,
  mut commands:Commands,
){
  info!("Relayed event!");
  if let Ok((destination, repeat)) = query.get(event.entity){
    commands.trigger(TriggerEvent{ entity: destination.0, state:event.state });
    match repeat{
      TriggerRepeatType::Repeat => (),
      TriggerRepeatType::Once => commands.entity(event.entity).despawn(),
    }
  };
}