use core::slice;
use std::f32::consts::PI;
use avian3d::prelude::*;
use bevy::{ecs::relationship::DescendantIter, gltf::GltfMesh, math::{FloatPow, VectorSpace}, prelude::*};
use crate::{asset_management::{AssetLoadState, GameAssets}, effect_sprite::{EFFECT_TYPE_SPLOSION, EffectSpriteMessage}, game_physics::GameLayer, game_state::GameState, get_gltf_primative, health::{Dead, Health, Hurtable}, player::Player, weapons::{AttachedWeapon, ProjectileGun, Weapon, WeaponAttachments}, wreckage::{Wreck, WreckResources}};



const TURRET_ACTIVATION_RANGE:f32 = 60.;
const TURRET_SEARCH_TIMER:f32 = 5.;
const DEPLOY_TIME: f32 = 0.5;
const TURRET_DEPLOY_HEIGHT:f32 = 6.2;
const TURRET_GIMBLE_TRACK_FACTOR: f32 = 2.5;
const TURRET_TOWER_TRACK_FACTOR:f32 = 5.;

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
        deploy_turret, 
        retract_turret, 
        search_timer,
        track_target,
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
  tower_collider:Option<Collider>,
  gimble_collider:Option<Collider>,
}

#[derive(Component, Default, Reflect, Debug)]
#[reflect(Component, Default)]
#[type_path = "api"]
struct TurretSpawn;


#[derive(Component)]
struct Turret;


#[derive(Component, Default)]
struct TurretTower;


#[derive(Component, Default)]
struct TurretGimble;

#[derive(Component, Default)]
struct TurretGun;



#[derive(Component)]
#[relationship(relationship_target = TurretComponents)]
struct TurretComponent(pub Entity);


#[derive(Component, Clone)]
#[relationship_target(relationship = TurretComponent, linked_spawn)]
struct TurretComponents(Vec<Entity>);

impl<'a> IntoIterator for &'a TurretComponents {
    type Item = <Self::IntoIter as Iterator>::Item;

    type IntoIter = slice::Iter<'a, Entity>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}


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
  gimble_stowed:bool,
}


fn init_turret_resources(
  mut turret_resources:ResMut<TurretResources>,
  game_assets: Res<GameAssets>,
  gltf_assets: Res<Assets<Gltf>>,
  gltf_meshes: Res<Assets<GltfMesh>>,
  meshes: Res<Assets<Mesh>>,
) -> Result<()> {
  info!("Init turret resources");
  let models = gltf_assets.get(&game_assets.models).ok_or("Couldn't get models")?;
 
  let base = get_gltf_primative!(gltf_meshes, models,"turret-base" );
  let tower = get_gltf_primative!(gltf_meshes, models,"turret-tower" );
  let gimble = get_gltf_primative!(gltf_meshes, models,"turret-gimble" );
  let gun = get_gltf_primative!(gltf_meshes, models,"turret-gun" );


  let base_collider =  get_gltf_primative!(gltf_meshes, models,"turret-base-collision" );
  let base_collider_mesh = meshes.get(&base_collider.mesh).clone().ok_or("Couldn't get base collision mesh")?;

  let tower_collider =  get_gltf_primative!(gltf_meshes, models,"turret-tower-collision" );
  let tower_collider_mesh = meshes.get(&tower_collider.mesh).clone().ok_or("Couldn't get tower collision mesh")?;

  let gimble_collider = get_gltf_primative!(gltf_meshes, models, "turret-gimble-collision");
  let gimble_collider_mesh = meshes.get(&gimble_collider.mesh).cloned().ok_or("Couldn't get gibmle collision mesh")?;

  turret_resources.turret_material = base.material.clone().ok_or("no flame material")?;

  turret_resources.base_mesh = base.mesh.clone();
  turret_resources.gimble_mesh = gimble.mesh.clone();
  turret_resources.tower_mesh = tower.mesh.clone();
  turret_resources.gun_mesh = gun.mesh.clone();

  turret_resources.base_collider =  Some(Collider::convex_hull_from_mesh(base_collider_mesh).ok_or("couldn't create base collider from mesh")?);
  turret_resources.tower_collider =  Some(Collider::convex_hull_from_mesh(tower_collider_mesh).ok_or("couldn't create tower collider from mesh")?);
  turret_resources.gimble_collider = Some(Collider::convex_hull_from_mesh(&gimble_collider_mesh).ok_or("couldn't creat gimble collider from mesh")?);
  Ok(())
}




