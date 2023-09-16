struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

struct Globals {
    view: mat4x4<f32>,
    color: vec4<f32>,
    screen_px_range: f32,
}

@group(0) @binding(0) var<uniform> globals: Globals;
@group(0) @binding(1) var tex: texture_2d<f32>;
@group(0) @binding(2) var samp: sampler;

@vertex
fn main(
    //@builtin(vertex_index) vertex_index: u32
    @location(0) pos: vec2<f32>,
    @location(1) uv: vec2<f32>,
) -> VertexOutput {
    var result: VertexOutput;
    result.position = vec4<f32>(pos, 0.0, 1.0) * globals.view;
    result.uv = uv;
    return result;
}
