// Vertex shader

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
}

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.tex_coords = model.tex_coords;
    out.clip_position = vec4<f32>(model.position, 1.0);
    return out;
}

// Fragment shader

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0)@binding(1)
var s_diffuse: sampler;

// @fragment
// fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
//     return vec4(in.tex_coords.x, in.tex_coords.y, 0.0, 1.0);
//     // return textureSample(t_diffuse, s_diffuse, in.tex_coords);
// }

// fn decode_nv12(in: VertexOutput) -> vec4<f32> {
//     let width = 1920.0;
//     let height = 1080.0;
//     let uv_offset = height / (1.5 * height);
//     let uv_col = floor(in.tex_coords.x * width / 2.0);
//     let y = textureSample(t_diffuse, s_diffuse, in.tex_coords);
//     let u = textureSample(t_diffuse, s_diffuse, vec2<f32>(uv_col*2.0 / width, in.tex_coords.y + uv_offset));
//     let v = textureSample(t_diffuse, s_diffuse, vec2<f32>((uv_col*2.0+1.0) / width, in.tex_coords.y + uv_offset));
//     let r = 1.164 * (y.r - 0.0625) + 1.596 * (u.r - 0.5);
//     let g = 1.164 * (y.r - 0.0625) - 0.183 * (u.r - 0.5) - 0.391 * (v.r - 0.5);
//     let b = 1.164 * (y.r - 0.0625) + 1.596 * (v.r - 0.5);
//     return vec4(r, g, b, 1.0);
// }

fn decode_nv12(in: VertexOutput) -> vec4<f32> {
    let width = 1920.0;
    let height = 1080.0;
    let uv_offset = height / (1.5 * height);
    let uv_row = floor(in.tex_coords.y * floor(height * 1.5) / 2.0);
    let uv_col = floor(in.tex_coords.x * width / 2.0) * 2.0; // 2.0 because we need two pixels for one UV pair
    let y = textureSample(t_diffuse, s_diffuse, in.tex_coords);
    let u = textureSample(t_diffuse, s_diffuse, vec2<f32>(uv_col / width, uv_offset + uv_row / (height * 1.5)));
    let v = textureSample(t_diffuse, s_diffuse, vec2<f32>((uv_col + 1.0) / width, uv_offset + uv_row / (height * 1.5)));
    let r = y.r + 1.13983 * (v.r - 0.5);
    let g = y.r - 0.39465 * (u.r - 0.5) - 0.58060 * (v.r - 0.5);
    let b = y.r + 2.03211 * (u.r - 0.5);
    return vec4(r, g, b, 1.0);
}

/*
RGB img
| 0 | 1 | 2 | 3 |
| 4 | 5 | 6 | 7 |
| 8 | 9 | 10| 11|
| 12| 13| 14| 15|
| u0| v0| u1| v1|
| u2| v2| u3| v3|

p0 -> tex(0, 0) -> u = tex(0, p0.y + 4 / 6)
p4 -> tex(0, 1/6) -> u = tex(0, p4.y )
*/


@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return decode_nv12(in);
    // let width = 3840.0;
    // let height = 2160.0;

    // let width = 256.0;
    // let height = 256.0;
    // 256x256

    // let width = 1920.0;
    // let height = 1080.0;

    // let img_col = floor(in.tex_coords.x * width);
    // let img_row = floor(in.tex_coords.y * height);

    // let flat_index_r = img_row * (width * 3.0) + img_col * 3.0;
    // let flat_index_g = flat_index_r + 1.0;
    // let flat_index_b = flat_index_r + 2.0;

    // let tex_coords_r = vec2(flat_index_r % (width * 3.0) / (width * 3.0), in.tex_coords.y);
    // let tex_coords_g = vec2(flat_index_g % (width * 3.0) / (width * 3.0), in.tex_coords.y);
    // let tex_coords_b = vec2(flat_index_b % (width * 3.0) / (width * 3.0), in.tex_coords.y);

    // // let tex_coords_r = vec2(in.tex_coords.x * 3.0, in.tex_coords.y);
    // // let tex_coords_g = vec2(in.tex_coords.x * 3.0 + 1.0 / (width * 3.0), in.tex_coords.y);
    // // let tex_coords_b = vec2(in.tex_coords.x * 3.0 + 2.0 / (width * 3.0), in.tex_coords.y);
    
    // // let tex_coords_r = vec2(in.tex_coords.x, in.tex_coords.y);
    // // let tex_coords_g = vec2(in.tex_coords.x + 1.0 / (width*3.0), in.tex_coords.y);
    // // let tex_coords_b = vec2(in.tex_coords.x + 2.0 / (width*3.0), in.tex_coords.y);

    // // let tex_coords_r = in.tex_coords;
    // // let tex_coords_g = in.tex_coords;
    // // let tex_coords_b = in.tex_coords;
    // let r = textureSample(t_diffuse, s_diffuse, tex_coords_r);
    // let g = textureSample(t_diffuse, s_diffuse, tex_coords_g);
    // let b = textureSample(t_diffuse, s_diffuse, tex_coords_b);
    // return vec4(r.r, g.r, b.r, 1.0);
}

/*
RGB img
| 0 | 1 | 2 | 3 |
| 4 | 5 | 6 | 7 |
| 8 | 9 | 10| 11|
| 12| 13| 14| 15|

4x4 RGB in R8 texture
| r0 | g0 | b0 | r1 | g1 | b1 | r2 | g2 | b2 | r3 | g3 | b3 |
| r4 | g4 | b4 | r5 | g5 | b5 | r6 | g6 | b6 | r7 | g7 | b7 |
| r8 | g8 | b8 | r9 | g9 | b9 | r10| g10| b10| r11| g11| b11|
| r12| g12| b12| r13| g13| b13| r14| g14| b14| r15| g15| b15|

p0 -> frag (0, 0) -> tex(0,0), tex(1/12, 0), tex(2/12, 0)
p1 -> frag (0.25, 0) -> tex(3/12, 0), tex(4/12, 0), tex(5/12, 0)
p2 -> frag (0.5, 0) -> tex(6/12, 0), tex(7/12, 0), tex(8/12, 0)
p3 -> frag (0.75, 0) -> tex(9/12, 0), tex(10/12, 0), tex(11/12, 0)
p4 -> frag (0, 0.25) -> tex(0, 1/4), tex(1/12, 1/4), tex(2/12, 1/4)

This isn't the case tho, this will fail for 4k because texture width will be too big....
*/
