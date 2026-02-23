use avian3d::prelude::{AngularDamping, Collider, Forces, LockedAxes, MaxAngularSpeed, RigidBody, RigidBodyForces, TransformInterpolation};
use bevy::{color::palettes::css::WHITE, gltf::GltfMesh, light::NotShadowCaster, prelude::*, render::view::visibility, time::Stopwatch};
use bevy_enhanced_input::prelude::{Release, *};
use crate::{asset_management::{AssetLoadState, GameAssets}, game_state::GameState};


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
      .add_observer(player_yaw)
      .add_observer(player_thrust)
      .add_observer(player_thrust_release)
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
pub struct Player;


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


#[derive(Resource, Default)]
struct PlayerResources{
  collider: Option<Collider>,
  ship_mesh: Handle<Mesh>,
  ship_material: Handle<StandardMaterial>,
  flame_mesh: Handle<Mesh>,
  flame_material: Handle<StandardMaterial>,
}


fn init_player_reosurces(
  mut player_resources: ResMut<PlayerResources>,
  game_assets: Res<GameAssets>,
  gltf_assets: Res<Assets<Gltf>>,
  gltf_meshes: Res<Assets<GltfMesh>>,
  meshes: Res<Assets<Mesh>>,
  //materials: Res<Assets<StandardMaterial>>,
) -> Result<()> {
  let models = gltf_assets.get(&game_assets.models).ok_or("Couldn't get models")?;

  let collision_primative = &gltf_meshes.get( 
      models.named_meshes.get("ship-collision")
      .ok_or("Couldn't get ship collision mesh")?,)
    .ok_or("Couldn't get ship collision mesh data")?
    .primitives[0];

  let display_primative = &gltf_meshes.get( 
      models.named_meshes.get("ship-display")
      .ok_or("Couldn't get ship display mesh")?)
    .ok_or("Couldn't get ship display mesh data")?
    .primitives[0];

  let flame_primative =  &gltf_meshes.get( 
      models.named_meshes.get("ship-flame")
      .ok_or("Couldn't get ship flame mesh")?)
    .ok_or("Couldn't get ship flame mesh data")?
    .primitives[0];

  player_resources.flame_mesh = flame_primative.mesh.clone();
  player_resources.flame_material = flame_primative.material.clone().ok_or("no flame material")?;
  player_resources.ship_mesh = display_primative.mesh.clone(); 
  player_resources.ship_material = display_primative.material.clone().ok_or("no ship material")?;
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
      Mesh3d(player_resources.ship_mesh.clone()),
      MeshMaterial3d(player_resources.ship_material.clone()),
      start_transform.clone(),
      RigidBody::Dynamic,
      MaxAngularSpeed(2.0),
      AngularDamping(20.0),
      TransformInterpolation,
      LockedAxes::new().lock_rotation_y().lock_rotation_x().lock_translation_z(),
      player_resources.collider.clone().unwrap(),
      NotShadowCaster,
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
          HoldAndRelease::new(0.),
          bindings![KeyCode::ArrowUp, KeyCode::KeyW, GamepadButton::DPadUp],
        ),
      ]),
      children![(
            PointLight {
              intensity: 3_000_000.0,
              range: 50.,
              color: WHITE.into(),
              shadows_enabled: true,
              
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
  thrust:On<Ongoing<Thrust>>,
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
  _:On<Fire<Thrust>>,
  flame_visiblity:Single<(&mut Visibility, &mut PlayerFlame)>,
){
  let (mut visible, mut flame) = flame_visiblity.into_inner();
  flame.ignite_time.reset();
  *visible = Visibility::Hidden;     

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
