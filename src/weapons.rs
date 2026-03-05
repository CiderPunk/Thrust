use bevy::prelude::*;

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
  warm_up:Timer,
  cool_down:Timer,
  fire_rate:Timer,
}

impl Default for ProjectileGun{
  fn default() -> Self {
    Self { 
      fire_rate: Timer::from_seconds(0.5, TimerMode::Repeating), 
      warm_up:Timer::from_seconds(0., TimerMode::Once),
      cool_down: Timer::from_seconds(0., TimerMode::Once)
    }
  }
}

fn update_projectile_gun(
  query:Query<(&Weapon, &mut ProjectileGun, &GlobalTransform)>,
){
  for (weapon, gun, transform) in query{
    if weapon.trigger_active{
      info!("pew!");
    }
  }

}