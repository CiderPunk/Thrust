use bevy::{prelude::*, render::render_resource::AsBindGroup, shader::ShaderRef};

pub struct ShaderPlugin;
impl Plugin for ShaderPlugin{
    fn build(&self, app: &mut App) {
      app
        .init_resource::<ShaderMaterials>()
        .add_plugins(MaterialPlugin::<RaysShaderMaterial>::default())
        .add_systems(PreStartup, init_materials);

    }
}

const RAYS_SHADER_PATH: &str = "shaders/spotlight_ray_material.wgsl";


fn init_materials(
  mut commands:Commands,
  mut rays_materials: ResMut<Assets<RaysShaderMaterial>>,
){

  let shader_materials = ShaderMaterials{
    rays: rays_materials.add(RaysShaderMaterial{
      alpha_mode: AlphaMode::Premultiplied,
    }),
  };
  commands.insert_resource::<ShaderMaterials>(shader_materials);
}


#[derive(Resource, Default)]
pub struct ShaderMaterials{
  pub rays:Handle<RaysShaderMaterial>
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