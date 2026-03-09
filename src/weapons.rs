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
  pub warm_up:Timer,
  pub cool_down:Timer,
  pub fire_rate:Timer,
  pub warm:bool,
}

impl Default for ProjectileGun{
  fn default() -> Self {
    Self { 
      fire_rate: Timer::from_seconds(0.5, TimerMode::Repeating), 
      warm:false,
      warm_up:Timer::from_seconds(0., TimerMode::Once),
      cool_down: Timer::from_seconds(0., TimerMode::Once)
    }
  }
}

fn update_projectile_gun(
  query:Query<(&Weapon, &mut ProjectileGun, &GlobalTransform)>,
  mut effect_writer:MessageWriter<EffectSpriteMessage>,
  time:Res<Time>,
){
  for (weapon, mut gun, transform) in query{
    gun.cool_down.tick(time.delta());
    if !gun.cool_down.is_finished(){ continue; }
    if !weapon.trigger_active { 
      if gun.warm {
        gun.cool_down.reset();
        gun.warm_up.reset();
      }
      continue;
    }
    gun.warm_up.tick(time.delta());
    if gun.warm_up.just_finished(){ 
      gun.warm = true;
      gun.fire_rate.finish();
    }
    if gun.warm{
      gun.fire_rate.tick(time.delta());
      if gun.fire_rate.just_finished(){
        info!("pew");
        effect_writer.write(EffectSpriteMessage::new("splosion".to_string(), transform.translation(), 8., Vec3::ZERO));
      }

    } 

  }

}