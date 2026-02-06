use bevy::prelude::*;

use crate::{game_state::GameState, player::Player};


pub struct CameraPlugin;
impl Plugin for CameraPlugin{
  fn build(&self, app: &mut App) {
    app
    .add_systems(Startup, init_camera)
    .add_systems(OnEnter(GameState::Initialize), remove_scene_cameras)
    .add_systems(Update, update_camera);
  }
}

#[derive(Component)]
pub struct GameCamera;

fn init_camera(mut commands:Commands){
  commands.spawn((
    GameCamera,
    Camera3d::default(),
    Camera {
      order: 1,
      ..default()
    },
    Transform::from_translation(Vec3::new(0.,0.,100.)).looking_at(Vec3::ZERO, Vec3::Y),
  ));
}



fn remove_scene_cameras(query:Query<&mut Camera, Without<GameCamera>> ){
  for mut camera in query{
    camera.is_active = false;
  }
}

fn update_camera(
  player:Single<&GlobalTransform, With<Player>>,
  mut camera:Single<&mut Transform, With<GameCamera>>,
){
  camera.translation = Vec3::new(player.translation().x, player.translation().y, player.translation().z + 100.);

}