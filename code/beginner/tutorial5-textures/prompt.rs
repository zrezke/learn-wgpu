I've got some code that displays an NV12 image using wgpu. The image looks almost alright, just the colors are slightly off, it looks a bit brighter and a little bit orangy. What may be wrong?

impl State {
    async fn new(window: Window) -> Self {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });

        let surface = (unsafe { instance.create_surface(&window) }).unwrap();

        let adapter = instance
            .request_adapter(
                &(wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::default(),
                    compatible_surface: Some(&surface),
                    force_fallback_adapter: false,
                })
            ).await
            .unwrap();
        let (device, queue) = adapter
            .request_device(
                &(wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    // WebGL doesn't support all of wgpu's features, so if
                    // we're building for the web we'll have to disable some.
                    limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                }),
                None // Trace path
            ).await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats
            .iter()
            .copied()
            .find(|f| f.describe().srgb)
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);
        let Ok(diffuse_bytes) = std::fs::read("./frame.nv12") else {
            panic!("Couldn't load frame!");
        };
        let diffuse_texture = texture::Texture
            ::from_bytes(&device, &queue, &diffuse_bytes, "frame.png")
            .unwrap();

        let texture_bind_group_layout = device.create_bind_group_layout(
            &(wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Uint,
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
                label: Some("texture_bind_group_layout"),
            })
        );

        let diffuse_bind_group = device.create_bind_group(
            &(wgpu::BindGroupDescriptor {
                layout: &texture_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
                    },
                ],
                label: Some("diffuse_bind_group"),
            })
        );

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let render_pipeline_layout = device.create_pipeline_layout(
            &(wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&texture_bind_group_layout],
                push_constant_ranges: &[],
            })
        );

        let render_pipeline = device.create_render_pipeline(
            &(wgpu::RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    // buffers: &[Vertex::desc()],
                    buffers: &[],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &[
                        Some(wgpu::ColorTargetState {
                            format: config.format,
                            blend: Some(wgpu::BlendState {
                                color: wgpu::BlendComponent::REPLACE,
                                alpha: wgpu::BlendComponent::REPLACE,
                            }),
                            write_mask: wgpu::ColorWrites::ALL,
                        }),
                    ],
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleStrip,
                    cull_mode: None,
                    ..Default::default()
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview: None,
            })
        );
        Self {
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
            diffuse_texture,
            diffuse_bind_group,
            window,
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    #[allow(unused_variables)]
    fn input(&mut self, event: &WindowEvent) -> bool {
        false
    }

    fn update(&mut self) {}

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(
            &(wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            })
        );

        {
            let mut render_pass = encoder.begin_render_pass(
                &(wgpu::RenderPassDescriptor {
                    label: Some("Render Pass"),
                    color_attachments: &[
                        Some(wgpu::RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color {
                                    r: 0.1,
                                    g: 0.2,
                                    b: 0.3,
                                    a: 1.0,
                                }),
                                store: true,
                            },
                        }),
                    ],
                    depth_stencil_attachment: None,
                })
            );

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
            render_pass.draw(0..4, 0..1);
        }

        self.queue.submit(iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}


pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}


This is how the texture is created
impl Texture {

    pub fn _from_bytes(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        img: &[u8],
        label: Option<&str>,
    ) -> Result<Self> {
        // let rgba = img.to_rgba8();
        // let dimensions = img.dimensions();
        let size = wgpu::Extent3d {
            width: 1920,
            height: (1080.0 * 1.5) as u32,
            depth_or_array_layers: 1,
        };

        let format = wgpu::TextureFormat::R8Uint;//wgpu::TextureFormat::Rgba8UnormSrgb;
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });

        queue.write_texture(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            img,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: NonZeroU32::new(1920),
                rows_per_image: NonZeroU32::new((1080.0 * 1.5) as u32),
            },
            size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        Ok(Self {
            texture,
            view,
            sampler,
        })
    }

}

Shader:
fn _decode_nv12(texture: texture_2d<u32>, in_tex_coords: vec2<f32>) -> vec4<f32> {
    let texture_dim = vec2<f32>(textureDimensions(texture).xy); // 1920 , 1080 * 1.5
    let uv_offset = u32(floor(texture_dim.y / 1.5));
    let uv_row = u32(floor(in_tex_coords.y * texture_dim.y) / 2.0);
    var uv_col = u32(floor(in_tex_coords.x * texture_dim.x / 2.0)) * 2u;

    let tex_coords = vec2<f32>(in_tex_coords * vec2<f32>(texture_dim.x, texture_dim.y));
    let coords = vec2<u32>(floor(tex_coords.xy));
    var y = f32(textureLoad(texture, coords, 0).r);
    var u = (f32(textureLoad(texture, vec2<u32>(u32(uv_col), uv_offset + uv_row), 0).r));
    var v = (f32(textureLoad(texture, vec2<u32>((u32(uv_col) + 1u), uv_offset + uv_row), 0).r));

    u = (u - 128.0);
    v = (v - 128.0);

    y = (y - 16.0);

    var red = y + 1.402 * v;
    var green = y  - (0.344 * u + 0.714 * v);
    var blue = y + 1.772 * u;

    // return vec4(0.0, 0.0, 0.0, 1.0);
    return vec4(red / 255.0, green / 255.0, blue / 255.0, 1.0);
}
