use avian3d::prelude::LinearVelocity;
use bevy::{gltf::GltfMesh, prelude::*};

use crate::{asset_management::{AssetLoadState, GameAssets}, effect_sprite::EffectSpriteMessage, get_gltf_primative, movement::Velocity, wreckage};

pub struct WreckagePlugin;

impl Plugin for WreckagePlugin{
  fn build(&self, app: &mut App) {
    app.insert_resource(WreckResources::default())
      .add_systems(OnEnter(AssetLoadState::Loaded), init_wreck_reosurces)
      .add_systems(Update, wreck_update);
  }
}

#[derive(Resource, Default)]
pub struct WreckResources{
  pub wreck_material: Handle<StandardMaterial>,
}

#[derive(Component)]
pub struct Wreck{
  ttl:Timer,
  effect_timer:Timer,
  effect_scale:f32,
  effect_name:String,
}

impl Wreck{
  pub fn new(ttl:f32, effect_timer:f32, effect_scale:f32, effect_name:String)->Self{
    Self{ 
      ttl: Timer::from_seconds(ttl, TimerMode::Once),
      effect_timer: Timer::from_seconds(effect_timer, TimerMode::Repeating),
      effect_scale,
      effect_name,
    }
  }
}

fn init_wreck_reosurces(
  mut wreck_resources: ResMut<WreckResources>,
  game_assets: Res<GameAssets>,
  gltf_assets: Res<Assets<Gltf>>,
  gltf_meshes: Res<Assets<GltfMesh>>,
 // mut meshes: ResMut<Assets<Mesh>>,
) -> Result<()> {
  let models = gltf_assets.get(&game_assets.models).ok_or("Couldn't get models")?;
  let wreck_cube = get_gltf_primative!(gltf_meshes, models,"wreck-cube" );
  wreck_resources.wreck_material = wreck_cube.material.clone().ok_or("Missing wreck material")?;
  Ok(())
}

fn wreck_update(
  query:Query<(&mut Wreck, &GlobalTransform, Option<&LinearVelocity>, Entity,)>,
  time:Res<Time>,
  mut effect_writer:MessageWriter<EffectSpriteMessage>,
  mut commands:Commands,
){
  
  for (mut wreck, transform, velocity, entity) in query{
    wreck.effect_timer.tick(time.delta());
    if wreck.effect_timer.just_finished(){
      effect_writer.write(
        EffectSpriteMessage::new(
          wreck.effect_name.clone(), 
          transform.translation(), 
          wreck.effect_scale, 
          match velocity{
            Some(velocity) => **velocity,
            None => Vec3::ZERO,
          },
        ));
    }
    wreck.ttl.tick(time.delta());
    if wreck.ttl.is_finished()
    {
      commands.entity(entity).despawn();
    }

  } 
}