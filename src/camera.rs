use avian3d::prelude::LinearVelocity;
use bevy::prelude::*;

use crate::{game_schedule::GameSchedule, game_state::GameState, player::Player};


pub struct CameraPlugin;
impl Plugin for CameraPlugin{
  fn build(&self, app: &mut App) {
    app
    .add_systems(Startup, init_camera)
    .add_systems(OnEnter(GameState::Initialize), remove_scene_cameras)
    .add_systems(PostUpdate, update_camera);
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


const LOOK_AHEAD_TIME:f32 = 0.5;
const MIN_LOOK_AHEAD_VELOCITY:f32 = 4.;

fn update_camera(
  player:Single<(&GlobalTransform, &LinearVelocity), With<Player>>,
  mut camera:Single<&mut Transform, With<GameCamera>>,
  time:Res<Time>,
){
  let (transform, velocity) = *player;

  let view_position = Vec3::new(transform.translation().x, transform.translation().y, transform.translation().z + 100.) 
    - if velocity.0.length_squared() > MIN_LOOK_AHEAD_VELOCITY * MIN_LOOK_AHEAD_VELOCITY { (velocity.0 * LOOK_AHEAD_TIME) } else{ (Vec3::ZERO)};
  camera.translation = camera.translation.lerp(view_position, time.delta_secs() * 2.0);
  camera.look_at(transform.translation(), Vec3::Y);


  //camera.translation = Vec3::new(player.translation().x, player.translation().y, player.translation().z + 100.);

}