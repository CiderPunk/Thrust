#import bevy_pbr::{
    mesh_view_bindings::globals,
    forward_io::VertexOutput,
}

// The MIT License
// Copyright © 2013 Inigo Quilez
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions: The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software. THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
// https://www.youtube.com/c/InigoQuilez
// https://iquilezles.org
//https://www.shadertoy.com/view/Msf3WH


///look at https://www.shadertoy.com/view/XtBGDG

fn  hash(p: vec2<f32>) -> vec2<f32> // replace this by something better
{
	let q = vec2( dot(p,vec2(127.1,311.7)), dot(p,vec2(269.5,183.3)) );
	return -1.0 + 2.0*fract(sin(q)*43758.5453123);
}

fn noise(p: vec2<f32> ) -> f32
{
  const K1 = 0.366025404; // (sqrt(3)-1)/2;
  const K2 = 0.211324865; // (3-sqrt(3))/6;
	let i:vec2<f32> = floor( p + (p.x+p.y)*K1 );
  let a:vec2<f32> = p - i + (i.x+i.y)*K2;
  let m:f32 = step(a.y,a.x); 
  let o:vec2<f32> = vec2(m,1.0-m);
  let b:vec2<f32> = a - o + K2;
	let c:vec2<f32> = a - 1.0 + 2.0*K2;
  let h:vec3<f32> = max( 0.5-vec3<f32>(dot(a,a), dot(b,b), dot(c,c)), vec3<f32>(0.0) );
	let n:vec3<f32> = h*h*h*h*h*vec3( dot(a,hash(i+0.0)), dot(b,hash(i+o)), dot(c,hash(i+1.0)));
  return dot( n, vec3(70.0) );
}


@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {

  let p = mesh.uv;
  let green = noise(p*12. + vec2(1.2,2.33) * -globals.time);
  let blue = noise(p*9.6 + vec2(-1.4,-1.12) * -globals.time);

return vec4(0., green, blue, (green + blue * 0.5));

}