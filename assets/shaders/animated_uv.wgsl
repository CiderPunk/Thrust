#import bevy_pbr::{
  mesh_functions,

  mesh_functions::{get_world_from_local, mesh_position_local_to_clip},
  mesh_view_bindings::globals,
  view_transformations::position_world_to_clip
}

struct AnimationSettings{
  frame_rate:f32,
  frames_wide:f32, 
  frames_deep:f32,
  frame_count:f32,
}

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> settings: AnimationSettings;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var atlas_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var atlas_sampler: sampler;

struct Vertex {
  @builtin(instance_index) instance_index: u32,
  @location(0) position: vec3<f32>,
  @location(1) normal: vec3<f32>,
  @location(2) uv: vec2<f32>,
};

struct VertexOutput {
  @builtin(position) clip_position: vec4<f32>,
  @location(0) world_position: vec4<f32>,
  @location(1) uv: vec2<f32>,
};

@vertex
fn vertex(vertex:Vertex) -> VertexOutput{

  var out: VertexOutput;
  let world_from_local = mesh_functions::get_world_from_local(vertex.instance_index);
  out.world_position = mesh_functions::mesh_position_local_to_world(world_from_local, vec4(vertex.position, 1.0));
  out.clip_position = position_world_to_clip(out.world_position.xyz);
  //get time as tag as u32
  let tag:u32 = mesh_functions::get_tag(vertex.instance_index);
  //convert it back to f32
  let start_time = bitcast<f32>(tag);
  let frame = floor((globals.time - start_time) * settings.frame_rate);
  if frame > settings.frame_count{
    out.uv = vec2(0.,0.);
  }
  else{
    out.uv = vec2(
      ((frame % settings.frames_wide) * (1./settings.frames_wide)) + (vertex.uv.x / settings.frames_wide),
      (floor(frame/settings.frames_deep) * 1./settings.frames_deep) + (vertex.uv.y / settings.frames_deep),
    );
  }
  return out;
}

struct FragmentInput {
  @location(0) world_position: vec4<f32>,
  @location(1) uv: vec2<f32>,
};

@fragment
fn fragment(mesh: FragmentInput) -> @location(0) vec4<f32> {
  return textureSample(atlas_texture, atlas_sampler, mesh.uv);
  //return vec4(mesh.uv, 1.,1.);
}