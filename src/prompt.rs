use bevy::prelude::*;

pub struct PromptPlugin;

impl Plugin for PromptPlugin{
  fn build(&self, app: &mut App) {

  }
}

#[derive(Component, Default, Reflect, Debug)]
#[reflect(Component, Default)]
#[type_path = "api"]
pub struct Prompt{
  pub message:String,  
}

