use glam::{Mat4, Vec3};
use std::borrow::Cow;
use std::mem;
use wgpu::util::DeviceExt;

use crate::Arc;
use crate::{OwnedDisplayHandle, Window};

const RECT_HALF: f32 = 0.25;
const LOGIC_WIDTH: u32 = 160;
const LOGIC_HEIGHT: u32 = 144;

//TODO add vertex_buffer/pipeline/draw for base texture

// --------------------------------------------------------------------------------------------- struct for vertex buffer
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 2],
    uv: [f32; 2],
}

// ------------------------------------------------------------------------------------ descriptor for VertexBufferLayout
impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x2];
    // 0 => Vertex::position, 1 => Vertex::uv

    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress, // use std::mem;
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

// ------------------------------------------------------------------------------------------ data for base vertex buffer

const BASE_VERTICES: &[Vertex] = &[
    Vertex {
        position: [-0.5, 0.5], // top left
        uv: [0.0, 0.0],
    },
    Vertex {
        position: [-0.5, -0.5], // bottom left
        uv: [0.0, 1.0],
    },
    Vertex {
        position: [0.5, 0.5], // top right
        uv: [1.0, 0.0],
    },
    Vertex {
        position: [0.5, -0.5], // bottom rightV
        uv: [1.0, 1.0],
    },
];

// ------------------------------------------------------------------------------------------- data for mid vertex buffer
const MID_VERTICES: &[Vertex] = &[
    Vertex {
        position: [-RECT_HALF, RECT_HALF], // top left
        uv: [0.0, 0.0],
    },
    Vertex {
        position: [-RECT_HALF, -RECT_HALF], // bottom left
        uv: [0.0, 1.0],
    },
    Vertex {
        position: [RECT_HALF, RECT_HALF], // top right
        uv: [1.0, 0.0],
    },
    Vertex {
        position: [RECT_HALF, -RECT_HALF], // bottom rightV
        uv: [1.0, 1.0],
    },
];

// ---------------------------------------------------------------------------------------- data for scaler vertex buffer

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

// ------------------------------------------------------------------------------------------------------- uniform buffer
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Uniforms {
    pub model_matricx: [[f32; 4]; 4],
}

// ------------------------------------------------------------------------------------------------ data for index buffer
const INDICES: &[u16] = &[0, 1, 2, /**/ 1, 3, 2];

// ------------------------------------------------------------------------------------------------------- renderer state
pub struct Renderer {
    instance: wgpu::Instance,
    window: Arc<Window>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    size: winit::dpi::PhysicalSize<u32>,
    surface: wgpu::Surface<'static>,
    surface_format: wgpu::TextureFormat,

    base_vertex_buffer: wgpu::Buffer,
    mid_vertex_buffer: wgpu::Buffer,
    scaler_vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    uniform_buffer: wgpu::Buffer,
    num_indices: u32,

    base_texture_view: wgpu::TextureView,
    mid_texture_view: wgpu::TextureView,
    blend_texture_view: wgpu::TextureView,

    mid_bind_group: wgpu::BindGroup,
    scaler_bind_group: wgpu::BindGroup,
    blend_bind_group: wgpu::BindGroup,

    base_render_pipeline: wgpu::RenderPipeline,
    mid_render_pipeline: wgpu::RenderPipeline,
    blend_render_pipeline: wgpu::RenderPipeline,
    scaler_render_pipeline: wgpu::RenderPipeline,
}

impl Renderer {
    pub async fn new(display: OwnedDisplayHandle, window: Arc<Window>) -> Renderer {
        // ----------------------------------------------------------------------------------- create a new wgpu instance
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_with_display_handle(
            Box::new(display),
        ));

