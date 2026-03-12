use avian3d::prelude::{Forces, LinearVelocity, RigidBodyForces};
use bevy::{math::VectorSpace, prelude::*};

use crate::{bullet::{Bullet, BulletResources}, effect_sprite::EffectSpriteMessage};

pub struct WeaponsPlugin;
impl Plugin for WeaponsPlugin{
  fn build(&self, app: &mut App) {
    app.add_systems(Update, update_projectile_gun);
  }
}



#[derive(Component, Default)]
pub struct Weapon{
  pub trigger_active:bool,
}

#[derive(Component)]
pub struct ProjectileGun{
  pub firing:bool,
  fire_delay:Timer,
  cool_down:Timer,
}

impl ProjectileGun{
  pub fn new(fire_delay:f32, cool_down:f32)->Self{
    Self{ 
      firing:false,  
      fire_delay:Timer::from_seconds(fire_delay, TimerMode::Repeating),
      cool_down:Timer::from_seconds(cool_down,TimerMode::Once), 
    }
  }
}


fn update_projectile_gun(
  query:Query<(&Weapon, &mut ProjectileGun, &GlobalTransform, &ChildOf)>,
  
  parent_velocity_query:Query<&LinearVelocity, Without<Weapon>>,
  //mut parent_force_query:Query<Forces, Without<Weapon>>,
  time:Res<Time>,
  bullet_resources:Res<BulletResources>,
  mut commands:Commands,
){
  for (weapon, mut gun, transform, child_of) in query{
    gun.fire_delay.tick(time.delta());
    gun.cool_down.tick(time.delta());
    if !weapon.trigger_active{ 
      gun.firing = false;
      continue; 
    }
    if !gun.firing {
      if !gun.cool_down.is_finished(){ continue; }
      gun.fire_delay.finish();
      gun.firing = true;
    }
    if gun.fire_delay.is_finished(){ 

      let mut velocity = transform.up() * 80.;
      if let Ok(parent_velocity) = parent_velocity_query.get(child_of.0) {
        //forces.apply_local_force(  transform.up() * -8.);
        velocity += parent_velocity.0;
      };
      commands.spawn((
        Transform::from_translation(transform.translation()),
        Bullet::from_vector(velocity, child_of.0, 1.),
        Mesh3d(bullet_resources.bullet_mesh.clone()),
        MeshMaterial3d(bullet_resources.bullet_material.clone()),
      ));
      //apply forces
      /*
      if let Ok(mut forces) = parent_force_query.get_mut(child_of.0){
        forces.apply_linear_impulse(transform.up() * -8.);
      };
       */
      gun.cool_down.reset();
    }
  }
}