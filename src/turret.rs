use std::f32::consts::PI;

use bevy::{ecs::system::command, gltf::GltfMesh, math::VectorSpace, prelude::*};
use serde::de;

use crate::{asset_management::{AssetLoadState, GameAssets}, game_state::GameState, get_gltf_primative, player::Player};
pub struct TurretPlugin;

impl Plugin for TurretPlugin{
  fn build(&self, app: &mut App) {
    app
      .insert_resource(TurretResources{ 
        ..default()
      })  
      .add_systems(OnEnter(AssetLoadState::Loaded), init_turret_resources)
      .add_systems(OnEnter(GameState::Initialize), spawn_turrets)
      .add_systems(Update, (check_target_proximity, deploy_turret));
  }
}


#[derive(Resource, Default)]
struct TurretResources{
  turret_material:Handle<StandardMaterial>,
  base_mesh:Handle<Mesh>,
  tower_mesh:Handle<Mesh>,
  gimble_mesh:Handle<Mesh>,
  gun_mesh:Handle<Mesh>,
}

#[derive(Component, Default, Reflect, Debug)]
#[reflect(Component, Default)]
#[type_path = "api"]
struct TurretSpawn;


#[derive(Component, Default)]
struct Turret{}


#[derive(Component, Default)]
struct TurretTower;


#[derive(Component, Default)]
struct TurretGimble;


#[derive(Component, Default)]
struct Tracking{
  target:Option<Entity>,
}


#[derive(Component, Default)]
struct Deploy{
  extend_timer:Timer,
}

fn init_turret_resources(
  mut turret_resources:ResMut<TurretResources>,
  game_assets: Res<GameAssets>,
  gltf_assets: Res<Assets<Gltf>>,
  gltf_meshes: Res<Assets<GltfMesh>>,
  //mut meshes: ResMut<Assets<Mesh>>,
) -> Result<()> {
  info!("Init turret resources");
  let models = gltf_assets.get(&game_assets.models).ok_or("Couldn't get models")?;
 
  let base = get_gltf_primative!(gltf_meshes, models,"turret-base" );
  let tower = get_gltf_primative!(gltf_meshes, models,"turret-tower" );
  let gimble = get_gltf_primative!(gltf_meshes, models,"turret-gimble" );
  let gun = get_gltf_primative!(gltf_meshes, models,"turret-gun" );

  turret_resources.turret_material = base.material.clone().ok_or("no flame material")?;

  turret_resources.base_mesh = base.mesh.clone();
  turret_resources.gimble_mesh = gimble.mesh.clone();
  turret_resources.tower_mesh = tower.mesh.clone();
  turret_resources.gun_mesh = gun.mesh.clone();
  Ok(())
}


fn spawn_turrets(
  query:Query<&Transform, With<TurretSpawn>>,
  mut commands:Commands,
  turret_resources:Res<TurretResources>,
){
  info!("Spawning turrets");
  for start_transform in query.iter(){
    info!("Turret spawned");
    commands.spawn((
      Turret{
        ..default()
      },
      Mesh3d(turret_resources.base_mesh.clone()),
      MeshMaterial3d(turret_resources.turret_material.clone()),
      start_transform.clone().with_scale(Vec3::splat(1.)),
      children![
        (
          TurretTower,
          Mesh3d(turret_resources.tower_mesh.clone()),
          MeshMaterial3d(turret_resources.turret_material.clone()),
          Transform::from_translation(Vec3::new(0.,2.,0.)),
          children![
            (
              TurretGimble,
              Mesh3d(turret_resources.gimble_mesh.clone()),
              MeshMaterial3d(turret_resources.turret_material.clone()),
              Transform::from_translation(Vec3::new(0.,0.,0.)),
              children![
                (
                  Mesh3d (turret_resources.gun_mesh.clone()),
                  MeshMaterial3d(turret_resources.turret_material.clone()),
                  Transform::from_translation(Vec3::new(0.,0.,0.)),
                )
              ]
            )
          ]
        ),
      ]
    ));
  }
}

fn check_target_proximity(
  turret_query:Query<(Entity, &GlobalTransform), (With<Turret>, Without<Tracking>)>,
  target_query: Query<(Entity, &GlobalTransform), With<Player>>,
  mut commands:Commands,
){
  for (turret, turret_transform) in turret_query{
    for (player, player_transform) in target_query{
      if (turret_transform.translation() - player_transform.translation()).length_squared() < (40. * 40.){
        commands.entity(turret).insert((
          Tracking{ target: Some(player) },
          Deploy{ extend_timer:Timer::from_seconds(0.5, TimerMode::Once )}
        ));
      }
    }
  }
}

fn deploy_turret(
  query:Query<(&mut Deploy,  Entity, &Children)>,
  mut tower_query:Query<&mut Transform, With<TurretTower>>,
  time:Res<Time>,
  mut commands:Commands,
){
  for (mut deploy, entity, children) in query{
    deploy.extend_timer.tick(time.delta());
    for child in children{
      if let Ok(mut transform) = tower_query.get_mut(*child){
        let fraction = deploy.extend_timer.fraction();
        transform.translation.y = (fraction * 5.5) + 2.;
        transform.rotation = Quat::from_axis_angle(Vec3::Y, -0.5 * PI *fraction );
      }
    }


    if deploy.extend_timer.is_finished(){
      commands.entity(entity).remove::<Deploy>();
    }

  }
}