#import bevy_sprite::mesh2d_vertex_output::VertexOutput

#import bevy_fast_outline::types::Outline2d

@group(#{MATERIAL_BIND_GROUP}) @binding(0)
var material_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(1)
var material_sampler: sampler;
@group(#{MATERIAL_BIND_GROUP}) @binding(2)
var<uniform> outline: Outline2d;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let material_color = textureSample(material_texture, material_sampler, in.uv);
    if material_color.a > 0. {
        return material_color;
    }

    if textureSample(material_texture, material_sampler, in.uv + vec2<f32>(outline.texel_step.x, 0.)).a > 0. {
        return outline.color;
    }
    if textureSample(material_texture, material_sampler, in.uv + vec2<f32>(-outline.texel_step.x, 0.)).a > 0. {
        return outline.color;
    }
    if textureSample(material_texture, material_sampler, in.uv + vec2<f32>(0., outline.texel_step.y)).a > 0. {
        return outline.color;
    }
    if textureSample(material_texture, material_sampler, in.uv + vec2<f32>(0., -outline.texel_step.y)).a > 0. {
        return outline.color;
    }

    return vec4<f32>(0.);
}
