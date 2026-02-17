#import bevy_pbr::{
    mesh_view_bindings::globals,
    forward_io::VertexOutput,
}

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> color1: vec4<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var<uniform> color2: vec4<f32>;

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    
    let f = mesh.world_position.xz * (1.0 / 3.0);
    let s = floor(abs(sin(f.x+f.y + sin(globals.time) *5.5))+0.2);
    let t = floor(abs(sin(f.x-f.y + cos(globals.time) *3.142))+0.20);
    
    //let t = abs(sin(f.x+f.y + cos(globals.time + 3.142 ) * -32.0));
    //let t = sin(f.x-f.y + globals.time * 10.0);
    //return vec4<f32>(((1.0-mesh.uv.y) * s * color) .xyz, 1.0);
    //return (1.0-mesh.uv.y) * s * color;
    return sin(mesh.uv.y * 3.14159) * (min((s * color1) + (t * color2), vec4(1.,1.,1.,1.)));
}