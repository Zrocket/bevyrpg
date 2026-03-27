@group @binding(0) var screen_texture: texture_2d<f32>;
@group @binding(1) var screen_smp: sampler;

struct Params {
    pub scanline_count: f32,
    pub scanline_dark:  f32,
    pub barrel_str:     f32,
    pub vignette_str:   f32,
    pub chroma_offset:  f32,
    pub glow_amount:    f32,
    pub brightness:     f32,
    pub texel_scale:    f32,
    pub aspect:         vec2<f32>,
    _pad:               vec2<f32>,
}

@group(0) @binding(2) var<uniform> p: Params;

struct VsOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) uv:        vec2<f32>,
}

@vertex
fn vs_main(
    @location(0) in_pos: vec2<f32>,
    @location(1) in_uv:  vec2<f32>,
) -> VsOut {
    var out: VsOut;
    out.pos = vec4(in_pos, 0.0, 1.0);
    out.uv = in_uv;
    return out;
}

fn barrel(uv: vec2<f32>, strength: f32) -> vec2<f32> {
    let r2 = dot(uv, uv);
    return uv + uv * r2 * strength;
}

const LUM = vec3<f32>(0.2126, 0.7152, 0.0722);

fn to_mono(col: vec3<f32>) -> f32 {
    return dot(col, LUM);
}

fn sample_chroma(uv: vec2<f32>, offset: f32) -> vec3<f32> {
    let r = textureSample(screen_texture, screen_smp, uv + vec2(offset, 0.0)).r;
    let g = textureSample(screen_texture, screen_smp, uv).g;
    let b = textureSample(screen_texture, screen_smp, uv - vec2(offset, 0.0)).b;
    return vec3(r, g, b);
}

fn glow_sample(uv: vec2<f32>, texel: vec2<f32>) -> vec3<f32> {
    let w = array<f32, 4>(0.3829, 0.2417, 0.0606, 0.0);
    var acc = textureSample(screen_texture, screen_smp, uv).rgb * w[0];
    for (var i = 1; i < 4; i++) {
        let d = texel * f32(i) * 2.0;
        acc += textureSample(screen_texture, screen_smp, uv + vec2(d.x, 0.0)).rgb * w[i];
        acc += textureSample(screen_texture, screen_smp, uv - vec2(d.x, 0.0)).rgb * w[i];
        acc += textureSample(screen_texture, screen_smp, uv + vec2(0.0, d.y)).rgb * w[i];
        acc += textureSample(screen_texture, screen_smp, uv - vec2(0.0, d.y)).rgb * w[i];
    }
    return acc;
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    var uv = in.uv * 2.0 - 1.0;

    uv *= p.aspect;
    uv = barrel(uv, p.barrel_str);
    uv /= p.aspect;

    if any(abs(uv) > vec2(1.0)) {
        return vec4(0.0, 0.0, 0.0, 0.0);
    }

    let suv = uv * 0.5 + 0.5;

    let texel = (1.0 / vec2<f32>(textureDimensions(screen_texture, 0))) * p.texel_scale;

    var col = sample_chroma(suv, p.chroma_offset);

    let bloom = glow_sample(suv, texel);
    col = col + bloom * p.glow_amount;

    let scan_pos = fract(suv.y * p.scanline_count);
    let scan_mask = 1.0 - step(0.5, scan_pos) * p.scanline_dark;
    col *= scan_mask;

    let dist = length(uv);
    let vig = 1.0 - dist * p.vignette_str;
    col *= max(0.0, vig * vig);

    col *= p.brightness;

    return vec4(col, 1.0);
}
