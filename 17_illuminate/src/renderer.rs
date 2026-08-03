use image::GenericImageView;
use std::borrow::Cow;
use std::mem;
use wgpu::util::DeviceExt;

use crate::Arc;
use crate::{OwnedDisplayHandle, Window};

const LOGIC_WIDTH: u32 = 320;
const LOGIC_HEIGHT: u32 = 240;

struct Size {
    width: u32,
    height: u32,
}

// ------------------------------------------------------------------------------------------- struct for vertex buffer
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 2],
    uv: [f32; 2],
}

// ---------------------------------------------------------------------------------- descriptor for VertexBufferLayout
impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x2];
    // 0 => Vertex::position, 1 => Vertex::uv

    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

// -------------------------------------------------------------------------------------- data for full screen triangle
const FULLSCREEN_VERTICES: &[Vertex] = &[
    Vertex {
        position: [-1.0, -1.0], // bottom left
        uv: [0.0, 1.0],
    },
    Vertex {
        position: [-1.0, 3.0], // top left outside
        uv: [0.0, -1.0],
    },
    Vertex {
        position: [3.0, -1.0], // bottom right outside
        uv: [2.0, 1.0],
    },
];

// ----------------------------------------------------------------------------------------------------- renderer state
pub struct Renderer {
    instance: wgpu::Instance,
    window: Arc<Window>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    size: winit::dpi::PhysicalSize<u32>,
    surface: wgpu::Surface<'static>,
    surface_format: wgpu::TextureFormat,

    fullscreen_vertex_buffer: wgpu::Buffer,

    base_texture_view: wgpu::TextureView,
    extract_texture_view: wgpu::TextureView,
    down1_texture_view: wgpu::TextureView,
    down2_texture_view: wgpu::TextureView,
    down3_texture_view: wgpu::TextureView,
    up1_texture_view: wgpu::TextureView,
    up2_texture_view: wgpu::TextureView,
    up3_texture_view: wgpu::TextureView,

    resource_bind_group: wgpu::BindGroup,
    extract_bind_group: wgpu::BindGroup,
    down1_bind_group: wgpu::BindGroup,
    down2_bind_group: wgpu::BindGroup,
    down3_bind_group: wgpu::BindGroup,
    up1_bind_group: wgpu::BindGroup,
    up2_bind_group: wgpu::BindGroup,
    up3_bind_group: wgpu::BindGroup,
    scaler_bind_group: wgpu::BindGroup,

    base_render_pipeline: wgpu::RenderPipeline,
    extract_render_pipeline: wgpu::RenderPipeline,
    down1_render_pipeline: wgpu::RenderPipeline,
    down2_render_pipeline: wgpu::RenderPipeline,
    down3_render_pipeline: wgpu::RenderPipeline,
    up1_render_pipeline: wgpu::RenderPipeline,
    up2_render_pipeline: wgpu::RenderPipeline,
    up3_render_pipeline: wgpu::RenderPipeline,
    scaler_render_pipeline: wgpu::RenderPipeline,
}

impl Renderer {
    pub async fn new(display: OwnedDisplayHandle, window: Arc<Window>) -> Renderer {
        // --------------------------------------------------------------------------------- create a new wgpu instance
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_with_display_handle(
            Box::new(display),
        ));

