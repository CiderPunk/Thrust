use bevy::prelude::*;
use crate::{asset_management::{AssetLoadState, GameAssets}, game_state::GameState};
pub struct GamePlugin;

impl Plugin for GamePlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(OnEnter(AssetLoadState::Loaded), init_game)
      .add_systems(OnEnter(GameState::Initialize), start_game);
  }
}

fn init_game(
  mut next_state: ResMut<NextState<GameState>>,
 ){
  info!("Game initializing...");
  next_state.set(GameState::Initialize);

}

fn start_game(
  mut next_state: ResMut<NextState<GameState>>,
 ){
  info!("Game Started");
  next_state.set(GameState::Playing);

} 