fn spawn_turrets(
  query:Query<(&Transform, Entity), With<TurretSpawn>>,
  mut commands:Commands,
  turret_resources:Res<TurretResources>,
){
  info!("Spawning turrets");
  for (start_transform, placeholder_entity) in query.iter(){
    info!("Turret spawned");

    let turret = commands.spawn((
      Turret,
      Mesh3d(turret_resources.base_mesh.clone()),
      MeshMaterial3d(turret_resources.turret_material.clone()),
      start_transform.clone().with_scale(Vec3::splat(1.)),
      turret_resources.base_collider.clone().unwrap(),
      CollisionLayers::new([GameLayer::Enemy], [GameLayer::Player, GameLayer::Cargo]),
      RigidBody::Static,
      Health{ health:30. },
    )).observe(on_death).id();

    let tower = commands.spawn((
      TurretTower,
      ChildOf(turret),
      TurretComponent(turret),
      Mesh3d(turret_resources.tower_mesh.clone()),
      MeshMaterial3d(turret_resources.turret_material.clone()),
      Transform::from_translation(Vec3::new(0.,0.,0.)),
      turret_resources.tower_collider.clone().unwrap(),
      CollisionLayers::new([GameLayer::Enemy], [GameLayer::Cargo, GameLayer::Player]),
      RigidBody::Static,
      Hurtable,
    )).id();

    let gimble = commands.spawn((
      TurretGimble,
      ChildOf(tower),
      TurretComponent(turret),
      Mesh3d(turret_resources.gimble_mesh.clone()),
      MeshMaterial3d(turret_resources.turret_material.clone()),
      Transform::from_translation(Vec3::new(0.,0.,0.)),
    )).id();
      
    commands.spawn((
      TurretGun,
      ChildOf(gimble),
      TurretComponent(turret),
      Mesh3d (turret_resources.gun_mesh.clone()),
      MeshMaterial3d(turret_resources.turret_material.clone()),
      Transform::from_translation(Vec3::new(0.,0.,0.)),
      children![(
        TurretComponent(turret),
        Weapon{
          ..Default::default()
        },
        ProjectileGun::new(1.2, 1.2, SpatialQueryFilter::from_mask( GameLayer::Player.to_bits() | GameLayer::Cargo.to_bits() | GameLayer::Default.to_bits())),
        Transform::from_translation(Vec3::new(0.,-4.45,0.5)).with_rotation(Quat::from_axis_angle(Vec3::X, PI)),
        AttachedWeapon(turret),
      )],
    ));
    //remove placeholder
    commands.entity(placeholder_entity).despawn();


  }
}


