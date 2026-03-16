use std::f32::consts::PI;
use avian3d::prelude::Collider;
use bevy::{gltf::GltfMesh, math::FloatPow, prelude::*};
use crate::{asset_management::{AssetLoadState, GameAssets}, game_state::GameState, get_gltf_primative, health::{Health, Hurtable}, player::Player};



const TURRET_ACTIVATION_RANGE:f32 = 60.;
const TURRET_SEARCH_TIMER:f32 = 5.;
const DEPLOY_TIME: f32 = 0.5;


pub struct TurretPlugin;

impl Plugin for TurretPlugin{
  fn build(&self, app: &mut App) {
    app
      .insert_resource(TurretResources{ 
        ..default()
      })  
      .add_systems(OnEnter(AssetLoadState::Loaded), init_turret_resources)
      .add_systems(OnEnter(GameState::Initialize), spawn_turrets)
      .add_systems(Update, (
        check_target_proximity,
        check_target_escape, 
        deploy_turret, 
        retract_turret, 
        search_timer
      ));
  }
}





#[derive(Resource, Default)]
struct TurretResources{
  turret_material:Handle<StandardMaterial>,
  base_mesh:Handle<Mesh>,
  tower_mesh:Handle<Mesh>,
  gimble_mesh:Handle<Mesh>,
  gun_mesh:Handle<Mesh>,
  base_collider:Option<Collider>,
  tower_collider:Option<Collider>
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


#[derive(Component)]
struct Tracking{
  target:Entity,
}


#[derive(Component)]
struct Searching{
  search_timer:Timer,
}


#[derive(Component, Default)]
struct Deploy{
  timer:Timer,
}

#[derive(Component, Default)]
struct Retract{
  timer:Timer,
}


fn init_turret_resources(
  mut turret_resources:ResMut<TurretResources>,
  game_assets: Res<GameAssets>,
  gltf_assets: Res<Assets<Gltf>>,
  gltf_meshes: Res<Assets<GltfMesh>>,
  mut meshes: ResMut<Assets<Mesh>>,
) -> Result<()> {
  info!("Init turret resources");
  let models = gltf_assets.get(&game_assets.models).ok_or("Couldn't get models")?;
 
  let base = get_gltf_primative!(gltf_meshes, models,"turret-base" );
  let tower = get_gltf_primative!(gltf_meshes, models,"turret-tower" );
  let gimble = get_gltf_primative!(gltf_meshes, models,"turret-gimble" );
  let gun = get_gltf_primative!(gltf_meshes, models,"turret-gun" );


  let base_collider =  get_gltf_primative!(gltf_meshes, models,"turret-base-collision" );
  let base_collider_mesh = meshes.get(&base_collider.mesh).clone().ok_or("Couldn't get collision mesh")?;

  let tower_collider =  get_gltf_primative!(gltf_meshes, models,"turret-tower-collision" );
  let tower_collider_mesh = meshes.get(&tower_collider.mesh).clone().ok_or("Couldn't get collision mesh")?;

  turret_resources.turret_material = base.material.clone().ok_or("no flame material")?;

  turret_resources.base_mesh = base.mesh.clone();
  turret_resources.gimble_mesh = gimble.mesh.clone();
  turret_resources.tower_mesh = tower.mesh.clone();
  turret_resources.gun_mesh = gun.mesh.clone();

  turret_resources.base_collider =  Some(Collider::convex_hull_from_mesh(base_collider_mesh).ok_or("couldn't create collider from mesh")?);
  turret_resources.tower_collider =  Some(Collider::convex_hull_from_mesh(tower_collider_mesh).ok_or("couldn't create collider from mesh")?);

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
      turret_resources.base_collider.clone().unwrap(),
      Health{ health:20. },
      children![
        (
          TurretTower,
          Mesh3d(turret_resources.tower_mesh.clone()),
          MeshMaterial3d(turret_resources.turret_material.clone()),
          Transform::from_translation(Vec3::new(0.,2.,0.)),
          turret_resources.tower_collider.clone().unwrap(),
          Hurtable,
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
  turret_query:Query<(Entity, &GlobalTransform, Option<&Searching>), (With<Turret>, Without<Tracking>)>,
  target_query: Query<(Entity, &GlobalTransform), With<Player>>,
  mut commands:Commands,
){
  for (turret, turret_transform, searching) in turret_query{
    for (player, player_transform) in target_query{
      if (turret_transform.translation() - player_transform.translation()).length_squared() < TURRET_ACTIVATION_RANGE.squared(){
        let mut turret = commands.entity(turret);
        turret.insert(
          Tracking{ target: player },
        );
        if searching.is_none(){          
          turret.insert(
            Deploy{ timer:Timer::from_seconds(DEPLOY_TIME, TimerMode::Once )}
          );
        }
      }
    }
  }
}

fn check_target_escape(
  turret_query:Query<(Entity, &GlobalTransform, &Tracking), (With<Turret>, With<Tracking>, Without<Deploy>)>,
  target_query: Query<&GlobalTransform>,
  mut commands:Commands,
){
  for (turret, turret_transform, tracking) in turret_query{
    if let Ok(target_transform) = target_query.get(tracking.target){
      if (turret_transform.translation() - target_transform.translation()).length_squared() > TURRET_ACTIVATION_RANGE.squared(){
        commands.entity(turret)
          .remove::<Tracking>()
          .insert( 
            Searching{ search_timer: Timer::from_seconds(TURRET_SEARCH_TIMER, TimerMode::Once) }
          );
      }
    }
  }
}

fn search_timer(
  query:Query<(&mut Searching, Entity)>,
  time:Res<Time>,
  mut commands:Commands,
){
  for (mut searcher, entity) in query{
    searcher.search_timer.tick(time.delta());
    if searcher.search_timer.is_finished(){
      commands.entity(entity)
        .remove::<Searching>()
        .insert(Retract{ timer:Timer::from_seconds(DEPLOY_TIME, TimerMode::Once)});
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
    deploy.timer.tick(time.delta());
    for child in children{
      if let Ok(mut transform) = tower_query.get_mut(*child){
        let fraction = deploy.timer.fraction();
        transform.translation.y = (fraction * 5.2) + 2.;
        transform.rotation = Quat::from_axis_angle(Vec3::Y, -0.5 * PI *fraction );
      }
    }
    if deploy.timer.is_finished(){
      commands.entity(entity).remove::<Deploy>();
    }
  }
}

fn retract_turret(
  query:Query<(&mut Retract,  Entity, &Children)>,
  mut tower_query:Query<&mut Transform, With<TurretTower>>,
  time:Res<Time>,
  mut commands:Commands,
){
  for (mut deploy, entity, children) in query{
    deploy.timer.tick(time.delta());
    for child in children{
      if let Ok(mut transform) = tower_query.get_mut(*child){
        let fraction = deploy.timer.fraction();
        transform.translation.y = ((1.-fraction) * 5.2) + 2.;
        transform.rotation = Quat::from_axis_angle(Vec3::Y, -0.5 * PI *fraction );
      }
    }
    if deploy.timer.is_finished(){
      commands.entity(entity).remove::<Retract>();
    }
  }
}