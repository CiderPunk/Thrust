mod game_state;
mod asset_management;
mod game;
mod player;
mod map;
mod game_schedule;
mod camera;
mod static_lights;
mod shaders;
mod cargo;
mod macros;

use bevy::{asset::AssetMetaCheck, color::palettes::css::WHITE, prelude::*};
use bevy_enhanced_input::EnhancedInputPlugin;
use bevy_skein::SkeinPlugin;
use avian3d::prelude::*;

use crate::{asset_management::AssetManagementPlugin, camera::CameraPlugin, cargo::CargoPlugin, game::GamePlugin, game_schedule::GameSchedulePlugin, game_state::GameStatePlugin, map::MapPlugin, player::PlayerPlugin, shaders::ShaderPlugin, static_lights::StaticLightsPlugin};


const APP_NAME: &str = "Caves";

fn main() {
  App::new()
    .add_plugins(
      DefaultPlugins.set(WindowPlugin {
          primary_window: Some(Window {
            title: APP_NAME.into(),
            name: Some(APP_NAME.into()),
            fit_canvas_to_parent: true,
            visible: true,
            ..default()
          }),
          ..default()
        })
         .set(AssetPlugin {
          meta_check: AssetMetaCheck::Never,
          watch_for_changes_override: Some(true),
          ..default()
        }),
      )
    .add_plugins((
      SkeinPlugin::default(), 
      PhysicsPlugins::default(),
      EnhancedInputPlugin,
      //PhysicsDebugPlugin,
    ))
    .add_plugins((
      GameSchedulePlugin,
      AssetManagementPlugin,
      CameraPlugin,
      GamePlugin,
      GameStatePlugin,
      PlayerPlugin,
      MapPlugin,
      StaticLightsPlugin,
      ShaderPlugin,
      CargoPlugin,
    ))
    .insert_resource(ClearColor(Color::srgb(0., 0., 0.)))
    .insert_resource(GlobalAmbientLight {
        color: WHITE.into(),
        brightness: 20.0,
        ..default()
    })
    .run();

}
