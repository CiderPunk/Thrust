use avian3d::prelude::{AngularDamping, Collider, Forces, LockedAxes, MaxAngularSpeed, RigidBody, RigidBodyForces};
use bevy::{color::palettes::css::WHITE, gltf::GltfMesh, light::NotShadowCaster, prelude::*};
use bevy_enhanced_input::prelude::*;


use crate::{asset_management::{AssetLoadState, GameAssets}, game_schedule::GameSchedule, game_state::GameState};

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(PlayerResources{
        collider: None,
        mesh: Handle::default(),
        material: Handle::default(),
      })
      .add_systems(OnEnter(AssetLoadState::Loaded), init_player_reosurces)
      .add_systems(OnEnter(GameState::Initialize), spawn_player)
      .add_observer(player_yaw)
      .add_observer(player_thrust)
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




#[derive(Component)]
struct Player;



#[derive(Component, Default, Reflect, Debug)]
#[reflect(Component, Default)]
#[type_path = "api"]
struct PlayerStart;


#[derive(Resource)]
struct PlayerResources{
  collider: Option<Collider>,
  mesh: Handle<Mesh>,
  material: Handle<StandardMaterial>,
}


fn init_player_reosurces(
  mut player_resources: ResMut<PlayerResources>,
  game_assets: Res<GameAssets>,
  gltf_assets: Res<Assets<Gltf>>,
  gltf_meshes: Res<Assets<GltfMesh>>,
  meshes: Res<Assets<Mesh>>,
  materials: Res<Assets<StandardMaterial>>,
) -> Result<()> {
  let model = gltf_assets.get(&game_assets.models).ok_or("Couldn't get models")?;

  let collision_primative = &gltf_meshes.get( 
      model.named_meshes.get("ship-collision")
      .ok_or("Couldn't get ship collision mesh")?,)
    .ok_or("Couldn't get ship collision mesh data")?
    .primitives[0];

  let display_primative = &gltf_meshes.get( 
      model.named_meshes.get("ship-display")
      .ok_or("Couldn't get ship display mesh")?)
    .ok_or("Couldn't get ship display mesh data")?
    .primitives[0];
  player_resources.mesh = display_primative.mesh.clone(); 
  player_resources.material = display_primative.material.clone().ok_or("no material")?;
  let collision_mesh = meshes.get(&collision_primative.mesh).clone().ok_or("Couldn't get collision mesh")?;
  player_resources.collider = Some(Collider::convex_hull_from_mesh(collision_mesh).ok_or("couldn't create collider from mesh")?);
  Ok(())
}


fn spawn_player(
  query: Query<&GlobalTransform, With<PlayerStart>>,
  mut commands: Commands,
  player_resources: Res<PlayerResources>,
 ){
  for start_transform in query.iter() {
    commands.spawn((
      Player,
      Mesh3d(player_resources.mesh.clone()),
      MeshMaterial3d(player_resources.material.clone()),
      start_transform.clone(),
      RigidBody::Dynamic,
      MaxAngularSpeed(2.0),
      AngularDamping(10.0),
      LockedAxes::new().lock_rotation_y().lock_rotation_x().lock_translation_z(),
      player_resources.collider.clone().unwrap(),
      NotShadowCaster,
      actions!(Player[
        (
          Action::<Yaw>::new(),
          DeadZone::default(),
          SmoothNudge::default(),
          Scale::splat(400.0),
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
      ]),
      children![(
            PointLight {
              intensity: 1_000_000.0,
              color: WHITE.into(),
              shadows_enabled: true,
              ..default()
            },
            Transform::from_xyz(0.,0.,0.)
          )],
      
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
){
  let mut forces = forces_query.get_mut(thrust.context).unwrap();
  //info!("Yaw {}", yaw.value);
  if thrust.value{
    forces.apply_local_force(Vec3::new(0.,200., 0.));
  }
}
