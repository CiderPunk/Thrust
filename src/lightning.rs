use bevy::{light::NotShadowCaster, prelude::*, render::render_resource::AsBindGroup, shader::ShaderRef};

pub struct LightningPlugin;

impl Plugin for LightningPlugin{
  fn build(&self, app: &mut App) {
      app
        .init_resource::<LightningMaterials>()
        .add_plugins(MaterialPlugin::<LightningShaderMaterial>::default())
        .add_systems(PreStartup, init_lightning)
        .add_observer(lighting_material_substitute);
  }
}

const LIGHTNING_SHADER_PATH: &str = "shaders/lightning.wgsl";

#[derive(Resource, Default)]
pub struct LightningMaterials{
  pub tether:Handle<LightningShaderMaterial>,
}


fn init_lightning(
  mut commands:Commands,
  mut lightning_materials: ResMut<Assets<LightningShaderMaterial>>,
){

  let shader_materials = LightningMaterials{
    tether: lightning_materials.add(LightningShaderMaterial{
      alpha_mode: AlphaMode::Premultiplied,
      primary_col: Vec4::new(0.6, 0.2, 0.8, 1.),
      secondary_col: Vec4::new(0.3, 0.05, 0.4, 1.),
    }),

  };
  commands.insert_resource::<LightningMaterials>(shader_materials);
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct LightningShaderMaterial {
  
  #[uniform(0)]
  primary_col: Vec4,
    
  #[uniform(1)]
  secondary_col: Vec4,
  alpha_mode: AlphaMode,
}

impl Material for LightningShaderMaterial {
  fn fragment_shader() -> ShaderRef {
    LIGHTNING_SHADER_PATH.into()
  }
  fn alpha_mode(&self) -> AlphaMode {
    self.alpha_mode
  }
}


#[derive(Component, Default, Reflect, Debug)]
#[reflect(Component, Default)]
#[type_path = "api"]
struct LightningMaterial{
  primary:Color,
  secondary:Color,
}






#[derive(Component)]
struct LightningPointLight;


fn lighting_material_substitute(
  event: On<Add, LightningMaterial>,
  query:Query<&LightningMaterial>,
  mut commands:Commands,
  mut lightning_materials: ResMut<Assets<LightningShaderMaterial>>,
){

  let Ok(mat) = query.get(event.entity) else{ return; };
  
  let material = lightning_materials.add(LightningShaderMaterial{
    alpha_mode: AlphaMode::Premultiplied,
    primary_col: LinearRgba::from(mat.primary).to_vec4(),
    secondary_col:LinearRgba::from(mat.secondary).to_vec4(),
  });

  let light= commands.spawn((
        LightningPointLight,
        Transform::from_xyz(0.,0.,0.),
        PointLight {
          intensity: 1_000_000.0,
          range: 100.,
          color: mat.primary,
          ..default()
        },
      )).id();
  commands
    .entity(event.entity)
    .remove::<MeshMaterial3d<StandardMaterial>>()
    .insert((
      NotShadowCaster,
      MeshMaterial3d(material)
    ))
    .add_child(  
      light
    );


}