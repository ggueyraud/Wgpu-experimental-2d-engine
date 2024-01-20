struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>
};

@group(0) @binding(0)
var<uniform> mouse_position: vec2<f32>;
@group(1) @binding(0)
var<uniform> resolution: vec2<f32>;

@group(2) @binding(0)
var<uniform> projection: mat4x4<f32>;

@group(3) @binding(0)
var<uniform> transforms: mat4x4<f32>;

@vertex
fn vs_main(
    model: VertexInput
) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = projection * transforms * vec4<f32>(model.position, 1.0);
    out.color = model.color;

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}