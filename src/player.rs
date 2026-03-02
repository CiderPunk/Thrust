use core::f32;

use avian3d::prelude::{AngularDamping, Collider, CollisionLayers, DistanceJoint, Forces, LockedAxes, MaxAngularSpeed, RigidBody, RigidBodyForces, SpatialQuery, SpatialQueryFilter, TransformInterpolation};
use bevy::{color::palettes::css::WHITE, ecs::schedule::graph::Direction, gltf::GltfMesh, light::NotShadowCaster, prelude::*, render::view::visibility, time::Stopwatch};
use bevy_enhanced_input::prelude::{Release, *};
use crate::{asset_management::{AssetLoadState, GameAssets}, game_schedule::GameSchedule, game_state::GameState, get_gltf_primative, physics::GameLayer, shaders::ShaderMaterials};


const PLAYER_THRUST: f32 = 200.;


pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(PlayerResources{
        ..Default::default()
      })
      .add_systems(OnEnter(AssetLoadState::Loaded), init_player_reosurces)
      .add_systems(OnEnter(GameState::Initialize), spawn_player)
      .add_systems(Update, animate_flame)
      .add_systems(Update, ( cargo_scan, tether_update ).in_set(GameSchedule::EntityUpdates))
      .add_observer(on_remove_tether)
      .add_observer(player_yaw)
      .add_observer(player_thrust)
      .add_observer(player_thrust_release)
      .add_observer(player_shield_activate)
      .add_observer(player_shield_release)
      .add_input_context::<Player>();
  }
}


#[derive(Debug, InputAction)]
#[action_output(f32)]
struct Yaw;

#[derive(InputAction)]
#[action_output(bool)]
struct Thrust;

#[derive(InputAction)]
#[action_output(bool)]
struct Shoot;


#[derive(InputAction)]
#[action_output(bool)]
struct ActivateShield;


#[derive(Component)]
pub struct Player{
  shield_active:bool,
  cargo_scan_timer:Timer,
}

#[derive(Component)]
struct Shield;


#[derive(Component, Default)]
pub struct PlayerFlame{
  ignite_time:Stopwatch,
}


#[derive(Component)]
pub struct PlayerLight;

#[derive(Component, Default, Reflect, Debug)]
#[reflect(Component, Default)]
#[type_path = "api"]
struct PlayerStart;




#[derive(Component, Debug)]
struct Tether{
  target:Entity,
  joint:Option<Entity>,
}



#[derive(Resource, Default)]
struct PlayerResources{
  collider: Option<Collider>,
  ship_mesh: Handle<Mesh>,
  ship_material: Handle<StandardMaterial>,
  flame_mesh: Handle<Mesh>,
  flame_material: Handle<StandardMaterial>,
  shield_mesh: Handle<Mesh>,
}


fn init_player_reosurces(
  mut player_resources: ResMut<PlayerResources>,
  game_assets: Res<GameAssets>,
  gltf_assets: Res<Assets<Gltf>>,
  gltf_meshes: Res<Assets<GltfMesh>>,
  mut meshes: ResMut<Assets<Mesh>>,
  //materials: Res<Assets<StandardMaterial>>,
) -> Result<()> {
  let models = gltf_assets.get(&game_assets.models).ok_or("Couldn't get models")?;

  let collision_primative = get_gltf_primative!(gltf_meshes, models,"ship-collision" );
  let display_primative = get_gltf_primative!(gltf_meshes, models, "ship-display" );
  let flame_primative = get_gltf_primative!(gltf_meshes, models, "ship-flame" );
 
  player_resources.flame_mesh = flame_primative.mesh.clone();
  player_resources.flame_material = flame_primative.material.clone().ok_or("no flame material")?;
  player_resources.ship_mesh = display_primative.mesh.clone(); 
  player_resources.ship_material = display_primative.material.clone().ok_or("no ship material")?;
  let collision_mesh = meshes.get(&collision_primative.mesh).clone().ok_or("Couldn't get collision mesh")?;
  player_resources.collider = Some(Collider::convex_hull_from_mesh(collision_mesh).ok_or("couldn't create collider from mesh")?);
  player_resources.shield_mesh = meshes.add(Sphere::default().mesh().uv(16, 8));
  Ok(())
}

