use bevy::prelude::*;



///
/// wioll eventually handle dialogue prompts for the player
pub struct DialoguePlugin;
impl Plugin for DialoguePlugin{
  fn build(&self, app: &mut App) {

  }
}

#[derive(Component)]
pub struct Dialogue{
  pub content:String,
  pub display_time:Timer,

}