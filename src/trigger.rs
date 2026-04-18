use core::slice;
use bevy::{platform::collections::HashMap, prelude::*, tasks::futures_lite::io::Repeat};

use crate::{asset_management::GameAssets, dialogue::Dialogue, game_schedule::GameSchedule, game_state::GameState};

pub struct TriggerPlugin;
impl Plugin for TriggerPlugin{
  fn build(&self, app: &mut App) {
    app
      .add_systems(OnEnter(GameState::TriggerInitialize), wire_triggers)
      .add_systems(Update, delay_trigger.in_set(GameSchedule::EntityUpdates));
  }
}


#[derive(EntityEvent)]
#[entity_event(auto_propagate, propagate = &'static TriggerTarget)]
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
pub struct TriggerTarget(pub Entity);

#[derive(Component, Default, Debug, PartialEq, Eq, Clone)]
#[relationship_target(relationship = TriggerTarget, linked_spawn)]
pub struct TriggerSources (Vec<Entity>);

#[derive(Component)]
pub struct TriggerRelay{
  targets:Vec<Entity>,
  delay:Option<f32>,
  repeat:bool,
  invert:bool,
}

#[derive(Component)]
pub struct TriggerDelay{
  timer:Timer,
  state:bool,
}

fn wire_triggers(
  sender_query:Query<(Entity, &TriggerSender)>,
  receiver_query:Query<(Entity, &TriggerReceiver)>,
  trigger_data: Res<Assets<TriggerDataCollection>>,
  game_assets: Res<GameAssets>,
  mut commands:Commands,
){

  info!("Initializing triggers");
  let Some(trigger_collection) = trigger_data.get(&game_assets.map_data) else{ return; };

  let dialogue_map:HashMap<String,Entity> = trigger_collection.dialogues.iter()
    .map(|dialogue_def|
      (
        dialogue_def.name.clone(), 
        commands.spawn(
          Dialogue{ content: dialogue_def.content.clone(), display_time: Timer::from_seconds(dialogue_def.display_time, TimerMode::Once)  }
        ).id()  
      )
    ).collect();

  let trigger_def_map:HashMap<String,&TriggerDef> = trigger_collection.triggers.iter()
    .map(|data|(data.name.clone(), data))
    .collect();



  let mut receivers_map:HashMap<String,Vec<Entity>> = HashMap::new();
  for (entity, receiver) in receiver_query{
    receivers_map.entry(receiver.name.clone())
      .or_default().push(entity);
  }
  

  let mut relays:HashMap<String,Entity> = HashMap::new();

  for (entity, trigger_sender) in sender_query{
    if let Some(relay_entity) = get_trigger_relay(&mut commands, &trigger_sender.name, &receivers_map, &dialogue_map, &mut relays, &trigger_def_map){
      commands.entity(entity).insert(TriggerTarget(relay_entity));
    }
  }
}

fn get_trigger_relay(
  commands: &mut Commands, 
  name:&String,
  receivers_map: &HashMap<String, Vec<Entity>>,
  dialogue_map:&HashMap<String,Entity>,
  relays: &mut HashMap<String, Entity>,
  trigger_def_map: &HashMap<String,&TriggerDef>,
)->Option<Entity> {

  //have we already creatyed this relay?
  if let Some(relay) = relays.get(name){
    info!("got existing trigger relay {}", name);
    return Some(*relay);
  };
  //get the def
  let Some(trigger_def) = trigger_def_map.get(name) else { 
    return None; 
  };
  
  let mut targets:Vec<Entity> = Vec::with_capacity(trigger_def.targets.len());
  for target_name in &trigger_def.targets{
    if let Some(reciever) =  receivers_map.get(target_name){
      targets.extend(reciever);
    }
    if let Some(dialogue) = dialogue_map.get(target_name){
      targets.push(*dialogue);
    }if let Some(relay) = get_trigger_relay(commands, target_name, &receivers_map, &dialogue_map, relays, &trigger_def_map){
      targets.push(relay);
    }
  }
   
  let relay = commands.spawn(
    TriggerRelay{ 
      targets, 
      delay:trigger_def.delay,
      repeat:  match trigger_def.repeat { Some(val) => val, None => true,},
      invert:  match trigger_def.invert { Some(val) => val, None => false,},
    }
  ).observe(trigger_relay).id();

  info!("added new trigger relay {}", name);

  relays.insert(name.clone(), relay);
  return Some(relay);
}

fn trigger_relay(
  event:On<TriggerEvent>,
  query:Query<&TriggerRelay>,
  mut commands:Commands,
){
  info!("Relayed event {} {}", event.entity, event.state);
  if let Ok(trigger_relay) = query.get(event.entity){
    match trigger_relay.delay{
        Some(delay) => {
          commands.entity(event.entity).insert(TriggerDelay{
            timer:Timer::from_seconds(delay,TimerMode::Once),
            state:event.state,
          });
        },
        None => {
          for target in trigger_relay.targets.clone(){
            commands.trigger( TriggerEvent{ entity:target, state: trigger_relay.invert != event.state} );
          }
          if !trigger_relay.repeat {
            commands.entity(event.entity).despawn();
          }
        },
    }
  }
}

fn delay_trigger(
  query:Query<(&mut TriggerDelay, &TriggerRelay, Entity)>,
  mut commands:Commands,
  time:Res<Time>,
){
  for (mut delay, relay, entity) in query{
    info!("delay timer: {}", delay.timer.elapsed_secs());
    delay.timer.tick(time.delta());
    if delay.timer.is_finished(){
      for target in relay.targets.clone(){
        commands.trigger( TriggerEvent{ entity:target, state: relay.invert != delay.state} );
      }
      if !relay.repeat{
        commands.entity(entity).despawn();
      }
      else{
        commands.entity(entity).remove::<TriggerDelay>();
      }
    }
  }
}


#[derive(serde::Deserialize, Asset, TypePath)]
pub struct TriggerDataCollection{
  triggers:Vec<TriggerDef>,
  dialogues:Vec<DialogueDef>,
}

#[derive(serde::Deserialize, Asset, TypePath)]
struct TriggerDef{
  name: String,
  targets:Vec<String>,
  delay:Option<f32>,
  invert:Option<bool>,
  repeat:Option<bool>,
}
#[derive(serde::Deserialize, Asset, TypePath)]
struct DialogueDef{
  name: String,
  display_time:f32,
  content:String,
  image:Option<String>,
}
