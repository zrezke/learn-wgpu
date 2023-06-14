// Vertex shader

struct VertexInput {
    @location(1) tex_coords: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
}

@vertex
fn vs_main(
    @builtin(vertex_index) v_idx: u32,
) -> VertexOutput {
    let texcoord = vec2<f32>(f32(v_idx / 2u), f32(v_idx % 2u));
    var out: VertexOutput;
    var position: vec2<f32>;
    if (v_idx == 0u) {
        position = vec2<f32>(-1.0, 1.0);
    } else if (v_idx == 1u) {
        position = vec2<f32>(-1.0, -1.0);
    } else if (v_idx == 2u) {
        position = vec2<f32>(1.0, 1.0);
    } else {
        position = vec2<f32>(1.0, -1.0);
    }
    out.tex_coords = texcoord * vec2<f32>(1.0, 2.0 / 3.0); // vertex * extent_u, extent_v
    out.clip_position = vec4<f32>(position, 0.0, 1.0);
    return out;
}

// Fragment shader
@group(0) @binding(0)
var t_diffuse: texture_2d<u32>;
@group(0)@binding(1)
var s_diffuse: sampler;

fn decode_nv12(in: VertexOutput) -> vec4<f32> {
    return _decode_nv12(t_diffuse, in.tex_coords);
}


fn _decode_nv12(texture: texture_2d<u32>, in_tex_coords: vec2<f32>) -> vec4<f32> {
    let texture_dim = vec2<f32>(textureDimensions(texture).xy); // 1920 , 1080 * 1.5
    let uv_offset = u32(floor(texture_dim.y / 1.5));
    let uv_row = u32(floor(in_tex_coords.y * texture_dim.y) / 2.0);
    // let uv_col = u32(floor(in_tex_coords.x * texture_dim.x) / 2.0) * 2u; // 2.0 because we need two pixels for one UV pair

    var uv_col = u32(floor(in_tex_coords.x * texture_dim.x / 2.0)) * 2u;

    let tex_coords = vec2<f32>(in_tex_coords * vec2<f32>(texture_dim.x, texture_dim.y));
    let coords = vec2<u32>(floor(tex_coords.xy));
    var y = f32(textureLoad(texture, coords, 0).r);
    var u = (f32(textureLoad(texture, vec2<u32>(u32(uv_col), uv_offset + uv_row), 0).r));
    var v = (f32(textureLoad(texture, vec2<u32>((u32(uv_col) + 1u), uv_offset + uv_row), 0).r));

    // if y == 128.0 {
    //     return vec4(0.0, 0.0, 1.0, 1.0);
    // }

    // if u == 128.0 && v == 12.0 {
    //     return vec4(0.0, 1.0, 0.0, 1.0);
    // }
    // if u == 10.0 && v == 15.0 {
    //     return vec4(0.0, 0.0, 1.0, 1.0);
    // }

    u = (u - 128.0) / 224.0;
    v = (v - 128.0) / 224.0;

    y = (y - 16.0) / 219.0;

    var red = y + 1.402 * v;
    var green = y  - (0.344 * u + 0.714 * v);
    var blue = y + 1.772 * u;

    // let a = /*0.2627;*/ /*0.2126;*/ 0.299;
    // let b = /*0.6780;*/ /*0.7152;*/ 0.587;
    // let c = /*0.0593;*/ /*0.0722;*/ 0.114;
    // let d = /*1.8814;*/ /*1.8556;*/ 1.772;
    // let e = /*1.4746;*/ /*1.5748;*/ 1.402;

    // let red = y + e * (v);
    // let green = y - (a * e / b) * (u) - (c * d / b) * (v);
    // let blue = y + d * (u);
    return vec4(red, green, blue, 1.0);
    // return vec4(pow(red, 2.2), pow(green, 2.2), pow(blue, 2.2), 1.0);
}

//  r =  yy + (int)(1.402f*v);

//             RGBi[RGBidx++] = (uint8_t) (r > 255 ? 255 : r < 0 ? 0 : r);
//             g =  yy - (int)(0.344f*u + 0.714*v);

//             RGBi[RGBidx++] = (uint8_t) (g > 255 ? 255 : g < 0 ? 0 : g);
//             b =  yy + (int)(1.772f*u);


@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return decode_nv12(in);
}
