use core::slice;
use std::default;

use bevy::prelude::*;

pub struct TriggerPlugin;
impl Plugin for TriggerPlugin{
  fn build(&self, app: &mut App) {

  }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Copy, Reflect)]
pub enum TriggerRepeatType{
  #[default]
  Repeat,
  Once,
}


#[derive(Component, Default, Reflect, Debug)]
#[reflect(Component, Default)]
#[type_path = "api"]
pub struct TriggerOutput{
  groups:Vec<String>,
  repeat:TriggerRepeatType,

}

#[derive(Component, Default, Reflect, Debug)]
#[reflect(Component, Default)]
#[type_path = "api"]
pub struct TriggeruInput{
  groups:Vec<String>
}


#[derive(Component)]
pub struct Trigger{
  input:ChildTriggers,
  ouput:ChildTriggers,
}


#[derive(Component, Debug, PartialEq, Eq)]
#[relationship(relationship_target = ChildTriggers)]
pub struct ParentTrigger(pub Entity);

#[derive(Component, Default, Debug, PartialEq, Eq, Clone)]
#[relationship_target(relationship = ParentTrigger, linked_spawn)]
pub struct ChildTriggers(Vec<Entity>);





impl<'a> IntoIterator for &'a ChildTriggers {
  type Item = <Self::IntoIter as Iterator>::Item;

  type IntoIter = slice::Iter<'a, Entity>;

  #[inline(always)]
  fn into_iter(self) -> Self::IntoIter {
    self.0.iter()
  }
}

