use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub struct AssetManagementPlugin;

impl Plugin for AssetManagementPlugin {
  fn build(&self, app: &mut App) {
    app
      .init_state::<AssetLoadState>()
      .add_loading_state(
        LoadingState::new(AssetLoadState::Startup)
        .continue_to_state(AssetLoadState::Loaded)
        .load_collection::<GameAssets>()
      );
  }
} 


#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default, Copy)]
pub enum AssetLoadState {
  #[default]
  Startup,
  Loaded,
}


#[derive(AssetCollection, Resource)]
pub struct GameAssets {
  #[asset(path = "models.glb")]
  pub models: Handle<Gltf>,
  #[asset(path = "maps/map1.glb")]
  pub map_model: Handle<Gltf>,
}

