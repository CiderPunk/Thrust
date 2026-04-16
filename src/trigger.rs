use core::slice;
use bevy::{platform::collections::HashMap, prelude::*};

use crate::{asset_management::GameAssets, game_state::GameState};

pub struct TriggerPlugin;
impl Plugin for TriggerPlugin{
  fn build(&self, app: &mut App) {
    app
      .add_systems(OnEnter(GameState::TriggerInitialize), wire_triggers);
  }
}


#[derive(EntityEvent)]
pub struct TriggerEvent{
  #[event_target]
  pub entity:Entity,
  pub state:bool,
}


#[derive(Component, Default, Reflect, Debug)]
#[reflect(Component, Default)]
#[type_path = "api"]
pub struct TriggerSender{
  name:String,
}

#[derive(Component, Default, Reflect, Debug)]
#[reflect(Component, Default)]
#[type_path = "api"]
pub struct TriggerReceiver{
  name:String,
}

#[derive(Component, Debug, PartialEq, Eq)]
#[relationship(relationship_target = TriggerSources)]
pub struct TriggerRelay(pub Entity);

#[derive(Component, Default, Debug, PartialEq, Eq, Clone)]
#[relationship_target(relationship = TriggerRelay, linked_spawn)]
pub struct TriggerSources (Vec<Entity>);

/*
#[derive(Component, Debug, PartialEq, Eq)]
#[relationship(relationship_target = TriggerSenders)]
pub struct TriggerDestination(pub Entity);

#[derive(Component, Default, Debug, PartialEq, Eq, Clone)]
#[relationship_target(relationship = TriggerDestination, linked_spawn)]
pub struct TriggerSenders(Vec<Entity>);


impl<'a> IntoIterator for &'a TriggerRecipients {
    type Item = <Self::IntoIter as Iterator>::Item;

    type IntoIter = slice::Iter<'a, Entity>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}
 */



fn wire_triggers(
  sender_query:Query<(Entity, &TriggerSender)>,
  receiver_query:Query<(Entity, &TriggerReceiver)>,
  trigger_data: Res<Assets<TriggerDataCollection>>,
  game_assets: Res<GameAssets>,
  mut commands:Commands,
){
  
  let Some(trigger_collection) = trigger_data.get(&game_assets.map_data) else{ return; };

  let receivers:HashMap<String,Entity> = receiver_query.iter()
    .map(|(entity,trigger)|(trigger.name.clone(), entity))
    .collect();

  let mut trigger_defs:HashMap<String,(&TriggerData, Option<Entity>)> = trigger_collection.triggers.iter()
    .map(|trigger_data|(
      trigger_data.name.clone(),
      (trigger_data, None)
    )).collect();

  for (sender_entity, sender_trigger) in sender_query{
    spawn_trigger_relay(commands, );
    
    _sender_trigger.name
    
    
    
    commands.spawn((



    )).id();




  }
}


fn spawn_trigger_relay(
  mut commands:Commands,
  name:&String,
  triggering_entity:Entity,
  trigger_defs:&mut HashMap<String,(&TriggerData, &mut Option<Entity>)>,
)->Entity{
  let Some((trigger_data,  existing_entity)) = trigger_defs.get_mut(name) else{ return; };
  let relay = match **existing_entity{
      Some(entity) => entity,
      None => {
        let triggers = match trigger_data.triggers{
          Some(triggers) => triggers.iter().map(|t| spawn_trigger_relay(commands, name, triggering_entity, trigger_defs)),
          None => todo!(),
        }

        if let Some(triggers) = trigger_data.triggers{

        }

        let id = commands.spawn((  

        )).id();
        **existing_entity = Some(id);
        id
      },
    };
  relay
}

/*
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

        commands.entity(entity).observe(propegate_triggers);
        //spawn our trigger link
        commands.spawn(( 
          trigger.repeat.clone(),
          TriggerSource(entity),
          TriggerDestination(*receiver),
        ))
        .observe(trigger_relay);

      }
    }
  }
}
 */
fn propegate_triggers(
  event:On<TriggerEvent>,
  query:Query<&TriggerRecipients>,
  mut commands:Commands,
){
  info!("Propegate event {} {}", event.entity, event.state);
  if let Ok(recipients) = query.get(event.entity){
    for &entity in recipients.into_iter(){
      commands.trigger(TriggerEvent{ entity, state:event.state });
    }
  };
}

fn trigger_relay(
  event:On<TriggerEvent>,
  query:Query<(&TriggerDestination, &TriggerRepeatType)>,
  mut commands:Commands,
){
  info!("Relayed event {} {}", event.entity, event.state);
  if let Ok((destination, repeat)) = query.get(event.entity){
    commands.trigger(TriggerEvent{ entity: destination.0, state:event.state });
    match repeat{
      TriggerRepeatType::Repeat => (),
      TriggerRepeatType::Once => commands.entity(event.entity).despawn(),
    }
  };
}



#[derive(serde::Deserialize, Asset, TypePath)]
pub struct TriggerDataCollection{
  triggers:Vec<TriggerData>,
  dialogues:Vec<Dialogue>,
}

#[derive(serde::Deserialize, Asset, TypePath)]
struct TriggerData{
  name: String,
  triggers:Option<Vec<String>>,
  groups:Option<Vec<String>>,
  dialogue:Option<String>,
  delay:Option<f32>,
  invert:Option<bool>,
  one_shot:Option<bool>,
}
#[derive(serde::Deserialize, Asset, TypePath)]
struct Dialogue{
  name: String,
  display_time:f32,
  content:String,
  image:Option<String>,
}
