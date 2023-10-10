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

// struct ColorInput {
//     @location(1) color: vec3<f32>,
// }

struct InstanceInput {
    // @location(5) model_matrix_0: vec4<f32>,
    // @location(6) model_matrix_1: vec4<f32>,
    // @location(7) model_matrix_2: vec4<f32>,
    // @location(8) model_matrix_3: vec4<f32>,

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
    // model_color: ColorInput,
    instance: InstanceInput,
) -> VertexOutput {
    // let model_matrix = mat4x4<f32>(
    //     instance.model_matrix_0,
    //     instance.model_matrix_1,
    //     instance.model_matrix_2,
    //     instance.model_matrix_3,
    // );

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
    @location(4) entity: u32,
};

@fragment
fn fs_main(in: VertexOutput) -> FragmentOutput {

    var out: FragmentOutput;
    out.surface = vec4<f32>(in.color, 1.0);
    // out.position = in.clip_position;
    // out.position =  vec4<f32>(0.5, 0.5, 0.5, 1.0);
    // out.normal =  vec4<f32>(0.5, 0.5, 0.5, 1.0);
    out.position =  vec4<f32>(in.position, 1.0);
    out.normal =  vec4<f32>(in.normal, 1.0);
    out.albedo = vec4<f32>(in.color, 1.0);
    out.entity = in.entity[0];

    return out;
}
