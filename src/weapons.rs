use bevy::{math::VectorSpace, prelude::*};

use crate::effect_sprite::EffectSpriteMessage;

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
  query:Query<(&Weapon, &mut ProjectileGun, &GlobalTransform)>,
  mut effect_writer:MessageWriter<EffectSpriteMessage>,
  time:Res<Time>,
){
  for (weapon, mut gun, transform) in query{
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
      effect_writer.write(EffectSpriteMessage::new("splosion".to_string(), transform.translation(), 20., Vec3::ZERO));
      gun.cool_down.reset();
    }
  }

}