use std::borrow::Cow;

use bevy::{core_pipeline::core_2d::{graph::{Core2d, Node2d}}, mesh::{PrimitiveTopology, VertexBufferLayout, VertexFormat}, prelude::*, render::{Render, RenderApp, RenderSystems, extract_resource::{ExtractResource, ExtractResourcePlugin}, render_asset::{RenderAssets}, render_graph::{Node, RenderGraphExt, RenderLabel}, render_resource::{AddressMode, AsBindGroup, BindGroup, BindGroupEntries, BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntries, BlendState, CachedRenderPipelineId, ColorTargetState, ColorWrites, FilterMode, FragmentState, LoadOp, MultisampleState, Operations, PipelineCache, PrimitiveState, RenderPassColorAttachment, RenderPassDescriptor, RenderPipelineDescriptor, Sampler, SamplerBindingType, SamplerDescriptor, ShaderStages, ShaderType, StoreOp, TextureFormat, TextureSampleType, UniformBuffer, VertexAttribute, VertexState, VertexStepMode, binding_types::{sampler, texture_2d, uniform_buffer}}, renderer::{RenderDevice, RenderQueue}, texture::GpuImage, view::ViewTarget}};

use crate::furniture::computer::computer_display::ComputerImage;

#[derive(Resource, Clone, ExtractResource)]
pub struct CrtScreenSettings(pub CrtScreenParams);

#[derive(Clone, Copy, ShaderType)]
#[repr(C)]
pub struct CrtScreenParams {
    pub scanline_count: f32,
    pub scanline_dark:  f32,
    pub barrel_str:     f32,
    pub vignette_str:   f32,
    pub chroma_offset:  f32,
    pub glow_amount:    f32,
    pub brightness:     f32,
    pub texel_scale:    f32,
    pub aspect:         Vec2,
}

impl Default for CrtScreenParams {
    fn default() -> Self {
        Self {
            scanline_count: 120.0,
            scanline_dark: 0.55,
            barrel_str: 0.18,
            vignette_str: 0.70,
            chroma_offset: 0.006,
            glow_amount: 0.35,
            brightness: 1.10,
            texel_scale: 1.0,
            aspect: Vec2::new(16.0 / 9.0, 1.0),
        }
    }
}

#[derive(Asset, TypePath, AsBindGroup, Clone)]
pub struct CrtScreenMaterial {
    /// Render Target
    #[texture(0)]
    #[texture(1)]
    pub screen_texture: Handle<Image>,

    /// Crt screen effect Parameters
    #[uniform(2)]
    pub params: CrtScreenParams,
}

impl Material for CrtScreenMaterial {
    fn fragment_shader() -> bevy::shader::ShaderRef {
        "shaders/crt_shader.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

#[derive(Resource)]
pub struct CrtScreenParamBuffer {
    pub buffer: UniformBuffer<CrtScreenParams>
}

impl FromWorld for CrtScreenParamBuffer {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();
        let mut buffer = UniformBuffer::default();
        buffer.set(CrtScreenParams::default());
        buffer.write_buffer(render_device, world.resource());
        Self { buffer }
    }
}

#[derive(Resource)]
pub struct CrtScreenPipeline {
    pub pipeline_id: CachedRenderPipelineId,
    pub bind_group_layout: BindGroupLayout,
    pub sampler: Sampler,
}

impl FromWorld for CrtScreenPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();
        let pipeline_cache = world.resource::<PipelineCache>();
        let asset_server = world.resource::<AssetServer>();

        let shader = asset_server.load("shaders/crt_shader.wgsl");

        let entries = BindGroupLayoutEntries::sequential(
            ShaderStages::FRAGMENT,
            (
                texture_2d(TextureSampleType::Float { filterable: true }),
                sampler(SamplerBindingType::Filtering),
                uniform_buffer::<CrtScreenParams>(false),
            ),
        );

        let bind_group_layout = render_device.create_bind_group_layout(
            "crt_bind_group_layout",
            &entries,
        );

        let layout_descriptor = BindGroupLayoutDescriptor {
            label: Cow::from("crt_bind_group_layout"),
            entries: entries.to_vec(),
        };

        let sampler = render_device.create_sampler(
            &SamplerDescriptor {
                label: Some("crt_sampler"),
                address_mode_u: AddressMode::ClampToEdge,
                address_mode_v: AddressMode::ClampToEdge,
                address_mode_w: AddressMode::ClampToEdge,
                mag_filter: FilterMode::Linear,
                min_filter: FilterMode::Linear,
                mipmap_filter: FilterMode::Nearest,
                ..default()
            });

