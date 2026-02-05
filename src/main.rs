mod game_state;
mod asset_management;
mod game;
mod player;
mod map;
mod game_schedule;


use bevy::prelude::*;
use bevy_enhanced_input::EnhancedInputPlugin;
use bevy_skein::SkeinPlugin;
use avian3d::prelude::*;

use crate::{asset_management::AssetManagementPlugin, game::GamePlugin, game_state::GameStatePlugin, map::MapPlugin, player::PlayerPlugin, game_schedule::GameSchedulePlugin};

fn main() {
  App::new()
    .add_plugins((
      DefaultPlugins, 
      SkeinPlugin::default(), 
      PhysicsPlugins::default(),
      EnhancedInputPlugin,
      PhysicsDebugPlugin,
    ))
    .add_plugins((
      GameSchedulePlugin,
      AssetManagementPlugin,
      GamePlugin,
      GameStatePlugin,
      PlayerPlugin,
      MapPlugin,
    ))
    .run();

}
