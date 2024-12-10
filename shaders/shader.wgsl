@group(0) @binding(0) var texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;

struct Vertex {
  @location(0) position: vec2<f32>,
  @location(1) texture_coords: vec2<f32>,
}

struct VertexPayload {
  @builtin(position) position: vec4<f32>,
  @location(0) texture_coords: vec2<f32>,
}

@vertex
fn vs_main(vertex: Vertex) -> VertexPayload {
    var out: VertexPayload;
    out.position = vec4<f32>(vertex.position, 0.0, 1.0);
    out.texture_coords = vertex.texture_coords;
    return out;
}

@fragment
fn fs_main(in: VertexPayload) -> @location(0) vec4<f32> {
    return textureSample(texture, texture_sampler, in.texture_coords);
}
