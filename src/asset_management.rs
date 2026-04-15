use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_common_assets::json::JsonAssetPlugin;

use crate::trigger::TriggerDataCollection;

pub struct AssetManagementPlugin;

impl Plugin for AssetManagementPlugin {
  fn build(&self, app: &mut App) {
    app
      .init_state::<AssetLoadState>()
      .add_plugins(JsonAssetPlugin::<TriggerDataCollection>::new(&["map.json"]))
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
  #[asset(path = "maps/map3.glb")]
  pub map_model: Handle<Gltf>,
  #[asset(path = "maps/map3.map.json")]
  pub map_data: Handle<TriggerDataCollection>
}

