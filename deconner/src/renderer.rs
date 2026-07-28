use image::GenericImageView;
use std::borrow::Cow;
use std::mem;
use wgpu::util::DeviceExt;

use crate::Arc;
use crate::{OwnedDisplayHandle, Window};

const RECT_HALF: f32 = 0.75;
const LOGIC_WIDTH: u32 = 100;
const LOGIC_HEIGHT: u32 = 100;

// ------------------------------------------------------------------------------------------------ data for index buffer
const INDICES: &[u16] = &[0, 1, 2, /**/ 1, 3, 2];

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
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

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

// ---------------------------------------------------------------------------------------- data for effect vertex buffer
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

// ------------------------------------------------------------------------------------------------------- renderer state
pub struct Renderer {
    instance: wgpu::Instance,
    window: Arc<Window>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    size: winit::dpi::PhysicalSize<u32>,
    surface: wgpu::Surface<'static>,
    surface_format: wgpu::TextureFormat,

    uniform_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,

    mid_vertex_buffer: wgpu::Buffer,
    effect_vertex_buffer: wgpu::Buffer,

    mid_texture_view: wgpu::TextureView,

    diffuse_bind_group: wgpu::BindGroup,
    effect_bind_group: wgpu::BindGroup,

    mid_render_pipeline: wgpu::RenderPipeline,
    effect_render_pipeline: wgpu::RenderPipeline,
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

        // ----------------------------------------------------------------------------------------------- uniform buffer

        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("uniform buffer time"),
            contents: &0.0_f32.to_ne_bytes(),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // ------------------------------------------------------------------------------------------------- index buffer
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        let num_indices = INDICES.len() as u32;

        // -------------------------------------------------------------------------------------------- mid vertex buffer
        let mid_vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("mid vertex buffer"),
            contents: bytemuck::cast_slice(MID_VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        // ----------------------------------------------------------------------------------------- effect vertex buffer
        let effect_vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("effect vertex buffer"),
            contents: bytemuck::cast_slice(FULLSCREEN_VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });
        // ----------------------------------------------------------------------------------------- surface to draw onto
        let surface = instance.create_surface(window.clone()).unwrap();
        let cap = surface.get_capabilities(&adapter);
        let surface_format = cap.formats[0];

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

        // -------------------------------------------------------------------------------------------------------- image
        let diffuse_bytes = include_bytes!("../img/fish.png");
        let diffuse_image = image::load_from_memory(diffuse_bytes).unwrap();

        let diffuse_rgba = diffuse_image.to_rgba8();
        let dimentions = diffuse_image.dimensions();

        // ---------------------------------------------------------------------------------------------- diffuse texture
        let diffuse_texture_size = wgpu::Extent3d {
            width: dimentions.0,
            height: dimentions.1,
            depth_or_array_layers: 1,
        };

