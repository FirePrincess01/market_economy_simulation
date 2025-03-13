// Vertex shader
struct CameraUniform {
    view_pos: vec4<f32>,
    view_proj: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
}

struct InstanceInput {
    @location(5) position: vec3<f32>,
    @location(6) light_color: vec3<f32>,
    @location(7) radius: f32,
    @location(8) linear: f32,
    @location(9) quadratic: f32,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) view_position: vec3<f32>,
    @location(1) model_position: vec3<f32>,
    @location(2) light_color: vec3<f32>,
};

@vertex 
fn vs_main(
    model: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {

    let position = instance.position + model.position * instance.radius;

    var out: VertexOutput;
    out.clip_position = camera.view_proj * vec4<f32>(position, 1.0);
    out.view_position = camera.view_pos.xyz;
    out.model_position = instance.position;
    out.light_color = instance.light_color;
    return out;
}

// Fragment shader

@group(1) @binding(0)
var t_position: texture_2d<f32>;
@group(1) @binding(1)
var t_normal: texture_2d<f32>;
@group(1) @binding(2)
var t_albedo: texture_2d<f32>;

struct FragmentOutput {
    @location(0) surface: vec4<f32>,
};

@fragment
fn fs_main(in: VertexOutput) -> FragmentOutput {
    var out: FragmentOutput;
    out.surface = vec4(in.light_color, 1.0);
    return out;
}
