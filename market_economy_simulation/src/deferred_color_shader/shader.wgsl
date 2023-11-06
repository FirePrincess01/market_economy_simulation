// Vertex shader
struct CameraUniform {
    view_pos: vec4<f32>,
    view_proj: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
}

struct InstanceInput {
    @location(5) position: vec3<f32>,
    @location(6) color: vec3<f32>,
    @location(7) entity: vec3<u32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
    @location(1) position: vec3<f32>,
    @location(2) normal: vec3<f32>,
    @location(3) entity: vec3<u32>,
};

@vertex 
fn vs_main(
    model: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {

    let position = instance.position + model.position;

    var out: VertexOutput;
    out.clip_position = camera.view_proj * vec4<f32>(position, 1.0);
    out.color = instance.color;
    out.position = position;
    out.normal = model.normal;
    out.entity = instance.entity;

    return out;
}

// Fragment shader
struct FragmentOutput {
    @location(0) surface: vec4<f32>,
    @location(1) position: vec4<f32>,
    @location(2) normal: vec4<f32>,
    @location(3) albedo: vec4<f32>,
    @location(4) entity: vec4<f32>,
};

@fragment
fn fs_main(in: VertexOutput) -> FragmentOutput {

    var entity0 = (in.entity[0] >> 0u) & 0xffu;
    var entity1 = (in.entity[0] >> 8u) & 0xffu;
    var entity2 = (in.entity[0] >> 16u) & 0xffu;
    var entity3 = (in.entity[0] >> 24u) & 0xffu;

    var out: FragmentOutput;
    out.surface = vec4<f32>(in.color, 1.0);
    out.position =  vec4<f32>(in.position, 1.0);
    out.normal =  vec4<f32>(in.normal, 1.0);
    out.albedo = vec4<f32>(in.color, 1.0);
    out.entity =  vec4<f32>(
        f32(entity0)/255.0, 
        f32(entity1)/255.0, 
        f32(entity2)/255.0, 
        f32(entity3)/255.0);

    return out;
}
