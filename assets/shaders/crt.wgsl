// CRT Scanline Shader for Mahjong Scoundrel
// Creates a retro CRT monitor effect with scanlines and subtle distortion

#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct CrtMaterial {
    time: f32,
    intensity: f32,
};

@group(2) @binding(0)
var<uniform> material: CrtMaterial;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;

    // Scanline effect
    let scanline = sin(uv.y * 800.0) * 0.5 + 0.5;
    let scanline_intensity = mix(1.0, scanline, material.intensity * 0.3);

    // Subtle color aberration at edges
    let center = vec2<f32>(0.5, 0.5);
    let dist = distance(uv, center);
    let aberration = dist * 0.02 * material.intensity;

    // Flicker effect
    let flicker = sin(material.time * 10.0) * 0.02 + 0.98;

    // Vignette
    let vignette = 1.0 - dist * 0.5;

    // Combine effects
    let alpha = (1.0 - scanline_intensity) * 0.15 * vignette;

    return vec4<f32>(0.0, 0.0, 0.0, alpha * flicker);
}
