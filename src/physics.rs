use avian3d::prelude::PhysicsLayer;
use bevy::prelude::*;

#[derive(PhysicsLayer, Default)]
pub enum GameLayer{
  #[default]
  Default,
  Player,
  Cargo,
  Bullets,
}