        // ---------------------------------------------------------------------------------------------- physical device
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions::default())
            .await
            .unwrap();

        // ----------------------------------------------------------------------------------------------- logical device
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default())
            .await
            .unwrap();

        // ----------------------------------------------------------------------------------------------- size of window
        let size = window.inner_size();

        // ------------------------------------------------------------------------------------------------- load shaders
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
        });

        // ----------------------------------------------------------------------------------------- base vertex buffer
        let base_vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("base vertex buffer"),
            contents: bytemuck::cast_slice(BASE_VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        // -------------------------------------------------------------------------------------------- mid vertex buffer
        let mid_vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("mid vertex buffer"),
            contents: bytemuck::cast_slice(MID_VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        // ----------------------------------------------------------------------------------------- scaler vertex buffer
        let scaler_vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("scaler vertex buffer"),
            contents: bytemuck::cast_slice(FULLSCREEN_VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        // ------------------------------------------------------------------------------------------------- index buffer
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        let num_indices = INDICES.len() as u32;

        // ----------------------------------------------------------------------------------------------- uniform buffer
        let initial_uniforms = Uniforms {
            model_matricx: Mat4::IDENTITY.to_cols_array_2d(),
        };

        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("uniform buffer"),
            contents: bytemuck::cast_slice(&[initial_uniforms]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        // ----------------------------------------------------------------------------------------- surface to draw onto
        let surface = instance.create_surface(window.clone()).unwrap();
        let cap = surface.get_capabilities(&adapter);
        let surface_format = cap.formats[0];

        // ------------------------------------------------------------------------------------ base texture to draw onto
        let base_texture_size = wgpu::Extent3d {
            width: LOGIC_WIDTH,
            height: LOGIC_HEIGHT,
            depth_or_array_layers: 1,
        };

        let base_texture = device.create_texture(&wgpu::wgt::TextureDescriptor {
            label: Some("base texture"),
            size: base_texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        // -------------------------------------------------------------------------------------------- base texture view
        let base_texture_view = base_texture.create_view(&wgpu::TextureViewDescriptor::default());

        // ------------------------------------------------------------------------------------- mid texture to draw onto
        let mid_texture_size = wgpu::Extent3d {
            width: LOGIC_WIDTH,
            height: LOGIC_HEIGHT,
            depth_or_array_layers: 1,
        };

        let mid_texture = device.create_texture(&wgpu::wgt::TextureDescriptor {
            label: Some("mid texture"),
            size: mid_texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        // --------------------------------------------------------------------------------------------- mid texture view
        let mid_texture_view = mid_texture.create_view(&wgpu::TextureViewDescriptor::default());

        // ------------------------------------------------------------------------------------------------------ sampler
        let sampler_nearest = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::MipmapFilterMode::Nearest,
            ..Default::default()
        });

        // ----------------------------------------------------------------------------------- blend texture to draw onto
        let blend_texture_size = wgpu::Extent3d {
            width: LOGIC_WIDTH,
            height: LOGIC_HEIGHT,
            depth_or_array_layers: 1,
        };

        let blend_texture = device.create_texture(&wgpu::wgt::TextureDescriptor {
            label: Some("blend texture"),
            size: blend_texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        // -------------------------------------------------------------------------------------------- blend texture view
        let blend_texture_view = blend_texture.create_view(&wgpu::TextureViewDescriptor::default());

        // ----------------------------------------------------------------------------------------------- mid bind group

        let mid_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("mid bind group layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(
                            std::mem::size_of::<Uniforms>() as u64
                        ),
                    },
                    count: None,
                }],
            });

        let mid_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("mid bind group"),
            layout: &mid_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &uniform_buffer,
                    offset: 0,
                    size: None,
                }),
            }],
        });

        // -------------------------------------------------------------------------------------------- blend bind group
        let blend_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("blend bind group layout"),
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
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
            });

        let blend_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("blend bind group"),
            layout: &blend_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&base_texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&mid_texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(&sampler_nearest),
                },
            ],
        });

        // -------------------------------------------------------------------------------------------- scaler bind group
        let scaler_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("bind group layout"),
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

        let scaler_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("scaler bind group"),
            layout: &scaler_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&blend_texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler_nearest),
                },
            ],
        });

        // ------------------------------------------------------------------------------------------------- base pipeline
        let base_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("render pipeline layout for base texture"),
            bind_group_layouts: &[],
            immediate_size: 0,
        });

        let base_render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("render pipeline for base texture"),
            layout: Some(&base_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_base"),
                buffers: &[Vertex::desc()],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_base"),
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

        // ------------------------------------------------------------------------------------------------- mid pipeline
        let mid_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("render pipeline layout for mid texture"),
            bind_group_layouts: &[Some(&mid_bind_group_layout)],
            immediate_size: 0,
        });

        let mid_render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("render pipeline for mid texture"),
            layout: Some(&mid_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_mid"),
                buffers: &[Vertex::desc()],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_mid"),
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

        // --------------------------------------------------------------------- pipeline to sample base and mid to blend
        let blend_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("render pipeline layout for blend texture"),
                bind_group_layouts: &[Some(&blend_bind_group_layout)],
                immediate_size: 0,
            });

        let blend_render_pipeline =
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("render pipeline for surface"),
                layout: Some(&blend_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: Some("vs_blend"),
                    buffers: &[Vertex::desc()],
                    compilation_options: Default::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: Some("fs_blend"),
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

        // --------------------------------------------------------- final pipeline to sample mid and draw on the surface
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
                    module: &shader,
                    entry_point: Some("vs_scaler"),
                    buffers: &[Vertex::desc()],
                    compilation_options: Default::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: Some("fs_scaler"),
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

            base_vertex_buffer,
            mid_vertex_buffer,
            scaler_vertex_buffer,
            index_buffer,
            num_indices,
            uniform_buffer,

            base_texture_view,
            mid_texture_view,
            blend_texture_view,

            mid_bind_group,
            blend_bind_group,
            scaler_bind_group,

            base_render_pipeline,
            mid_render_pipeline,
            blend_render_pipeline,
            scaler_render_pipeline,
        };

        // ------------------------------------------------------------------------- configure surface for the first time
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
        // ---------------------------------------------------------------------------------- create surface texture view
        // NOTE: We must handle Timeout because the surface may be unavailable
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

        // ------------------------------------------------------------------------------------------ renderpass for base
        let mut base_renderpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &self.base_texture_view,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });

        // ------------------------------------------------------------------------------------------- end the renderpass
        base_renderpass.set_pipeline(&self.base_render_pipeline);
        base_renderpass.set_vertex_buffer(0, self.base_vertex_buffer.slice(..));
        base_renderpass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        base_renderpass.set_viewport(0.0, 0.0, LOGIC_WIDTH as f32, LOGIC_HEIGHT as f32, 0.0, 1.0);
        base_renderpass.draw_indexed(0..self.num_indices, 0, 0..1);
        drop(base_renderpass);

        // ------------------------------------------------------------------------------------------- renderpass for mid
        let mut mid_renderpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("mid renderpass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &self.mid_texture_view,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLUE),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });

        // ------------------------------------------------------------------------------------------- use the renderpass
        mid_renderpass.set_pipeline(&self.mid_render_pipeline);
        mid_renderpass.set_bind_group(0, Some(&self.mid_bind_group), &[]);
        mid_renderpass.set_vertex_buffer(0, self.mid_vertex_buffer.slice(..));
        mid_renderpass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        mid_renderpass.set_viewport(0.0, 0.0, LOGIC_WIDTH as f32, LOGIC_HEIGHT as f32, 0.0, 1.0);
        mid_renderpass.draw_indexed(0..self.num_indices, 0, 0..1);

        // ------------------------------------------------------------------------------------------- end the renderpass
        drop(mid_renderpass);

        // ----------------------------------------------------------------------------------------- renderpass for blend
        let mut blend_renderpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("blend renderpass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &self.blend_texture_view,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLUE),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });

        // ------------------------------------------------------------------------------------------- use the renderpass
        blend_renderpass.set_pipeline(&self.blend_render_pipeline);
        blend_renderpass.set_bind_group(0, Some(&self.blend_bind_group), &[]);
        blend_renderpass.set_vertex_buffer(0, self.scaler_vertex_buffer.slice(..));
        blend_renderpass.set_viewport(0.0, 0.0, LOGIC_WIDTH as f32, LOGIC_HEIGHT as f32, 0.0, 1.0);
        blend_renderpass.draw(0..3, 0..1);

        // ------------------------------------------------------------------------------------------- end the renderpass
        drop(blend_renderpass);

        // --------------------------------------------------------------------------------------- renderpass for surface
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

        let vieport_xywh = self.calc_ratio();

        // ------------------------------------------------------------------------------------------- use the renderpass
        scaler_renderpass.set_pipeline(&self.scaler_render_pipeline);
        scaler_renderpass.set_bind_group(0, Some(&self.scaler_bind_group), &[]);
        scaler_renderpass.set_vertex_buffer(0, self.scaler_vertex_buffer.slice(..));
        scaler_renderpass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        scaler_renderpass.set_viewport(
            vieport_xywh[0],
            vieport_xywh[1],
            vieport_xywh[2],
            vieport_xywh[3],
            0.0,
            1.0,
        );
        scaler_renderpass.draw(0..3, 0..1);

        // ------------------------------------------------------------------------------------------- end the renderpass
        drop(scaler_renderpass);

        // ------------------------------------------------------------------- submit the command in the queue to execute
        self.queue.submit([encoder.finish()]);
        self.window.pre_present_notify();
        surface_texture.present();
    }

    fn calc_ratio(&self) -> [f32; 4] {
        let mid_w = self.mid_texture_view.texture().size().width as i16;
        let mid_h = self.mid_texture_view.texture().size().height as i16;

        let surface_w = self.window.inner_size().width as i16;
        let surface_h = self.window.inner_size().height as i16;

        let mid_ratio = mid_h / mid_w;
        let surface_ratio = surface_h / surface_w;

        let ratio = if mid_ratio <= surface_ratio {
            surface_w / mid_w
        } else {
            surface_h / mid_h
        };

        let w = mid_w * ratio;
        let h = mid_h * ratio;

        let x = (surface_w - w) / 2;
        let y = (surface_h - h) / 2;

        [x as f32, y as f32, w as f32, h as f32]
    }

    pub fn get_window(&self) -> &Window {
        &self.window
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.configure_surface();
    }

    pub fn update(&self, direction: (f32, f32)) {
        let mut model = Mat4::IDENTITY;
        model *= Mat4::from_translation(Vec3::new(direction.0, direction.1, 0.0));

        self.queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::cast_slice(&[Uniforms {
                model_matricx: model.to_cols_array_2d(),
            }]),
        );
    }
}
