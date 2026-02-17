// Draws a progress bar with properties defined in CustomUiMaterial
#import bevy_ui::ui_vertex_output::UiVertexOutput

@group(1) @binding(0) var<uniform> color: vec4<f32>;
@group(1) @binding(1) var<uniform> slider: vec4<f32>;
@group(1) @binding(2) var material_color_texture: texture_2d<f32>;
@group(1) @binding(3) var material_color_sampler: sampler;
@group(1) @binding(4) var<uniform> border_color: vec4<f32>;


@fragment
fn fragment(in: UiVertexOutput) -> @location(0) vec4<f32> {
    // sample the texture at this position if it's to the left of the slider value
    // otherwise return a fully transparent color
    let output_color = textureSample(material_color_texture, material_color_sampler, in.uv) * color;
    if in.uv.x < slider.x {
        return output_color;
    } else {
        return vec4(0.0);
    }
}
