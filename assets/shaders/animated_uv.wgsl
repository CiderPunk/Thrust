#import bevy_pbr::{
  mesh_functions,
  mesh_functions::{get_world_from_local, mesh_position_local_to_clip, mesh_position_local_to_world},
  mesh_view_bindings::globals,
  view_transformations::position_world_to_clip
}

struct FrameDefinition {
  uv_rect: vec4<f32>,
  trim_rect: vec4<f32>,
}

struct EffectSpriteSettings {
  frame_rate: f32,
  frame_count: u32,
  filler: vec2<f32>,
}

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> settings: EffectSpriteSettings;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var<uniform> frames: array<FrameDefinition, 50>;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var atlas_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(3) var atlas_sampler: sampler;

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
  //get time as tag 
  let tag:u32 = mesh_functions::get_tag(vertex.instance_index);
  //convert it back to f32
  let start_time = bitcast<f32>(tag);
  let frame_no =u32(floor((globals.time - start_time) * settings.frame_rate));// % settings.frame_count;

  var position = vertex.position;

  if frame_no > settings.frame_count{
    out.uv = vec2(0.,0.);
  }
  else{
    let frame = frames[frame_no];
    position.x *= mix(frame.trim_rect.x,frame.trim_rect.z, vertex.uv.x);
    position.y *= mix(frame.trim_rect.y,frame.trim_rect.w, vertex.uv.y);
    out.uv.x = mix(frame.uv_rect.x, frame.uv_rect.z, vertex.uv.x);
    out.uv.y = mix(frame.uv_rect.y, frame.uv_rect.w, vertex.uv.y);
  }

  let world_from_local = get_world_from_local(vertex.instance_index);
  out.world_position = mesh_position_local_to_world(world_from_local, vec4(position, 1.0));
  out.clip_position = position_world_to_clip(out.world_position.xyz);
  return out;
}

struct FragmentInput {
  @location(0) world_position: vec4<f32>,
  @location(1) uv: vec2<f32>,
};

@fragment
fn fragment(mesh: FragmentInput) -> @location(0) vec4<f32> {
  return textureSample(atlas_texture, atlas_sampler, mesh.uv);
  //return vec4(1.,0., 0.,1.);
}