fn check_target_proximity(
  turret_query:Query<(Entity, &GlobalTransform, Option<&Searching>), (With<Turret>, Without<Tracking>,  Without<Retract>)>,
  target_query: Query<(Entity, &GlobalTransform), With<Player>>,
  mut commands:Commands,
){
  for (turret, turret_transform, searching) in turret_query{
    for (player, player_transform) in target_query{
      if (turret_transform.translation() - player_transform.translation()).length_squared() < TURRET_ACTIVATION_RANGE.squared(){
        let mut turret = commands.entity(turret);
        turret
          .insert(Tracking{ target: player })
          .remove::<Searching>();

        if searching.is_none(){          
          turret.insert(
            Deploy{ timer:Timer::from_seconds(DEPLOY_TIME, TimerMode::Once )}
          );
        }
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
        .insert(Retract{ timer:Timer::from_seconds(DEPLOY_TIME, TimerMode::Once), gimble_stowed:false });
    }
  }
}

fn deploy_turret(
  query:Query<(&mut Deploy, &GlobalTransform, &TurretComponents, Option<&Tracking>, Entity), With<Turret>>,
  target_query:Query<&GlobalTransform>,
  mut tower_query:Query<&mut Transform, (With<TurretTower>,  Without<Turret>)>,
  time:Res<Time>,
  mut commands:Commands,
){
  for (mut deploy, turret_transform, components, tracking, entity) in query{
    deploy.timer.tick(time.delta());
    let tower_angle = match(tracking){
      Some(tracking) => {
        match target_query.get(tracking.target){
          Ok(target_transform) => {
            let target_vector = turret_transform.translation() - target_transform.translation();
            if target_vector.dot(turret_transform.left().into()) < 0.{  
              PI * 0.5 
            } else{ 
              -PI * 0.5 
            }
          },
          Err(_) => 0.,
        }
      },
      None => 0.,
    };
    for component in &components.0{
      if let Ok(mut transform) = tower_query.get_mut(*component){
        let fraction = deploy.timer.fraction();
        transform.translation.y = fraction * TURRET_DEPLOY_HEIGHT;
        transform.rotation = transform.rotation.slerp( Quat::from_axis_angle(Vec3::Y, tower_angle), time.delta_secs() * TURRET_TOWER_TRACK_FACTOR);
      }
    }
    if deploy.timer.is_finished(){
      commands.entity(entity).remove::<Deploy>();
    }
  }
}

fn retract_turret(
  query:Query<(&mut Retract,  Entity, &TurretComponents)>,
  mut gimble_query:Query<&mut Transform, (With<TurretGimble>,  Without<TurretTower>, Without<Turret>)>,
  mut tower_query:Query<&mut Transform, (With<TurretTower>, Without<TurretGimble>, Without<Turret>)>,
  time:Res<Time>,
  mut commands:Commands,
){
  for (mut deploy, entity, components) in query{
    if !deploy.gimble_stowed{
      for &component in &components.0 {
        if let Ok(mut transform) = gimble_query.get_mut(component) { 
          let target = Quat::from_axis_angle(Vec3::X, 0.);
          transform.rotation = transform.rotation.rotate_towards(target, time.delta_secs() * TURRET_GIMBLE_TRACK_FACTOR); 

          let diff = transform.rotation.angle_between(target);
          if diff < 0.01{
            deploy.gimble_stowed = true;  
          }
        }
      }
      continue;
    }
    deploy.timer.tick(time.delta());
    for &component in &components.0 {
      if let Ok(mut transform) = tower_query.get_mut(component) {
        transform.translation.y = (1.-deploy.timer.fraction()) * TURRET_DEPLOY_HEIGHT;
      }
    } 
    if deploy.timer.is_finished(){
      commands.entity(entity).remove::<Retract>();
    }
  }
}



/*
fn enable_weapon(
  query:Query<(&Transform, &Children), With<TurretGimble>>,
  mut weapon_query:Query<&mut Weapon>,
){
  for (transform, children) in query{

    for child in children{
      if let Ok(mut weapon) = weapon_query.get_mut(*child){
        weapon.trigger_active = transform.rotation.angle_between(Quat::from_axis_angle(Vec3::X, 0.)) > 1.0;
      }
    }
  }
}
 */


fn track_target(
  turret_query:Query<(Entity, &GlobalTransform, &Tracking, &TurretComponents, &WeaponAttachments), (With<Turret>, With<Tracking>, Without<Deploy>, Without<Retract>)>,
  mut tower_query:Query<&mut Transform, (With<TurretTower>, Without<TurretGimble>, Without<Turret>)>,
  mut gimble_query:Query<&mut Transform, (With<TurretGimble>,  Without<TurretTower>, Without<Turret>)>,
  target_query: Query<&GlobalTransform>,
  mut commands:Commands,
  time:Res<Time>,
  mut weapon_query:Query<&mut Weapon>,
){
 for (turret, turret_transform, tracking, components, weapons) in turret_query{
    if let Ok(target_transform) = target_query.get(tracking.target){
      let gun_translation = turret_transform.translation() + (turret_transform.up() * 5.2);
      let target_vector = gun_translation - target_transform.translation();
      //check for out of range
      if target_vector.length_squared() > TURRET_ACTIVATION_RANGE.squared(){
        commands.entity(turret)
          .remove::<Tracking>()
          .insert( 
            Searching{ search_timer: Timer::from_seconds(TURRET_SEARCH_TIMER, TimerMode::Once) }
          );
          for weapon in weapons.into_iter(){
            if let Ok(mut weapon)= weapon_query.get_mut(*weapon){
              weapon.trigger_active = false;
            }
          }
        continue;
      }
      
      let is_left = target_vector.dot(turret_transform.left().into()) < 0.;
      let gimble_angle = target_vector.angle_between(turret_transform.up().into());
      let tower_angle = if is_left {  PI * 0.5 } else{ -PI * 0.5 };
      for &component in &components.0 {
        if let Ok(mut transform) = gimble_query.get_mut(component) {
          transform.rotation = transform.rotation.rotate_towards(Quat::from_axis_angle(Vec3::X, gimble_angle), time.delta_secs() * TURRET_GIMBLE_TRACK_FACTOR);
        }
        if let Ok(mut transform) = tower_query.get_mut(component) {
          transform.rotation = transform.rotation.rotate_towards(Quat::from_axis_angle(Vec3::Y, tower_angle), time.delta_secs() * TURRET_TOWER_TRACK_FACTOR);
        }
      } 

      for weapon in weapons.into_iter(){
        if let Ok(mut weapon)= weapon_query.get_mut(*weapon){
          weapon.trigger_active = gimble_angle > 0.8;
        }
      }

    }
  }
}



fn on_death(
  event:On<Add, Dead>,
  mut query:Query<(&mut Dead, &Transform, &TurretComponents, &WeaponAttachments)>,
  gimble_query:Query<(Entity,&GlobalTransform),With<TurretGimble>>,
  tower_query:Query<Entity,With<TurretTower>>, 
  mut commands:Commands,
  wreck_resources:Res<WreckResources>,
  turret_resources:Res<TurretResources>,
  mut effect_writer:MessageWriter<EffectSpriteMessage>,
){
  if let Ok((mut dead, transform, components, weapons)) = query.get_mut(event.entity){
    dead.timer = Timer::from_seconds(5., TimerMode::Once);
    effect_writer.write(EffectSpriteMessage::new(EFFECT_TYPE_SPLOSION.to_string(), transform.translation, 20., Vec3::ZERO));
    commands.entity(event.entity).remove::<(Turret, Dead, Health)>();


    for &component in &components.0{
      if let Ok((gimble,transform)) = gimble_query.get(component) {
        //effect
        effect_writer.write(EffectSpriteMessage::new(EFFECT_TYPE_SPLOSION.to_string(), transform.translation(), 16., Vec3::ZERO));
        //get rid of old gimble
        commands.entity(gimble).despawn();
        //spawn some wreckage
        commands.spawn((
          Wreck::new(2.0, 0.6, 12., EFFECT_TYPE_SPLOSION.to_string()),
          Mesh3d(turret_resources.gimble_mesh.clone()),
          MeshMaterial3d(wreck_resources.wreck_material.clone()),
          transform.compute_transform(),
          CollisionLayers::new([GameLayer::Default], [GameLayer::Default]),
          ColliderDensity(0.1),
          RigidBody::Dynamic,
          turret_resources.gimble_collider.clone().unwrap(),
          children![(
            Mesh3d (turret_resources.gun_mesh.clone()),
            MeshMaterial3d(wreck_resources.wreck_material.clone()),
            Transform::from_translation(Vec3::ZERO),
          )],
        ));  
      };
      if let Ok(tower) = tower_query.get(component){
        commands.entity(tower)
          .insert((
            Wreck::new(1., 0.4, 12., EFFECT_TYPE_SPLOSION.to_string()),
            MeshMaterial3d(wreck_resources.wreck_material.clone())
          ))
          .remove::<Hurtable>();
      };
    }
  };
}