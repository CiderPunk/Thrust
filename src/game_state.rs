use bevy::prelude::*;


pub struct GameStatePlugin;
impl Plugin for GameStatePlugin {
  fn build(&self, app: &mut App) {
    app.init_state::<GameState>();
  }
}



#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default, Copy)]
pub enum GameState {
  #[default]
  Loading,
  Initialize,
  Playing,
}