// ── CRT Shader ─────────────────────────────────────────────────────────────
// Vertex and Fragment shaders simulating a CRT monitor.
// Bind group 0:
//   binding 0 → uniform CrtParams
//   binding 1 → texture_2d<f32> (your screen texture)
//   binding 2 → sampler

// ── Uniforms ────────────────────────────────────────────────────────────────

struct CrtParams {
    // Barrel warp strength. 0.0 = flat, 0.1 = mild, 0.25 = heavy
    warp_strength:     f32,
    // How many scanline bands per screen height (e.g. 240, 480)
    scanline_count:    f32,
    // 0.0 = no scanlines, 1.0 = full darkness in gaps
    scanline_strength: f32,
    // Phosphor dot grid scale (0 = off, 1 = subtle, 3 = heavy)
    phosphor_scale:    f32,
    // Chromatic aberration offset in UV space (e.g. 0.002)
    aberration:        f32,
    // Vignette intensity (0 = none, 1 = strong)
    vignette:          f32,
    // Overall brightness multiplier (e.g. 1.2 for a CRT's bloom)
    brightness:        f32,
    // Screen resolution, needed for pixel-accurate effects
    resolution:        vec2<f32>,
}

@group(0) @binding(0) var<uniform> params: CrtParams;
@group(0) @binding(1) var screen_tex: texture_2d<f32>;
@group(0) @binding(2) var screen_smp: sampler;


// ── Vertex shader ───────────────────────────────────────────────────────────
// A simple full-screen triangle. Pass a 3-vertex draw call with no VBO.

struct VertexOut {
    @builtin(position) position: vec4<f32>,
    @location(0)       uv:       vec2<f32>,
}

@vertex
fn vs_main(@builtin(vertex_index) idx: u32) -> VertexOut {
    // Full-screen triangle trick: cover clip space with 3 verts
    var positions = array<vec2<f32>, 3>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>( 3.0, -1.0),
        vec2<f32>(-1.0,  3.0),
    );
    let pos = positions[idx];
    var out: VertexOut;
    out.position = vec4<f32>(pos, 0.0, 1.0);
    // Flip Y: clip space goes -1→+1 bottom-to-top; UV goes 0→1 top-to-bottom
    out.uv = pos * vec2<f32>(0.5, -0.5) + 0.5;
    return out;
}


// ── Helpers ─────────────────────────────────────────────────────────────────

// 1. Barrel distortion
// Maps a UV coordinate to a curved version that mimics convex glass.
// Points near the centre are unmoved; corners are pulled outward.
fn barrel_warp(uv: vec2<f32>, strength: f32) -> vec2<f32> {
    // Re-centre to [-1, 1]
    let c = uv * 2.0 - 1.0;
    // Offset each axis by the squared magnitude of the OTHER axis.
    // This creates the characteristic "pincushion" (negative) or "barrel"
    // (positive) distortion of a curved screen.
    let warp = c + c * strength * (c.yx * c.yx);
    // Map back to [0, 1]
    return warp * 0.5 + 0.5;
}

// 2. Chromatic aberration
// Real CRT lenses refract R, G, B at slightly different angles.
// We simulate this by sampling R and B at UV positions nudged toward/away
// from the centre of the screen.
fn sample_rgb_split(uv: vec2<f32>, offset: f32) -> vec3<f32> {
    let dir = uv - 0.5;                       // vector from centre to pixel
    let r = textureSample(screen_tex, screen_smp,
                uv + dir * offset).r;
    let g = textureSample(screen_tex, screen_smp,
                uv).g;                        // green is reference (sharpest)
    let b = textureSample(screen_tex, screen_smp,
                uv - dir * offset).b;
    return vec3<f32>(r, g, b);
}

// 3. Scanline mask
// Returns a multiplier in [0, 1].  It dips to (1 - strength) in gaps.
// Uses a smooth sine wave so there are no hard edges.
fn scanline_mask(uv_y: f32, count: f32, strength: f32) -> f32 {
    // One full sin cycle per scanline band
    let wave = sin(uv_y * count * 3.14159265);
    // Map sin output [-1, 1] → [0, 1], then lerp toward 1 by (1-strength)
    return 1.0 - strength * (1.0 - wave * wave);
}

// 4. Phosphor dot grid
// A CRT phosphor screen is made of a tiny RGB triad grid.
// We multiply by a sinusoidal grid pattern in both axes.
fn phosphor_mask(uv: vec2<f32>, px_size: vec2<f32>, scale: f32) -> f32 {
    if scale <= 0.0 { return 1.0; }
    // Convert UV to pixel coordinates
    let px = uv / px_size;
    // Dot grid: alternating bright/dark dots at a pixel-level frequency.
    // Even and odd rows are offset by half a cell (hex-grid feel).
    let row     = floor(px.y);
    let col_off = (row % 2.0) * 0.5;
    let dot_x   = sin((px.x + col_off) * 6.28318 * scale);
    let dot_y   = sin( px.y            * 6.28318 * scale);
    // Combine and remap to a mild darkening
    let mask = (dot_x * dot_y) * 0.5 + 0.5;
    return mix(1.0, mask, 0.3 * scale);  // keep subtle
}

// 5. Vignette
// Darkens the corners to simulate light fall-off through a curved lens.
fn vignette(uv: vec2<f32>, strength: f32) -> f32 {
    // Distance from centre, in a smooth [0, 1] range
    let d = length((uv - 0.5) * 2.0);
    // smoothstep edge: begin falloff at 0.5 from centre, full black at 1.4
    return 1.0 - strength * smoothstep(0.5, 1.4, d);
}


// ── Fragment shader ────────────
