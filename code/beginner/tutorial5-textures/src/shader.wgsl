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

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let width = 2494.0;
    let height = 2494.0;
    let tex_coords = in.tex_coords;


    // // Calculate pixel position
    // let x = floor(tex_coords.x * width);
    // let y = floor(tex_coords.y * height);
  
    // // Calculate offset to represent how many components have been shifted
    // let offset = (x + y * width) % 4.0;
  
    // // Shift the pixel color components based on the offset
    // var color = textureSample(t_diffuse, s_diffuse, tex_coords);
  
    // if (offset == 1.0) {
    //     color = vec4<f32>(color.a, color.r, color.g, color.b);
    // } else if (offset == 2.0) {
    //     color = vec4<f32>(color.b, color.a, color.r, color.g);
    // } else if (offset == 3.0) {
    //     color = vec4<f32>(color.g, color.b, color.a, color.r);
    // }
    // return color;

    return vec4(tex_coords.x, tex_coords.y, 0.0, 1.0);
}

/*
(0,0) -> r,g,b,1 | (aa) tc.rgb, 1
(1,0) -> aa, r, g, 1 | (bb, ba)  prev_tc_x = tc.x - 1 / w, prev_tc_y = 0
(2, 0) -> bb, ba, r, 1 | (cg, cb, ca)
(3, 0) -> cg, cb, ca, 1 | (dr, dg, db, da)
(4, 0) -> dr, dg, db, 1 | (da, er, eg, eb, ea)
(0, 1) -> da, er, eg, 1 | (eb, ea, fa, fb, fg, fa)
(1, 1) -> eb, ea, fa, 1 | (fb, fg, fa, ga, gb, gc)
(2, 1) -> fb, fg, fa, 1 | (ga, gb, gc, gd, ge, gf)
(3, 1) -> ga, gb, gc, 1 | (gd, ge, gf, gg, gh, gi)
(4, 1) -> gd, ge, gf, 1 | (gg, gh, gi, gj, gk, gl)
*/