        // -------------------------------------------------------------------------------------------- physical device
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions::default())
            .await
            .unwrap();

        // --------------------------------------------------------------------------------------------- logical device
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default())
            .await
            .unwrap();

        // --------------------------------------------------------------------------------------------- size of window
        let size = window.inner_size();

        // ----------------------------------------------------------------------------------------------- load shaders
        let base_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("base shader"),
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shaders/base.wgsl"))),
        });

        let extract_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("extract shader"),
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shaders/extract.wgsl"))),
        });

        let down_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("down shader"),
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shaders/down.wgsl"))),
        });

        let up_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("up shader"),
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shaders/up.wgsl"))),
        });

        let scaler_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("scaler shader"),
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shaders/scaler.wgsl"))),
        });

        // ---------------------------------------------------------------------------------- full screen vertex buffer
        let fullscreen_vertex_buffer =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("full screen vertex buffer"),
                contents: bytemuck::cast_slice(FULLSCREEN_VERTICES),
                usage: wgpu::BufferUsages::VERTEX,
            });

        // --------------------------------------------------------------------------------------- surface to draw onto
        let surface = instance.create_surface(window.clone()).unwrap();
        let cap = surface.get_capabilities(&adapter);
        let surface_format = cap.formats[0];

        // ---------------------------------------------------------------------------------------------------- sampler
        let sampler_nearest = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::MipmapFilterMode::Nearest,
            ..Default::default()
        });

        let sampler_linear = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::MipmapFilterMode::Linear,
            ..Default::default()
        });

        // ----------------------------------------------------------- bind group layout for simple sampling and drawing
        let simple_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("simple bind group layout for sampling and drawing"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
            });

        // ----------------------------------------------------------- bind group layout for blending
        let blend_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("blend bind group layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 2,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                ],
            });

        // ------------------------------------------------------------------------------------------------------- image
        let resource_bytes = include_bytes!("../img/illuminate_test.png");

        let resource_texture_view =
            resource_texture_builder(&device, &queue, resource_bytes, "resource texture");

        let resource_bind_group = simple_bind_group_builder(
            &device,
            "resource bind group",
            &simple_bind_group_layout,
            &resource_texture_view,
            &sampler_nearest,
        );

        // ----------------------------------------------------------------------------------- base texture to draw onto
        let base_texture_view = texture_builder(
            &device,
            "base texture",
            &Size {
                width: LOGIC_WIDTH,
                height: LOGIC_HEIGHT,
            },
        );

        // ------------------------------------------------------------------------- extract texture view and bind group
        let extract_texture_view = texture_builder(
            &device,
            "extract texture",
            &Size {
                width: LOGIC_WIDTH,
                height: LOGIC_HEIGHT,
            },
        );
        let extract_bind_group = simple_bind_group_builder(
            &device,
            "extract bind group",
            &simple_bind_group_layout,
            &base_texture_view,
            &sampler_nearest,
        );

        // ------------------------------------------------------------------------- smaller texture to make image blur
        let down1_texture_view = texture_builder(
            &device,
            "down1 texture",
            &Size {
                width: LOGIC_WIDTH / 2,
                height: LOGIC_HEIGHT / 2,
            },
        );
        let down1_bind_group = simple_bind_group_builder(
            &device,
            "down1 bind group",
            &simple_bind_group_layout,
            &extract_texture_view,
            &sampler_linear,
        );

        let down2_texture_view = texture_builder(
            &device,
            "down2 texture",
            &Size {
                width: LOGIC_WIDTH / 4,
                height: LOGIC_HEIGHT / 4,
            },
        );
        let down2_bind_group = simple_bind_group_builder(
            &device,
            "down2 bind group",
            &simple_bind_group_layout,
            &down1_texture_view,
            &sampler_linear,
        );

        let down3_texture_view = texture_builder(
            &device,
            "down3 texture",
            &Size {
                width: LOGIC_WIDTH / 8,
                height: LOGIC_HEIGHT / 8,
            },
        );
        let down3_bind_group = simple_bind_group_builder(
            &device,
            "down3 bind group",
            &simple_bind_group_layout,
            &down2_texture_view,
            &sampler_linear,
        );

        let up1_texture_view = texture_builder(
            &device,
            "up1 texture",
            &Size {
                width: LOGIC_WIDTH / 4,
                height: LOGIC_HEIGHT / 4,
            },
        );
        let up1_bind_group = blend_bind_group_builder(
            &device,
            "up1 bind group",
            &blend_bind_group_layout,
            &down2_texture_view,
            &down3_texture_view,
            &sampler_linear,
        );

        let up2_texture_view = texture_builder(
            &device,
            "up2 texture",
            &Size {
                width: LOGIC_WIDTH / 2,
                height: LOGIC_HEIGHT / 2,
            },
        );
        let up2_bind_group = blend_bind_group_builder(
            &device,
            "up2 bind group",
            &blend_bind_group_layout,
            &up1_texture_view,
            &down1_texture_view,
            &sampler_linear,
        );

        let up3_texture_view = texture_builder(
            &device,
            "up3 texture",
            &Size {
                width: LOGIC_WIDTH,
                height: LOGIC_HEIGHT,
            },
        );
        let up3_bind_group = blend_bind_group_builder(
            &device,
            "up3 bind group",
            &blend_bind_group_layout,
            &up2_texture_view,
            &extract_texture_view,
            &sampler_linear,
        );
        // ------------------------------------------------------------------------------------------- scaler bind group
        let scaler_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("bind group layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 2,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 3,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                ],
            });

        let scaler_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("scaler bind group"),
            layout: &scaler_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Sampler(&sampler_nearest),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler_linear),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureView(&base_texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::TextureView(&up3_texture_view),
                },
            ],
        });

        // ----------------------------------------------------------------------------------------------- base pipeline
        let base_render_pipeline = pipeline_builder(
            &device,
            "render pipeline for base",
            &simple_bind_group_layout,
            &base_shader,
        );

        // -------------------------------------------------------------------------------------------- extract pipeline
        let extract_render_pipeline = pipeline_builder(
            &device,
            "render pipeline for extract",
            &simple_bind_group_layout,
            &extract_shader,
        );

        // ------------------------------------------------------------------------- pipeline to sample and draw smaller
        let down1_render_pipeline = pipeline_builder(
            &device,
            "render pipeline for down1",
            &simple_bind_group_layout,
            &down_shader,
        );

        let down2_render_pipeline = pipeline_builder(
            &device,
            "render pipeline for down2",
            &simple_bind_group_layout,
            &down_shader,
        );

        let down3_render_pipeline = pipeline_builder(
            &device,
            "render pipeline for down3",
            &simple_bind_group_layout,
            &down_shader,
        );

        let up1_render_pipeline = pipeline_builder(
            &device,
            "render pipeline for up1",
            &blend_bind_group_layout,
            &up_shader,
        );

        let up2_render_pipeline = pipeline_builder(
            &device,
            "render pipeline for up2",
            &blend_bind_group_layout,
            &up_shader,
        );

        let up3_render_pipeline = pipeline_builder(
            &device,
            "render pipeline for up3",
            &blend_bind_group_layout,
            &up_shader,
        );

        // ----------------------------------------------------------------------------------------- pipeline for scaler
        let scaler_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("render pipeline layout for surface"),
                bind_group_layouts: &[Some(&scaler_bind_group_layout)],
                immediate_size: 0,
            });
        let swapchain_capabilities = surface.get_capabilities(&adapter);
        let swapchain_format = swapchain_capabilities.formats[0];

        let scaler_render_pipeline =
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("render pipeline for surface"),
                layout: Some(&scaler_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &scaler_shader,
                    entry_point: Some("vs_main"),
                    buffers: &[Vertex::desc()],
                    compilation_options: Default::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: &scaler_shader,
                    entry_point: Some("fs_main"),
                    compilation_options: Default::default(),
                    targets: &[Some(swapchain_format.into())],
                }),
                primitive: wgpu::PrimitiveState::default(),
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                multiview_mask: None,
                cache: None,
            });

        // --------------------------------------------------------------------------------------------------------------

        let renderer = Renderer {
            instance,
            window,
            device,
            queue,
            size,
            surface,
            surface_format,

            fullscreen_vertex_buffer,

            base_texture_view,
            extract_texture_view,
            down1_texture_view,
            down2_texture_view,
            down3_texture_view,
            up1_texture_view,
            up2_texture_view,
            up3_texture_view,

            resource_bind_group,
            extract_bind_group,
            down1_bind_group,
            down2_bind_group,
            down3_bind_group,
            up1_bind_group,
            up2_bind_group,
            up3_bind_group,
            scaler_bind_group,

            base_render_pipeline,
            extract_render_pipeline,
            down1_render_pipeline,
            down2_render_pipeline,
            down3_render_pipeline,
            up1_render_pipeline,
            up2_render_pipeline,
            up3_render_pipeline,
            scaler_render_pipeline,
        };

        // ------------------------------------------------------------------------ configure surface for the first time
        renderer.configure_surface();

        renderer
    }

    fn configure_surface(&self) {
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: self.surface_format,
            // Request compatibility with the sRGB-format texture view we‘re going to create later.
            view_formats: vec![self.surface_format.add_srgb_suffix()],
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            width: self.size.width,
            height: self.size.height,
            desired_maximum_frame_latency: 2,
            present_mode: wgpu::PresentMode::AutoVsync,
        };
        self.surface.configure(&self.device, &surface_config);
    }

    pub fn render(&mut self) {
        // --------------------------------------------------------------------------------- create surface texture view
        let surface_texture = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(texture) => texture,
            wgpu::CurrentSurfaceTexture::Occluded | wgpu::CurrentSurfaceTexture::Timeout => return,
            wgpu::CurrentSurfaceTexture::Suboptimal(texture) => {
                drop(texture);
                self.configure_surface();
                return;
            }
            wgpu::CurrentSurfaceTexture::Outdated => {
                self.configure_surface();
                return;
            }
            wgpu::CurrentSurfaceTexture::Validation => {
                unreachable!("No error scope registered, so validation errors will panic")
            }
            wgpu::CurrentSurfaceTexture::Lost => {
                self.surface = self.instance.create_surface(self.window.clone()).unwrap();
                self.configure_surface();
                return;
            }
        };

        let surface_texture_view =
            surface_texture
                .texture
                .create_view(&wgpu::TextureViewDescriptor {
                    // Without add_srgb_suffix() the image we will be working with
                    // might not be "gamma correct".
                    format: Some(self.surface_format.add_srgb_suffix()),
                    ..Default::default()
                });

        let mut encoder = self.device.create_command_encoder(&Default::default());

        // ----------------------------------------------------------------------------------------- renderpass for base
        let mut base_renderpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("base renderpass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &self.base_texture_view,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });

        // ------------------------------------------------------------------------------------------ use the renderpass
        base_renderpass.set_pipeline(&self.base_render_pipeline);
        base_renderpass.set_bind_group(0, Some(&self.resource_bind_group), &[]);
        base_renderpass.set_vertex_buffer(0, self.fullscreen_vertex_buffer.slice(..));
        base_renderpass.set_viewport(0.0, 0.0, LOGIC_WIDTH as f32, LOGIC_HEIGHT as f32, 0.0, 1.0);
        base_renderpass.draw(0..3, 0..1);

        // ------------------------------------------------------------------------------------------ end the renderpass
        drop(base_renderpass);

        // --------------------------------------------------------------------------------------- renderpass for extract
        let mut extract_renderpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("extract renderpass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &self.extract_texture_view,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });

        // ------------------------------------------------------------------------------------------ use the renderpass
        extract_renderpass.set_pipeline(&self.extract_render_pipeline);
        extract_renderpass.set_bind_group(0, Some(&self.extract_bind_group), &[]);
        extract_renderpass.set_vertex_buffer(0, self.fullscreen_vertex_buffer.slice(..));
        extract_renderpass.set_viewport(
            0.0,
            0.0,
            LOGIC_WIDTH as f32,
            LOGIC_HEIGHT as f32,
            0.0,
            1.0,
        );
        extract_renderpass.draw(0..3, 0..1);

        // ------------------------------------------------------------------------------------------ end the renderpass
        drop(extract_renderpass);

        // --------------------------------------------------------------------------------------- renderpass for down1
        let mut down1_renderpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("down1 renderpass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &self.down1_texture_view,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });

        // ------------------------------------------------------------------------------------------ use the renderpass
        down1_renderpass.set_pipeline(&self.down1_render_pipeline);
        down1_renderpass.set_bind_group(0, Some(&self.down1_bind_group), &[]);
        down1_renderpass.set_vertex_buffer(0, self.fullscreen_vertex_buffer.slice(..));
        down1_renderpass.set_viewport(
            0.0,
            0.0,
            (LOGIC_WIDTH / 2) as f32,
            (LOGIC_HEIGHT / 2) as f32,
            0.0,
            1.0,
        );
        down1_renderpass.draw(0..3, 0..1);

        // ------------------------------------------------------------------------------------------ end the renderpass
        drop(down1_renderpass);

        // --------------------------------------------------------------------------------------- renderpass for down2
        let mut down2_renderpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("down2 renderpass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &self.down2_texture_view,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });

        // ------------------------------------------------------------------------------------------ use the renderpass
        down2_renderpass.set_pipeline(&self.down2_render_pipeline);
        down2_renderpass.set_bind_group(0, Some(&self.down2_bind_group), &[]);
        down2_renderpass.set_vertex_buffer(0, self.fullscreen_vertex_buffer.slice(..));
        down2_renderpass.set_viewport(
            0.0,
            0.0,
            (LOGIC_WIDTH / 4) as f32,
            (LOGIC_HEIGHT / 4) as f32,
            0.0,
            1.0,
        );
        down2_renderpass.draw(0..3, 0..1);

        // ------------------------------------------------------------------------------------------ end the renderpass
        drop(down2_renderpass);

        // --------------------------------------------------------------------------------------- renderpass for down3
        let mut down3_renderpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("down3 renderpass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &self.down3_texture_view,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });

        // ------------------------------------------------------------------------------------------ use the renderpass
        down3_renderpass.set_pipeline(&self.down3_render_pipeline);
        down3_renderpass.set_bind_group(0, Some(&self.down3_bind_group), &[]);
        down3_renderpass.set_vertex_buffer(0, self.fullscreen_vertex_buffer.slice(..));
        down3_renderpass.set_viewport(
            0.0,
            0.0,
            (LOGIC_WIDTH / 8) as f32,
            (LOGIC_HEIGHT / 8) as f32,
            0.0,
            1.0,
        );
        down3_renderpass.draw(0..3, 0..1);

        // ------------------------------------------------------------------------------------------ end the renderpass
        drop(down3_renderpass);

        // --------------------------------------------------------------------------------------- renderpass for up1
        let mut up1_renderpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("up1 renderpass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &self.up1_texture_view,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });

        // ------------------------------------------------------------------------------------------ use the renderpass
        up1_renderpass.set_pipeline(&self.up1_render_pipeline);
        up1_renderpass.set_bind_group(0, Some(&self.up1_bind_group), &[]);
        up1_renderpass.set_vertex_buffer(0, self.fullscreen_vertex_buffer.slice(..));
        up1_renderpass.set_viewport(
            0.0,
            0.0,
            (LOGIC_WIDTH / 4) as f32,
            (LOGIC_HEIGHT / 4) as f32,
            0.0,
            1.0,
        );
        up1_renderpass.draw(0..3, 0..1);

        // ------------------------------------------------------------------------------------------ end the renderpass
        drop(up1_renderpass);

        // --------------------------------------------------------------------------------------- renderpass for up2
        let mut up2_renderpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("up2 renderpass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &self.up2_texture_view,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });

        // ------------------------------------------------------------------------------------------ use the renderpass
        up2_renderpass.set_pipeline(&self.up2_render_pipeline);
        up2_renderpass.set_bind_group(0, Some(&self.up2_bind_group), &[]);
        up2_renderpass.set_vertex_buffer(0, self.fullscreen_vertex_buffer.slice(..));
        up2_renderpass.set_viewport(
            0.0,
            0.0,
            (LOGIC_WIDTH / 2) as f32,
            (LOGIC_HEIGHT / 2) as f32,
            0.0,
            1.0,
        );
        up2_renderpass.draw(0..3, 0..1);

        // ------------------------------------------------------------------------------------------ end the renderpass
        drop(up2_renderpass);

        // --------------------------------------------------------------------------------------- renderpass for up3
        let mut up3_renderpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("up3 renderpass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &self.up3_texture_view,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });

        // ------------------------------------------------------------------------------------------ use the renderpass
        up3_renderpass.set_pipeline(&self.up3_render_pipeline);
        up3_renderpass.set_bind_group(0, Some(&self.up3_bind_group), &[]);
        up3_renderpass.set_vertex_buffer(0, self.fullscreen_vertex_buffer.slice(..));
        up3_renderpass.set_viewport(0.0, 0.0, LOGIC_WIDTH as f32, LOGIC_HEIGHT as f32, 0.0, 1.0);
        up3_renderpass.draw(0..3, 0..1);

        // ------------------------------------------------------------------------------------------ end the renderpass
        drop(up3_renderpass);

        // -------------------------------------------------------------------------------------- renderpass for surface
        let mut scaler_renderpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("scaler renderpass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &surface_texture_view,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });

        let viewport_xywh = self.calc_ratio();

        // ------------------------------------------------------------------------------------------ use the renderpass
        scaler_renderpass.set_pipeline(&self.scaler_render_pipeline);
        scaler_renderpass.set_bind_group(0, Some(&self.scaler_bind_group), &[]);
        scaler_renderpass.set_vertex_buffer(0, self.fullscreen_vertex_buffer.slice(..));
        scaler_renderpass.set_viewport(
            viewport_xywh[0],
            viewport_xywh[1],
            viewport_xywh[2],
            viewport_xywh[3],
            0.0,
            1.0,
        );
        scaler_renderpass.draw(0..3, 0..1);

        // ------------------------------------------------------------------------------------------ end the renderpass
        drop(scaler_renderpass);

        // ------------------------------------------------------------------ submit the command in the queue to execute
        self.queue.submit([encoder.finish()]);
        self.window.pre_present_notify();
        surface_texture.present();
    }

    fn calc_ratio(&self) -> [f32; 4] {
        let w = LOGIC_WIDTH as f32;
        let h = LOGIC_HEIGHT as f32;

        let surface_w = self.window.inner_size().width as f32;
        let surface_h = self.window.inner_size().height as f32;

        let ratio = h / w;
        let surface_ratio = surface_h / surface_w;

        let ratio = if ratio <= surface_ratio {
            surface_w / w
        } else {
            surface_h / h
        };

        let w = w * ratio;
        let h = h * ratio;

        let x = (surface_w - w) / 2.0;
        let y = (surface_h - h) / 2.0;

        [x, y, w, h]
    }

    #[allow(unused)]
    pub fn get_window(&self) -> &Window {
        &self.window
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.configure_surface();
    }
}

fn texture_builder(device: &wgpu::Device, label: &str, size: &Size) -> wgpu::TextureView {
    // ----------------------------------------------------------------------------------- new texture to draw onto
    let new_texture_size = wgpu::Extent3d {
        width: size.width,
        height: size.height,
        depth_or_array_layers: 1,
    };

    let new_texture = device.create_texture(&wgpu::wgt::TextureDescriptor {
        label: Some(label),
        size: new_texture_size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::TEXTURE_BINDING
            | wgpu::TextureUsages::RENDER_ATTACHMENT
            | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });

    // ------------------------------------------------------------------------------------------- new texture view
    new_texture.create_view(&wgpu::TextureViewDescriptor::default())
}

// ---------------------------------------------------------------------- creates simple sample and draw builder
fn simple_bind_group_builder(
    device: &wgpu::Device,
    label: &str,
    bind_group_layout: &wgpu::BindGroupLayout,
    resource: &wgpu::TextureView,
    sampler: &wgpu::Sampler,
) -> wgpu::BindGroup {
    let new_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some(label),
        layout: &bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&resource),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(&sampler),
            },
        ],
    });
    new_bind_group
}

// ---------------------------------------------------------------------- creates simple sample and blend builder
fn blend_bind_group_builder(
    device: &wgpu::Device,
    label: &str,
    bind_group_layout: &wgpu::BindGroupLayout,
    resource_1: &wgpu::TextureView,
    resource_2: &wgpu::TextureView,
    sampler: &wgpu::Sampler,
) -> wgpu::BindGroup {
    let new_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some(label),
        layout: &bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Sampler(&sampler),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::TextureView(&resource_1),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: wgpu::BindingResource::TextureView(&resource_2),
            },
        ],
    });
    new_bind_group
}

