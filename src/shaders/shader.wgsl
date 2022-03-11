// Vertex shader

struct VertexInput {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] colour: vec3<f32>;
};

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] colour: vec3<f32>;
};

// Entry point set in the render pipeline
[[stage(vertex)]]
fn vs_main(
    // [[builtin(vertex_index)]] in_vertex_index: u32
    model: VertexInput
) -> VertexOutput {
    var out: VertexOutput;
    // let x = f32(1 - i32(in_vertex_index)) * 0.5;
    // let y = f32(i32(in_vertex_index & 1u) * 2 - 1) * 0.5;
    // out.position = vec2<f32>(x, y);
    out.colour = model.colour;
    out.clip_position = vec4<f32>(model.position, 1.0);
    return out;
}

// Fragment shader
// Entry point set in the render pipeline
[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    return vec4<f32>(in.colour, 1.0);
}
