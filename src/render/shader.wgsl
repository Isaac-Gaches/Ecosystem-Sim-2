struct Instance{
    @location(1) c0: vec4<f32>,
    @location(2) c1: vec4<f32>,
    @location(3) c2: vec4<f32>,
    @location(4) c3: vec4<f32>,
}

struct Vertex{
    @location(0) position: vec2<f32>
}

@vertex
fn vs_main(vertex: Vertex, instance: Instance) -> @builtin(position) vec4<f32> {
    let transform = mat4x4<f32>(instance.c0, instance.c1, instance.c2, instance.c3);
    let pos = vec4<f32>(vertex.position, 0.0, 1.0);
    return transform * pos;
}

@fragment
fn fs_main(@builtin(position) in: vec4<f32>)-> @location(0) vec4<f32>{
    return vec4<f32>(1.0);
}