        let pipeline_id = pipeline_cache.queue_render_pipeline(RenderPipelineDescriptor {
            label: Some("crt_render_pipeline".into()),
            layout: vec![layout_descriptor.clone()],

            vertex: VertexState {
                shader: shader.clone(),
                entry_point: Some(Cow::from("vs_main")),
                shader_defs: vec![],
                buffers: vec![VertexBufferLayout {
                    array_stride: 16,
                    step_mode: VertexStepMode::Vertex,
                    attributes: vec![
                        VertexAttribute {
                            format: VertexFormat::Float32x2,
                            offset: 0,
                            shader_location: 0,
                        },
                        VertexAttribute {
                            format: VertexFormat::Float32x2,
                            offset: 8,
                            shader_location: 1,
                        },
                    ]
                }],
            },

            fragment: Some(FragmentState {
                shader,
                entry_point: Some(Cow::from("fs_main")),
                shader_defs: vec![],
                targets: vec![Some(ColorTargetState {
                    format: TextureFormat::bevy_default(),
                    blend: Some(BlendState::ALPHA_BLENDING),
                    write_mask: ColorWrites::ALL,
                })],
            }),

            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                cull_mode: None,
                ..default()
            },

            depth_stencil: None,
            multisample: MultisampleState::default(),
            push_constant_ranges: vec![],
            zero_initialize_workgroup_memory: false,
        });

        Self { pipeline_id, bind_group_layout, sampler }
    }
}

#[derive(Resource)]
pub struct CrtScreenBindGroup(pub BindGroup);

fn queue_crt_screen_bind_group(
    mut commands: Commands,
    pipeline: Res<CrtScreenPipeline>,
    render_device: Res<RenderDevice>,
    gpu_images: Res<RenderAssets<GpuImage>>,
    computer_image: Option<Res<ComputerImage>>,
    crt_screen_settings: Option<Res<CrtScreenSettings>>,
    mut param_buff: ResMut<CrtScreenParamBuffer>,
    render_queue: Res<RenderQueue>,
) {
    let Some(computer_image) = computer_image else { return };

    let Some(gpu_image) = gpu_images.get(&computer_image.0) else { return };

    let params = crt_screen_settings
        .map(|s| s.0)
        .unwrap_or_default();

    param_buff.buffer.set(params);
    param_buff.buffer.write_buffer(&render_device, &render_queue);

    let Some(binding) = param_buff.buffer.binding() else { return };

    let bind_group = render_device.create_bind_group(
        "crt_bind_group",
        &pipeline.bind_group_layout,
        &BindGroupEntries::sequential((
                &gpu_image.texture_view,
                &pipeline.sampler,
                binding,
        )),
    );

    commands.insert_resource(CrtScreenBindGroup(bind_group));
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, RenderLabel)]
pub struct CrtScreenRenderNode;

impl CrtScreenRenderNode {
    pub const NAME: &'static str = "crt_node";
}

impl Default for CrtScreenRenderNode {
    fn default() -> Self {
        Self
    }
}

impl Node for CrtScreenRenderNode {
    fn run<'w>(
            &self,
            graph: &mut bevy::render::render_graph::RenderGraphContext,
            render_context: &mut bevy::render::renderer::RenderContext<'w>,
            world: &'w World,
        ) -> std::result::Result<(), bevy::render::render_graph::NodeRunError> {
        let Some(bind_group) = world.get_resource::<CrtScreenBindGroup>() else {
            return Ok(());
        };

        let pipeline_res = world.resource::<CrtScreenPipeline>();
        let pipeline_cache = world.resource::<PipelineCache>();

        let Some(pipeline) = pipeline_cache.get_render_pipeline(pipeline_res.pipeline_id) else {
            return Ok(());
        };

        let view_entity = graph.view_entity();
        let view_target = world
            .get::<ViewTarget>(view_entity)
            .expect("view entity missing ViewTarget");

        let mut pass = render_context
            .command_encoder()
            .begin_render_pass(&RenderPassDescriptor {
                label: Some("crt_pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: view_target.main_texture_view(),
                    resolve_target: None,
                    depth_slice: None,
                    ops: Operations {
                        load: LoadOp::Load,
                        store: StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                ..default()
            });

        pass.set_pipeline(pipeline);
        pass.set_bind_group(0, &bind_group.0, &[]);

        pass.draw(0..3, 0..1);

        Ok(())
    }
}

struct CrtShaderPlugin;
impl Plugin for CrtShaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ExtractResourcePlugin::<ComputerImage>::default(),
            ExtractResourcePlugin::<CrtScreenSettings>::default(),
        ));

       let render_app = app.get_sub_app_mut(RenderApp).unwrap();

       render_app
           .init_resource::<CrtScreenPipeline>()
           .init_resource::<CrtScreenParamBuffer>()
           .add_systems(
               Render,
               queue_crt_screen_bind_group.in_set(RenderSystems::Queue),
            )
           .add_render_graph_node::<CrtScreenRenderNode>(Core2d, CrtScreenRenderNode)
           .add_render_graph_edges(Core2d,
               (
                   Node2d::EndMainPass,
                   CrtScreenRenderNode,
                   Node2d::Bloom,
               ),
           );
    }
}
