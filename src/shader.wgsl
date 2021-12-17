// Vertex shader

struct VertexInput {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] color: vec4<f32>;
};

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] color: vec4<f32>;
};

[[stage(vertex)]]
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;

    // following is code without vertex buffer that drawing a triangle clip
    // let x = f32(1 - i32(in_vertex_index)) * 0.5; // x = (1 - m) / 2
    // let y = f32(i32(in_vertex_index & 1u) * 2 - 1) * 0.5; // y = (2 * (m == 1 ? 1 : 0) - 1) / 2
    // out.clip_position = vec4<f32>(x, y, 0.0, 1.0);
    // out.color = vec4<f32>(0.3, 0.2, 0.1, 0.0);

    // following is transparent vertex pass
    out.color = model.color;
    out.clip_position = vec4<f32>(model.position, 1.0);
    
    return out;
}





// Fragment shader

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    // use vertex color as fragment color
    return in.color;
}