#import bevy_pbr::{
    mesh_view_bindings::globals,
    forward_io::VertexOutput,
}
#import bevy_render::view::View



// Improved high-frequency noise (Interleaved Gradient Noise)
fn interleaved_gradient_noise(frag_coord: vec2<f32>) -> f32 {
    let magic = vec3<f32>(0.06711056, 0.00583715, 52.9829189);
    return fract(magic.z * fract(dot(frag_coord, magic.xy)));
}

@fragment
fn fragment(
  mesh: VertexOutput,
) -> @location(0) vec4<f32> {    
  let p = min(1.-abs(cos(mesh.uv.x * 3.14159 *0.5)),1.)  ;
  let q = min(1.-abs( cos( ((mesh.uv.y + (globals.time * 0.02)) * 3.14159 *20.))),1.) * p ;
  //let q = abs( cos( (mesh.uv.y * 3.14159 * 40.)));
  //let dither = (interleaved_gradient_noise(mesh.position.xy) - 0.5) / 16.0;
  let colour = ((p * p * 0.5) + (q * 0.01)) * vec4(1.,1.,0.8,0.2);
  //let colour = q * vec4(1.,1.,0.8,0.2);
  return colour.rgba;
}