        let diffuse_texture = device.create_texture(&wgpu::wgt::TextureDescriptor {
            label: Some("diffuse_texture"),
            size: diffuse_texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &diffuse_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &diffuse_rgba,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimentions.0),
                rows_per_image: Some(dimentions.1),
            },
            diffuse_texture_size,
        );

        // ------------------------------------------------------------------------------------------------- texture view
        let diffuse_texture_view =
            diffuse_texture.create_view(&wgpu::TextureViewDescriptor::default());

        // ------------------------------------------------------------------------------------------- texture bind group
        let diffuse_texture_bind_group_lauout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("diffuse texture bind group layout"),
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

        let diffuse_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("diffuse bind group"),
            layout: &diffuse_texture_bind_group_lauout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&diffuse_texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler_nearest),
                },
            ],
        });

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

        // -------------------------------------------------------------------------------------------- effect bind group
        let effect_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("effect bind group layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: wgpu::BufferSize::new(
                                std::mem::size_of::<f32>() as u64
                            ),
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

        let effect_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("effect bind group"),
            layout: &effect_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: &uniform_buffer,
                        offset: 0,
                        size: None,
                    }),
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

        // ------------------------------------------------------------------------------------------------- mid pipeline
        let mid_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("render pipeline layout for mid texture"),
            bind_group_layouts: &[Some(&diffuse_texture_bind_group_lauout)],
            immediate_size: 0,
        });

        let mid_render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("render pipeline for mid texture"),
            layout: Some(&mid_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
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

        // ------------------------------------------------------------------------ pipeline to sample mid and add effect
        let effect_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("render pipeline layout for surface"),
                bind_group_layouts: &[Some(&effect_bind_group_layout)],
                immediate_size: 0,
            });

        let swapchain_capabilities = surface.get_capabilities(&adapter);
        let swapchain_format = swapchain_capabilities.formats[0];

        let effect_render_pipeline =
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("render pipeline for effect"),
                layout: Some(&effect_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: Some("vs_main"),
                    buffers: &[Vertex::desc()],
                    compilation_options: Default::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: Some("fs_effect"),
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

            mid_vertex_buffer,
            effect_vertex_buffer,

            uniform_buffer,
            index_buffer,
            num_indices,

            mid_texture_view,

            diffuse_bind_group,
            effect_bind_group,

            mid_render_pipeline,
            effect_render_pipeline,
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

        let vieport_xywh = self.calc_ratio();

        // ------------------------------------------------------------------------------------------- renderpass for mid
        let mut mid_renderpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("mid renderpass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &self.mid_texture_view,
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

        // ------------------------------------------------------------------------------------------- use the renderpass
        mid_renderpass.set_pipeline(&self.mid_render_pipeline);
        mid_renderpass.set_bind_group(0, Some(&self.diffuse_bind_group), &[]);
        mid_renderpass.set_vertex_buffer(0, self.mid_vertex_buffer.slice(..));
        mid_renderpass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        mid_renderpass.set_viewport(0.0, 0.0, LOGIC_WIDTH as f32, LOGIC_HEIGHT as f32, 0.0, 1.0);
        mid_renderpass.draw_indexed(0..self.num_indices, 0, 0..1);

        // ------------------------------------------------------------------------------------------- end the renderpass
        drop(mid_renderpass);

        // ---------------------------------------------------------------------------------------- renderpass for effect
        let mut effect_renderpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("effect renderpass"),
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

        // ------------------------------------------------------------------------------------------- use the renderpass
        effect_renderpass.set_pipeline(&self.effect_render_pipeline);
        effect_renderpass.set_bind_group(0, Some(&self.effect_bind_group), &[]);
        effect_renderpass.set_vertex_buffer(0, self.effect_vertex_buffer.slice(..));
        effect_renderpass.set_viewport(
            vieport_xywh[0],
            vieport_xywh[1],
            vieport_xywh[2],
            vieport_xywh[3],
            0.0,
            1.0,
        );
        effect_renderpass.draw(0..3, 0..1);

        // ------------------------------------------------------------------------------------------- end the renderpass
        drop(effect_renderpass);

        // ------------------------------------------------------------------- submit the command in the queue to execute
        self.queue.submit([encoder.finish()]);
        self.window.pre_present_notify();
        surface_texture.present();
    }

    fn calc_ratio(&self) -> [f32; 4] {
        let mid_w = LOGIC_WIDTH as f32;
        let mid_h = LOGIC_HEIGHT as f32;

        let surface_w = self.window.inner_size().width as f32;
        let surface_h = self.window.inner_size().height as f32;

        let mid_ratio = mid_h / mid_w;
        let surface_ratio = surface_h / surface_w;

        let ratio = if mid_ratio <= surface_ratio {
            surface_w / mid_w
        } else {
            surface_h / mid_h
        };

        let w = mid_w * ratio;
        let h = mid_h * ratio;

        let x = (surface_w - w) / 2.0;
        let y = (surface_h - h) / 2.0;

        [x, y, w, h]
    }

    pub fn get_window(&self) -> &Window {
        &self.window
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.configure_surface();
    }

    pub fn update(&self, time: f32) {
        self.queue
            .write_buffer(&self.uniform_buffer, 0, &time.to_ne_bytes());
    }
}