fn spawn_player(
  query: Query<&Transform, With<PlayerStart>>,
  mut commands: Commands,
  player_resources: Res<PlayerResources>,
  shader_materials: Res<ShaderMaterials>,
 ){
  for start_transform in query.iter() {
    commands.spawn((
      Player{ 
        shield_active: false, 
        cargo_scan_timer: Timer::from_seconds(0.2, TimerMode::Repeating),
      },
      Mesh3d(player_resources.ship_mesh.clone()),
      MeshMaterial3d(player_resources.ship_material.clone()),
      start_transform.clone(),
      RigidBody::Dynamic,
      MaxAngularSpeed(2.0),
      AngularDamping(20.0),
      TransformInterpolation,
      LockedAxes::new().lock_rotation_y().lock_rotation_x().lock_translation_z(),
      CollisionLayers::new([GameLayer::Player, GameLayer::Default], [GameLayer::Default]),
      player_resources.collider.clone().unwrap(),
      //NotShadowCaster,
      actions!(Player[
        (
          Action::<Yaw>::new(),
          DeadZone::default(),
          SmoothNudge::default(),
          Scale::splat(800.0),
          Bindings::spawn((
            Bidirectional::new(KeyCode::KeyD, KeyCode::KeyA),
            Bidirectional::new(KeyCode::ArrowRight, KeyCode::ArrowLeft),
            Bidirectional::new(GamepadButton::DPadRight,  GamepadButton::DPadLeft),
            Axial::left_stick(),
          )),
        ),
        (
          Action::<Thrust>::new(),
          bindings![KeyCode::ArrowUp, KeyCode::KeyW, GamepadButton::DPadUp],
        ),
        (
          Action::<ActivateShield>::new(),
          bindings![KeyCode::KeyF, KeyCode::ControlRight, KeyCode::ControlLeft],
        )
      ]),
      children![
        (
          PointLight {
            intensity: 3_000_000.0,
            range: 50.,
            color: WHITE.into(),
            shadows_enabled: true,
            shadow_map_near_z: 2.0,
            ..default()
          },
          Transform::from_xyz(0.,0.,0.),
          PlayerLight,
        ),
        (  
          PlayerFlame{ ignite_time:Stopwatch::default() } ,
          Mesh3d(player_resources.flame_mesh.clone()),
          MeshMaterial3d(player_resources.flame_material.clone()),
          NotShadowCaster,
        ),
        (
          Shield,
          Visibility::Hidden,
          Mesh3d(player_resources.shield_mesh.clone()),
          MeshMaterial3d(shader_materials.shield.clone()),
          Transform::from_scale(Vec3::splat(6.))
        )
      ],
    
    ));
  }
}

fn player_yaw(
  yaw:On<Fire<Yaw>>,
  mut forces_query:Query<Forces>,
){
  let mut forces = forces_query.get_mut(yaw.context).unwrap();
  //info!("Yaw {}", yaw.value);
  forces.apply_torque(Vec3::new(0.,0.,-yaw.value));
}

fn player_thrust(
  thrust:On<Fire<Thrust>>,
  mut forces_query:Query<Forces>,
  flame_visiblity:Single<&mut Visibility, With<PlayerFlame>>,
){
  let mut flame = flame_visiblity.into_inner();
  let mut forces = forces_query.get_mut(thrust.context).unwrap();
  if thrust.value{
    forces.apply_local_force(Vec3::new(0.,PLAYER_THRUST, 0.));
    *flame = Visibility::Visible;     
  }
}

fn player_thrust_release(
  _:On<Complete<Thrust>>,
  flame_visiblity:Single<(&mut Visibility, &mut PlayerFlame)>,
){
  let (mut visible, mut flame) = flame_visiblity.into_inner();
  flame.ignite_time.reset();
  *visible = Visibility::Hidden;     
}

fn player_shield_activate(
  shield:On<Start<ActivateShield>>,
  player_query:Query<(&mut Player, &Children)>,
  shield_query:Query<&mut Visibility, With<Shield>>,
){
  toggle_shield(shield.context, true, player_query, shield_query);
}

fn player_shield_release(
  shield:On<Complete<ActivateShield>>,
  player_query:Query<(&mut Player, &Children)>,
  shield_query:Query<&mut Visibility, With<Shield>>,
){
  toggle_shield(shield.context, false, player_query, shield_query);
}


fn toggle_shield(
  entity:Entity,
  shield_state:bool,
  mut player_query:Query<(&mut Player, &Children)>,
  mut shield_query:Query<&mut Visibility, With<Shield>>,
){
   let Ok((mut player, children)) = player_query.get_mut(entity) else{
    return;
  };
  player.shield_active = shield_state;
  if shield_state{
    player.cargo_scan_timer.reset();
  }

  for child in children{
    if let Ok(mut visible) = shield_query.get_mut(*child){
      *visible = if shield_state { Visibility::Visible } else { Visibility::Hidden };
    }
  }
}

