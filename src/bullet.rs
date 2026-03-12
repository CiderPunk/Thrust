use avian3d::prelude::{Forces, PhysicsSystems, RigidBodyForces, SpatialQuery, SpatialQueryFilter};
use bevy::{prelude::*, render::render_resource::encase::private::Length};

use crate::{effect_sprite::EffectSpriteMessage, game_schedule::GameSchedule};

const BULLET_COLOUR: LinearRgba = LinearRgba::new(2., 1.8, 0.2, 1.0);

pub struct BulletPlugin;

impl Plugin for BulletPlugin{
  fn build(&self, app: &mut App) {
    app
      .init_resource::<BulletResources>()
      .add_systems(Startup, init_bullets)
      .add_systems(Update, (update_bullets).in_set(PhysicsSystems::First));
  }
}

#[derive(Resource, Default)]
pub struct BulletResources{
  pub bullet_mesh:Handle<Mesh>,
  pub bullet_material:Handle<StandardMaterial>,
}


#[derive(Component)]
pub struct Bullet{
  direction:Dir3,
  speed:f32,
  time_to_live:Timer,
  owner:Entity,
}

impl Bullet{
  pub fn from_vector(vec:Vec3, owner:Entity, time_to_live_seconds:f32) -> Self{
    let direction = Dir3::new(vec).unwrap_or(Dir3::NEG_Z);
    let speed = vec.length().max(0.001);
    Self{ 
      direction, 
      speed, 
      time_to_live: Timer::from_seconds(time_to_live_seconds,TimerMode::Once), 
      owner 
    }
  }
}


fn update_bullets(
  query:Query<(&mut Bullet, &mut Transform, Entity)>,
  mut forces_query:Query<Forces>,
  time:Res<Time>,
  spatial_query:SpatialQuery,
  mut commands:Commands,
  mut effect_writer:MessageWriter<EffectSpriteMessage>,
){
  for (mut bullet, mut transform, entity) in query{
    bullet.time_to_live.tick(time.delta());
    if bullet.time_to_live.just_finished(){
      commands.entity(entity).despawn();
      continue;
    }
    let distance = bullet.speed * time.delta_secs();
    if let Some(hit) = spatial_query.cast_ray(transform.translation, bullet.direction, 
      distance, false, &SpatialQueryFilter::from_excluded_entities([bullet.owner])){

      if let Ok(mut forces) = forces_query.get_mut(hit.entity){
        let hit_location = transform.translation + (bullet.direction * hit.distance);
        forces.apply_linear_impulse_at_point(bullet.direction * bullet.speed, hit_location)

      }

      effect_writer.write(
        EffectSpriteMessage::new(
          "splosion".to_string(), 
          transform.translation + (bullet.direction * hit.distance),
          8.,
          Vec3::ZERO
        ));  
      commands.entity(entity).despawn(); 
    }
    else{
      transform.translation += bullet.direction * distance;
    }
  }
}

fn init_bullets(
  mut resources:ResMut<BulletResources>,
  mut meshes:ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
){
  resources.bullet_mesh = meshes.add( Sphere::new(0.25).mesh().ico(2).unwrap());
  resources.bullet_material = materials.add(StandardMaterial{
    emissive:BULLET_COLOUR,
    ..default()
  })
}