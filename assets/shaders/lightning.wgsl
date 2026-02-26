//ported from https://www.shadertoy.com/view/Mds3W7

#import bevy_sprite::mesh2d_view_bindings::globals
#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import bevy_render::view::View
#import shadplay::shader_utils::common::{sd_sphere, sdBox, rotate2D, calcLookAtMatrix}

@group(0) @binding(0) var<uniform> view: View;

const STRENGTH: f32 = 0.4;  // Controls the strength of the waves
const SPEED: f32 = 0.33333; // Controls the speed at which the waves run



fn rand(n:vec2<f32>)->f32{
  return fract(sin(dot(n, vec2(12.9898,4.1414))) * 43758.5453);
}


fn noise(n:vec2<f32>)->f32{
  const d = vec2(0.0, 1.0);
  let b = floor(n);
  let f = smoothstep(vec2(0.0), vec2(1.0), fract(n));
  return mix(mix(rand(b), rand(b + d.yx), f.x), mix(rand(b + d.xy), rand(b + d.yy), f.x), f.y);
}

fn fbm(in:vec2<f32>)->f32{
  var total = 0.0; 
  var amplitude = 1.0;
  var n = in;
  for (var i:u32 = 0; i < 7; i++) {
    total += noise(n) * amplitude;
    n += n;
    amplitude *= 0.5;
  }
  return total;
}


@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {

  let col = vec4(0.,0.,0.,1.);
  let uv = in.uv;
  
  
  // draw a line, left side is fixed
  let t = uv * vec2(2.0,1.0) - globals.time * 3.0;
  let t2 = (vec2(1.,-1.) + uv) * vec2(2.0,1.0) - globals.time*3.0; // a second strand
  
  // draw the lines,
//  this make the left side fixed, can be useful
//  float ycenter = mix( 0.5, 0.25 + 0.25*fbm( t ), uv.x*4.0);
//    float ycenter2 = mix( 0.5, 0.25 + 0.25*fbm( t2 ), uv.x*4.0);
  let ycenter = fbm(t)*0.5;
  let ycenter2= fbm(t2)*0.5;

  // falloff
  let diff = abs(uv.y - ycenter);
  let c1 = 1.0 - mix(0.0,1.0,diff*20.0);
  
  let diff2 = abs(uv.y - ycenter2);
  let c2 = 1.0 - mix(0.0,1.0,diff2*20.0);
  
  let c = max(c1,c2);
  return vec4(c*0.6,0.2*c2,c,1.0); 
}
