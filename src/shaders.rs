use bevy::{light::NotShadowCaster, prelude::*, render::render_resource::AsBindGroup, shader::ShaderRef};

use bevy::color::ColorToComponents;


pub struct ShaderPlugin;
impl Plugin for ShaderPlugin{
    fn build(&self, app: &mut App) {
      app
        .init_resource::<ShaderMaterials>()
        .add_plugins(MaterialPlugin::<RaysShaderMaterial>::default())
        .add_plugins(MaterialPlugin::<ShieldShaderMaterial>::default())
        .add_plugins(MaterialPlugin::<LightningShaderMaterial>::default())
        .add_systems(PreStartup, init_materials)
        .add_observer(lighting_material_substitute);

    }
}

const RAYS_SHADER_PATH: &str = "shaders/spotlight_ray_material.wgsl";
const SHIELD_SHADER_PATH: &str = "shaders/shield.wgsl";
const LIGHTNING_SHADER_PATH: &str = "shaders/lightning.wgsl";

fn init_materials(
  mut commands:Commands,
  mut rays_materials: ResMut<Assets<RaysShaderMaterial>>,
  mut shield_materials: ResMut<Assets<ShieldShaderMaterial>>,
  mut lightning_materials: ResMut<Assets<LightningShaderMaterial>>,
){

  let shader_materials = ShaderMaterials{
    rays: rays_materials.add(RaysShaderMaterial{
      alpha_mode: AlphaMode::Premultiplied,
    }),
    shield: shield_materials.add(ShieldShaderMaterial{ 
      alpha_mode: AlphaMode::Premultiplied,
    }),
    tether: lightning_materials.add(LightningShaderMaterial{
      alpha_mode: AlphaMode::Premultiplied,
      primary_col: Vec4::new(0.6, 0.2, 0.8, 1.),
      secondary_col: Vec4::new(0.3, 0.05, 0.4, 1.),
    }),

  };
  commands.insert_resource::<ShaderMaterials>(shader_materials);
}


#[derive(Resource, Default)]
pub struct ShaderMaterials{
  pub rays:Handle<RaysShaderMaterial>,
  pub shield:Handle<ShieldShaderMaterial>,
  pub tether:Handle<LightningShaderMaterial>,
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct RaysShaderMaterial {
  alpha_mode: AlphaMode,
}

impl Material for RaysShaderMaterial{
  
  fn fragment_shader() -> ShaderRef {
    RAYS_SHADER_PATH.into()
  }
  fn alpha_mode(&self) -> AlphaMode {
    self.alpha_mode
  }

  //some BS to make this double sided
  fn specialize(
    _: &bevy::pbr::MaterialPipeline,
    descriptor: &mut bevy::render::render_resource::RenderPipelineDescriptor,
    _: &bevy::mesh::MeshVertexBufferLayoutRef,
    _: bevy::pbr::MaterialPipelineKey<Self>,
  ) -> Result<(), bevy::render::render_resource::SpecializedMeshPipelineError> {
    descriptor.primitive.cull_mode = None;
    Ok(())
  }
}


#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct ShieldShaderMaterial {
  alpha_mode: AlphaMode,
}

impl Material for ShieldShaderMaterial {
  fn fragment_shader() -> ShaderRef {
    SHIELD_SHADER_PATH.into()
  }
  fn alpha_mode(&self) -> AlphaMode {
    self.alpha_mode
  }
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