fn pipeline_builder(
    device: &wgpu::Device,
    label: &str,
    bind_group_layout: &wgpu::BindGroupLayout,
    shader: &wgpu::ShaderModule,
) -> wgpu::RenderPipeline {
    let new_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some(label),
        bind_group_layouts: &[Some(&bind_group_layout)],
        immediate_size: 0,
    });

    let new_render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some(label),
        layout: Some(&new_pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some("vs_main"),
            buffers: &[Vertex::desc()],
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some("fs_main"),
            compilation_options: Default::default(),
            targets: &[Some(wgpu::ColorTargetState {
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview_mask: None,
        cache: None,
    });
    new_render_pipeline
}

fn resource_texture_builder(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    resource_bytes: &[u8],
    label: &str,
) -> wgpu::TextureView {
    // ------------------------------------------------------------------------------------------------------- image
    let resource_image = image::load_from_memory(resource_bytes).unwrap();

    let resource_rgba = resource_image.to_rgba8();
    let dimentions = resource_image.dimensions();

    // --------------------------------------------------------------------------------------------- resource texture
    let resource_texture_size = wgpu::Extent3d {
        width: dimentions.0,
        height: dimentions.1,
        depth_or_array_layers: 1,
    };

    let resource_texture = device.create_texture(&wgpu::wgt::TextureDescriptor {
        label: Some(label),
        size: resource_texture_size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });

    queue.write_texture(
        wgpu::TexelCopyTextureInfo {
            texture: &resource_texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        &resource_rgba,
        wgpu::TexelCopyBufferLayout {
            offset: 0,
            bytes_per_row: Some(4 * dimentions.0),
            rows_per_image: Some(dimentions.1),
        },
        resource_texture_size,
    );

    // ------------------------------------------------------------------------------------------------ texture view
    resource_texture.create_view(&wgpu::TextureViewDescriptor::default())
}
