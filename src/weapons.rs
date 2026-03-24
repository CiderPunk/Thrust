use core::slice;

use avian3d::prelude::{Forces, RigidBodyForces, SpatialQueryFilter};
use bevy::{light::NotShadowCaster, math::VectorSpace, prelude::*};

use crate::{bullet::{Bullet, BulletResources}, game_physics::GameLayer};

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
  offset:Vec3,
  pub firing:bool,
  fire_delay:Timer,
  cool_down:Timer,
  filter:SpatialQueryFilter,
}

impl ProjectileGun{
  pub fn new(fire_delay:f32, cool_down:f32, filter:SpatialQueryFilter)->Self{
    Self{ 
      firing:false,  
      fire_delay:Timer::from_seconds(fire_delay, TimerMode::Repeating),
      cool_down:Timer::from_seconds(cool_down,TimerMode::Once), 
      offset: Vec3::ZERO,
      filter:filter,
    }
  }

  #[inline]
  #[must_use]
  pub const fn with_offset(mut self, offset: Vec3) -> Self {
      self.offset = offset;
      self
  }
}

#[derive(Component, Debug, PartialEq, Eq)]
#[relationship(relationship_target = WeaponAttachments)]
pub struct AttachedWeapon(pub Entity);

#[derive(Component, Default, Debug, PartialEq, Eq)]
#[relationship_target(relationship = AttachedWeapon, linked_spawn)]
pub struct WeaponAttachments(Vec<Entity>);


impl<'a> IntoIterator for &'a WeaponAttachments {
    type Item = <Self::IntoIter as Iterator>::Item;

    type IntoIter = slice::Iter<'a, Entity>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}



fn update_projectile_gun(
  query:Query<(&Weapon, &mut ProjectileGun, &GlobalTransform, &ChildOf)>,
  
  //parent_velocity_query:Query<&LinearVelocity, Without<Weapon>>,
  mut parent_force_query:Query<Forces, Without<Weapon>>,
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
      if let Ok(mut forces) = parent_force_query.get_mut(child_of.0){
        forces.apply_linear_impulse(transform.up() * -20.);
        velocity += forces.linear_velocity();
      };
      commands.spawn((
        NotShadowCaster,
        //Transform::from_translation(transform.translation() + gun.offset.y * transform.up() + gun.offset.x * transform.left()),
        Transform::from_translation(transform.translation()),
        Bullet::from_vector(velocity, child_of.0, 1.,10., gun.filter.clone() ),
        Mesh3d(bullet_resources.bullet_mesh.clone()),
        MeshMaterial3d(bullet_resources.bullet_material.clone()),
      ));
      gun.cool_down.reset();
    }
  }
}