#import bevy_pbr::mesh_view_bindings

struct PostProcessingMaterial {
    time_since_startup: f32,
};

@group(1) @binding(0)
var texture: texture_2d<f32>;

@group(1) @binding(1)
var our_sampler: sampler;

@group(1) @binding(2)
var<uniform> material: PostProcessingMaterial;

fn rand2(n: vec2<f32>) -> f32 {
    return fract(sin(dot(n, vec2<f32>(12.9898, 4.1414))) * 43758.5453);
}

fn noise2(coord: vec2<f32>) -> f32 {
    let i = floor(coord);
    let f = fract(coord);

    let a = rand2(i);
    let b = rand2(i + vec2(1.0, 0.0));
    let c = rand2(i + vec2(0.0, 1.0));
    let d = rand2(i + vec2(1.0, 1.0));

    let cubic = f * f * (3.0 - 2.0 * f);

    return mix(a, b, cubic.x) + (c - a) * cubic.y * (1.0 - cubic.x) + (d - b) * cubic.x * cubic.y;
}

@fragment
fn fragment(
    @builtin(position) position: vec4<f32>,
    #import bevy_sprite::mesh2d_vertex_output
) -> @location(0) vec4<f32> {
    // Get screen position with coordinates from 0 to 1
    let uv = position.xy / vec2<f32>(view.width, view.height);
    let scale = 0.003;

    let texture_dimensions = textureDimensions(texture);
    let dimensions = vec2(f32(texture_dimensions.x), f32(texture_dimensions.y));

    let noisecoord1 = uv * scale * dimensions;
    let noisecoord2 = uv * scale * dimensions + 4.0;

    let motion1 = vec2(material.time_since_startup * 0.3, material.time_since_startup * -0.4);
    let motion2 = vec2(material.time_since_startup * 0.1, material.time_since_startup * 0.5);

    let distort1 = vec2(noise2(noisecoord1 + motion1), noise2(noisecoord2 + motion1)) - vec2(0.5);
    let distort2 = vec2(noise2(noisecoord1 + motion2), noise2(noisecoord2 + motion2)) - vec2(0.5);
    let distort_sum = (distort1 + distort2) / vec2(60.0);

    let tint = vec4(0., 100., 255., 255.);
    let color = textureSampleBias(texture, our_sampler, uv + distort_sum, 0.0);
    var output_color = mix(color, tint, 0.0025);

    // Mixing with the tint can desaturate the colors, so add some saturation
    output_color.r = mix(0.5, output_color.r, 1.4);
    output_color.g = mix(0.5, output_color.g, 1.4);
    output_color.b = mix(0.5, output_color.b, 1.4);

    return output_color;
}
