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

    let position = instance.position + model.position;

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

 // read gbuffer
    let index = vec2<u32>(u32(in.clip_position.x), u32(in.clip_position.y));
    let vertex_position: vec4<f32> = textureLoad(t_position, index, 0);
    let vertex_normal: vec4<f32> = textureLoad(t_normal, index, 0);
    let vertex_color_raw: vec4<f32> = textureLoad(t_albedo, index, 0);

    let vertex_color = vec4(vertex_color_raw.xyz, 1.0);

    // calculate lighting
    let light_color = vec4(in.light_color, 1.0);

    // diffuse lighting
    let light_direction = normalize(in.model_position - vertex_position.xyz);
    let diffuse_lighting_strength = max(dot(vertex_normal.xyz, light_direction), 0.0);

    // specular lighting
    let view_position = in.view_position;
    let model_position = vertex_position.xyz;
    let model_normal = vertex_normal.xyz;

    let view_dir = normalize(view_position - model_position);
    
    // pong model
    // let reflect_dir = reflect(-light_direction, model_normal);
    // let specular_lighting_strength = pow(max(dot(view_dir, reflect_dir), 0.0), 32.0);

    // bling-pong model
    let halfway_dir = normalize(light_direction + view_dir);
    let specular_lighting_strength = pow(max(dot(model_normal, halfway_dir), 0.0), 32.0);

    // attenuation
    let constant = 1.0;
    let linear = 0.07;
    let quadratic = 0.017;

    let distance = length(in.model_position - vertex_position.xyz);
    let attenuation = 1.0 / (constant + linear * distance + 
    		    quadratic * (distance * distance)); 

    // pong shading
    let pong_lighting = light_color * (diffuse_lighting_strength + specular_lighting_strength) * attenuation;
    let pong_light: vec4<f32> = pong_lighting * vertex_color;

    // blend with intensity
    // let out_color: vec3<f32> = pong_light;

    // out color
    var out: FragmentOutput;
    // out.surface = vec4<f32>(out_color, 0.5);
    out.surface = pong_light;

    return out;
}