fn animate_flame(
  flame: Single<(&mut Transform,  &mut PlayerFlame, &Visibility)>,
  light: Single<&mut PointLight,  With<PlayerLight>>,
  time:Res<Time>,
){
  let (mut transform, mut flame, visibility ) = flame.into_inner();
  let mut light = light.into_inner();
  if visibility == Visibility::Visible{
    flame.ignite_time.tick(time.delta());
    let  scale = 0.5 + ((flame.ignite_time.elapsed_secs() * 10.).sin().abs() *0.8) - flame.ignite_time.elapsed_secs().min(0.2); 
    transform.scale = Vec3::splat(scale);
    light.intensity =1_000_000.0  + (2_000_000. * scale);
  }
  else{
    light.intensity = 1_000_000.0;
  }
}


const TETHER_DISTANCE: f32 = 14.;
const TETHER_START_DISTANCE: f32 = 10.;
const TETHER_MAX_DISTANCE: f32 = 20.;
const TETHER_MIN_DISTANCE: f32 = 2.;


fn on_remove_tether(
  trigger:On<Remove, Tether>,
  query:Query<&Tether>,
  mut commands:Commands,
){
  if let Ok(tether) = query.get(trigger.entity){
    if let Some(join) = tether.joint{
      commands.entity(join).despawn();
    }
  }
}



fn tether_update(
  query:Query<(&Player, &mut Tether, &GlobalTransform, Entity)>,
  target_query:Query<&GlobalTransform>,
  spatial_query:SpatialQuery,
  mut commands:Commands,
){
  for (player, mut tether, player_transform, player_entity) in query{
    let Ok(target_transform) = target_query.get(tether.target) else{ 
      info!("Tether target lost");
      commands.entity(player_entity).remove::<Tether>();
      continue;
    };

    //player cancelled tether
    if !player.shield_active &&  tether.joint.is_none(){
      info!("Tether cancelled");
      commands.entity(player_entity).remove::<Tether>();
      continue;
    }

    let player_translation = player_transform.translation();
    //check target visiblity
    let delta = target_transform.translation() - player_translation;
    let direction = Dir3::new_unchecked(delta.normalize());
    if  let Some(hit) = spatial_query.cast_ray(player_translation, direction, TETHER_MAX_DISTANCE, false,  &SpatialQueryFilter::from_excluded_entities([player_entity])){
      if hit.entity == tether.target{
        //check if we;'ve moved away further than the tether distanbce and can start actual lifting
        if (tether.joint.is_none() && hit.distance > TETHER_START_DISTANCE){
          //create physics constraint
          tether.joint = Some(commands.spawn(DistanceJoint::new(player_entity, tether.target).with_min_distance(TETHER_MIN_DISTANCE).with_max_distance(TETHER_DISTANCE)).id());
        }
        continue;
      }

    }
    info!("Tether obstruction");
    commands.entity(player_entity).remove::<Tether>();
    continue;
  }
}



fn cargo_scan(
  query:Query<(&mut Player, &GlobalTransform, Entity), Without<Tether>>,
  transform_query:Query<&GlobalTransform>,
  time:Res<Time>,
  spatial_query:SpatialQuery,
  mut commands:Commands,
){
  for (mut player, transform, player_entity) in query{
    if player.shield_active{
      player.cargo_scan_timer.tick(time.delta());
      if player.cargo_scan_timer.is_finished(){

        let player_translation = transform.translation();
        let mut nearest_target:Option<Entity> = None;
        let mut nearest_distance_squared = f32::MAX;
        //do scan!

        spatial_query.shape_intersections_callback(&Collider::sphere(TETHER_DISTANCE), transform.translation(), Quat::default(), &SpatialQueryFilter::from_mask(GameLayer::Cargo), |entity| { 

          if let Ok(target_transform) = transform_query.get(entity) {
            let delta = target_transform.translation()- player_translation;
            let distance_squared = delta.length_squared();
             //check if we're nearest
            if distance_squared < nearest_distance_squared{
              let direction_to_target = Dir3::new_unchecked(delta / distance_squared.sqrt());
              //test there's a direct line of sight to the target
              if let Some(hit) = spatial_query.cast_ray(player_translation, direction_to_target, TETHER_DISTANCE, false, &SpatialQueryFilter::from_excluded_entities([player_entity])){
                if hit.entity == entity{
                  nearest_target = Some(entity);
                  nearest_distance_squared = distance_squared;
                }
              };
            }
          }; 
          true
        });

        if nearest_target.is_some() {
          //target aquired
          info!("Tether target: {}  distance: {}", nearest_target.unwrap(), nearest_distance_squared);
          commands.entity(player_entity).insert(Tether{ target: nearest_target.unwrap(), joint:None, });
        }
      }
    }
  }
}


