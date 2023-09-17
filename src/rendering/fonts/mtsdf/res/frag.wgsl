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

fn median(r: f32, g: f32, b: f32) -> f32 {
    return max(min(r, g), min(max(r, g), b));
}

@fragment
fn main(vertex: VertexOutput) -> @location(0) vec4<f32> {
    var px: vec4<f32> = textureSample(tex, samp, vertex.uv);
    let screen_px_distance: f32 = globals.screen_px_range * (median(px.r, px.g, px.b) - .5);
    let alpha: f32 = clamp(screen_px_distance + .5, 0.0, 1.0);

    px = mix(vec4<f32>(0.0, 0.0, 0.0, 0.0), globals.color, alpha);

    if px.a < .05 {
        discard;
    }

    return px;
}
