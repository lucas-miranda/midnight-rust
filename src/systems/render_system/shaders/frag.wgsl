struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    //@location(0) uv: vec2<f32>,
};

struct Globals {
    view: mat4x4<f32>,
    color: vec4<f32>,
}

@group(0) @binding(0) var<uniform> globals: Globals;
//@group(0) @binding(1) var tex: texture_2d<f32>;
//@group(0) @binding(2) var samp: sampler;

@fragment
fn main(vertex: VertexOutput) -> @location(0) vec4<f32> {
    //return vec4<f32>(1.0, 0.0, 0.0, 1.0); // textureSample(tex, samp, vertex.uv);
    //return vec4<f32>(vertex.uv, 1.0, 1.0);
    //return textureSample(tex, samp, vertex.uv);
    return globals.color;
}
