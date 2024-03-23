struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec4<f32>,
    @location(2) tex_coords: vec2<f32>
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) tex_coords: vec2<f32>
};

@group(0) @binding(0)
var<uniform> mouse_position: vec2<f32>;
@group(1) @binding(0)
var<uniform> resolution: vec2<f32>;

@group(2) @binding(0)
var<uniform> projection: mat4x4<f32>;

@group(3) @binding(0)
var<uniform> transforms: mat4x4<f32>;

@group(4) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(4) @binding(1)
var s_diffuse: sampler;
@group(4) @binding(2)
var<uniform> t_size: vec2<f32>;

@vertex
fn vs_main(
    model: VertexInput
) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = projection * transforms * vec4<f32>(model.position, 1.0);
    out.color = model.color;
    out.tex_coords = model.tex_coords / t_size;

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.tex_coords); // * in.color;
}