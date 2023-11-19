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
    @location(6) intensity: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) view_position: vec3<f32>,
    @location(1) model_position: vec3<f32>,
    @location(2) intensity: vec3<f32>,
};

@vertex 
fn vs_main(
    model: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {

    let position = instance.position + model.position;

    var out: VertexOutput;
    out.clip_position = camera.view_proj * vec4<f32>(position, 1.0);
    out.view_position = camera.view_pos.xyz;
    out.model_position = instance.position;
    out.intensity = instance.intensity;
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

    // read gbuffer
    let index = vec2<u32>(u32(in.clip_position.x), u32(in.clip_position.y));
    let vertex_position: vec4<f32> = textureLoad(t_position, index, 0);
    let vertex_pnormal: vec4<f32> = textureLoad(t_normal, index, 0);
    let vertex_color: vec4<f32> = textureLoad(t_albedo, index, 0);

    // calculate light intensity based on distance
    let intensity_0 = in.intensity[0];
    let intensity_ambient = in.intensity[1];
    let distance: f32 = min(distance(vertex_position.xyz, in.model_position) * 1.0 * in.intensity[0] , 1.0);
    let light_intensity = 1.0 - distance;
    
    var out: FragmentOutput;
    if(intensity_ambient == 0.0) {
        out.surface = vertex_color * light_intensity;
    }
    else {
        out.surface = vertex_color * intensity_ambient;
    }

    return out;
}
