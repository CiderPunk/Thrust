use bevy::prelude::*;


pub struct MovementPlugin;

#[derive(Component, Default)]
pub struct Velocity(pub Vec3);

impl Plugin for MovementPlugin{
  fn build(&self, app: &mut App) {
    app.add_systems(Update, update_position);
  }
}

fn update_position(
  query:Query<(&mut Transform, &Velocity)>,
  time:Res<Time>,
){
  for (mut transform, velocity) in query {
    transform.translation += velocity.0 * time.delta_secs();
  }
}