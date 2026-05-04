#import bevy_pbr::{
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::alpha_discard,
    mesh_view_bindings::view,
    forward_io::{VertexOutput, FragmentOutput},
    pbr_functions,
}

struct SnakeInkSettings {
    hatch_density:   f32,
    hatch_thickness: f32,
    cross_angle:     f32,
    grain_intensity: f32,
    black_threshold: f32,
    white_threshold: f32,
    time:            f32,
    _pad:            f32,
}

@group(2) @binding(100)
var<uniform> settings: SnakeInkSettings;

// --- Noise functions ---

fn hash21(p: vec2<f32>) -> f32 {
    var q = fract(p * vec2<f32>(127.1, 311.7));
    q += dot(q, q + 19.19);
    return fract(q.x * q.y);
}

fn hash31(p: vec3<f32>) -> f32 {
    var q = fract(p * vec3<f32>(127.1, 311.7, 74.7));
    q += dot(q, q + 19.19);
    return fract(q.x * q.y + q.y * q.z);
}

// Smooth value noise for grain
fn noise2(p: vec2<f32>) -> f32 {
    let i = floor(p);
    let f = fract(p);
    let u = f * f * (3.0 - 2.0 * f); // smoothstep
    return mix(
        mix(hash21(i),               hash21(i + vec2(1.0, 0.0)), u.x),
        mix(hash21(i + vec2(0.0, 1.0)), hash21(i + vec2(1.0, 1.0)), u.x),
        u.y
    );
}

// Directional hatch — lines along angle `a`, density d
fn hatch(uv: vec2<f32>, a: f32, d: f32, thickness: f32) -> f32 {
    let dir = vec2<f32>(cos(a), sin(a));
    let perp = vec2<f32>(-sin(a), cos(a));
    // Project UV onto perpendicular axis to get line position
    let proj = dot(uv, perp) * d;
    // Sine wave creates parallel bands
    let band = sin(proj * 6.2831853);
    // Threshold to create sharp ink lines
    return smoothstep(1.0 - thickness * 2.0, 1.0, band);
}

// Multi-octave hatching — finer lines layered over coarser ones
fn hatching(uv: vec2<f32>, brightness: f32, time: f32) -> f32 {
    let d  = settings.hatch_density;
    let th = settings.hatch_thickness;
    let ca = settings.cross_angle;

    // Primary hatch direction (along body, UV.x axis)
    let h1 = hatch(uv, 0.0, d,        th);
    // Cross hatch (perpendicular)
    let h2 = hatch(uv, ca,  d * 0.85, th * 0.8);
    // Fine detail layer at 45 degrees
    let h3 = hatch(uv, 0.785, d * 1.6, th * 0.5);

    // Brightness controls which layers are visible:
    // Dark areas: all three layers (dense crosshatch = near black)
    // Mid areas:  two layers
    // Light areas: one layer or none (white)
    var ink = 0.0;
    if brightness < 0.65 { ink = max(ink, h1); }
    if brightness < 0.40 { ink = max(ink, h2); }
    if brightness < 0.22 { ink = max(ink, h3); }

    return ink;
}

@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> FragmentOutput {
    // --- Standard PBR input ---
    var pbr_input = pbr_input_from_standard_material(in, is_front);
    pbr_input.material.base_color = alpha_discard(
        pbr_input.material,
        pbr_input.material.base_color
    );

    // --- Compute PBR lighting to get a brightness value ---
    var out: FragmentOutput;
    out = pbr_functions::apply_pbr_lighting(pbr_input);

    // Convert lit color to grayscale luminance
    let lum = dot(out.color.rgb, vec3<f32>(0.299, 0.587, 0.114));

    // --- UV for hatch pattern ---
    // Use mesh UV, scaled so lines flow along body length
    let uv = in.uv * vec2<f32>(0.5, 4.0);

    // --- Hatching ---
    let ink_line = hatching(uv, lum, settings.time);

    // --- Film grain ---
    // 3D noise using world position + time for animated grain
    let grain_uv = in.world_position.xyz * 180.0 + settings.time * 0.4;
    let grain = hash31(grain_uv) * 2.0 - 1.0;
    let grained_lum = clamp(lum + grain * settings.grain_intensity * 0.15, 0.0, 1.0);

    // --- Compose final B&W value ---
    // Below black_threshold: pure black
    // Above white_threshold: pure white
    // In between: hatching controls density
    var bw: f32;
    if grained_lum < settings.black_threshold {
        bw = 0.0;
    } else if grained_lum > settings.white_threshold {
        bw = 1.0;
    } else {
        // Remap into 0..1 range
        let t = (grained_lum - settings.black_threshold)
              / (settings.white_threshold - settings.black_threshold);
        // Invert ink: ink lines are dark strokes on lighter base
        bw = clamp(t - ink_line * (1.0 - t) * 1.4, 0.0, 1.0);
    }

    // Apply grain on top of final value
    let final_grain = hash31(in.world_position.xyz * 220.0 + settings.time) - 0.5;
    bw = clamp(bw + final_grain * settings.grain_intensity * 0.08, 0.0, 1.0);

    // Output pure grayscale — no color
    out.color = vec4<f32>(bw, bw, bw, 1.0);

    return out;
}
