use std::{any::Any, iter};
use anyhow::*;
use std::sync::Arc;
use wgpu::util::DeviceExt;
use winit::window::Window;
use camera::{Camera, CameraUniform, ConfigUniform};
use instance::Instance;
use pipeline::create_pipeline;
use texture_manager::{TextureManager, Atlas};
use image_loader::ImageData;
use std::collections::HashMap;
use sprite::Sprite;
use window_map::WindowMapUniform;

pub mod camera;
mod vertex;
mod texture;
mod pipeline;
pub mod instance;
pub mod texture_manager;
pub mod image_loader;
pub mod sprite;
pub mod window_map;

pub struct Renderer {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    pub camera: Camera,
    camera_uniform: CameraUniform,
    camera_buffer: wgpu::Buffer,
    config_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    pub camera_controller: camera::CameraController,
    instance_buffer: wgpu::Buffer,
    num_instances: u32,
    staging_buffer: wgpu::Buffer,
    pub texture_manager: TextureManager,
    pub texture_map: HashMap<String, Atlas>,
    window_map_uniform: WindowMapUniform,
    window_map_buffer: wgpu::Buffer,
    window_map_bind_group: wgpu::BindGroup,
    pub images: HashMap<String, image_loader::ImageData>,
}

impl Renderer {
    pub async fn new(window: Arc<Window>) -> Renderer {

        let size = window.inner_size();

        // The instance is a handle to our GPU
        // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        //load arc window into surface
        let surface = instance.create_surface(window).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let limits = adapter.limits();
        let max_texture_array_layers = limits.max_texture_array_layers;
        println!("Max texture array layers: {}", max_texture_array_layers);

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    // WebGL doesn't support all of wgpu's features, so if
                    // we're building for the web we'll have to disable some.
                    required_limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        limits
                    },
                },
                None, // Trace path
            )
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);

        //let diffuse_bytes = include_bytes!("/home/chris/Downloads/pokemon_johto_ts.png");
        //let diffuse_texture = texture::Texture::from_bytes(&device, &queue, diffuse_bytes, "diffuse_texture").unwrap();

        let mut texture_manager = TextureManager::new(&device, &queue, 16, (1024, 8000));
        let mut texture_map = HashMap::new();

        let mut add_texture = |name: &str, grid_w: u32, grid_h: u32, path: &str| {
            texture_map.insert(name.to_string(), texture_manager.load_texture(path, grid_w, grid_h, &queue, &device).expect(""));
        };

        add_texture("landing", 16, 16, "/home/chris/games/SirSquare/assets/landing.png");
        add_texture("player", 16, 16, "/home/chris/games/SirSquare/assets/player.png");
        add_texture("menu", 16, 16, "/home/chris/games/SirSquare/assets/menu.png");
        add_texture("battle", 16, 16, "/home/chris/games/SirSquare/assets/battle.png");
        add_texture("pokemon_back", 32, 32, "/home/chris/games/SirSquare/assets/pokemon_back.png");
        add_texture("pokemon_front", 32, 32, "/home/chris/games/SirSquare/assets/pokemon_front.png");
        add_texture("party", 16, 16, "/home/chris/games/SirSquare/assets/party.png");
        add_texture("pokemon_party", 32, 32, "/home/chris/games/SirSquare/assets/pokemon_party.png");
        add_texture("white_font", 7, 11, "/home/chris/games/SirSquare/assets/white_font.png");
        add_texture("black_font", 7, 11, "/home/chris/games/SirSquare/assets/black_font.png");
        add_texture("debug", 16, 16, "/home/chris/games/SirSquare/assets/debug.png");
        add_texture("npcs", 16, 16, "/home/chris/games/SirSquare/assets/npcs.png");

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let camera = Camera {
            eye: (0.0, 0.0, 949.0).into(),
            // have it look at the origin
            target: (0.0, 0.0, 0.0).into(),
            // which way is "up"
            up: cgmath::Vector3::unit_y(),
            left: -0.5, // Left bound of the field
            right: 0.5, // Right bound of the field
            bottom: -0.5, // Bottom bound of the field
            top: 0.5, // Top bound of the field
            znear: 0.1, // Near clipping plane
            zfar: 100.0, // Far clipping plane
            aspect: 240.0 / 160.0,
        };

        let mut camera_uniform = CameraUniform::new();
        camera_uniform.update_view_proj(&camera);

        let config_uniform = ConfigUniform {
            apply_camera: 1, // 1 for true, 0 for false
            _padding: 0,
        };

        let camera_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Camera Buffer"),
                contents: bytemuck::cast_slice(&[camera_uniform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );

        let config_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Config Buffer"),
            contents: bytemuck::cast_slice(&[config_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let camera_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
            label: Some("camera_bind_group_layout"),
        });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: camera_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: config_buffer.as_entire_binding(),
                },
            ],
            label: Some("camera_bind_group"),
        });

        let camera_controller = camera::CameraController::new(1.0);

        println!("Window size: {:?}", size);
        // Initialize the WindowMapUniform
        let window_map_uniform = WindowMapUniform::new(size.width as f32, size.height as f32);

        let window_map_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("WindowMap Buffer"),
                contents: bytemuck::cast_slice(&[window_map_uniform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );

        let window_map_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
            label: Some("window_map_bind_group_layout"),
        });

        let window_map_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &window_map_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: window_map_buffer.as_entire_binding(),
                },
            ],
            label: Some("window_map_bind_group"),
        });

        let render_pipeline_layout =
        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[&texture_manager.bind_group_layout, &camera_bind_group_layout, &window_map_bind_group_layout],
            push_constant_ranges: &[],
        });

        let render_pipeline = create_pipeline(
            &device,
            &shader,
            &config,
            &render_pipeline_layout,
        );

        let (vertices, indices) = vertex::get_quad();

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        let num_indices = indices.len() as u32;


        let mut instances : Vec<Instance> = Vec::new();
        //print name of each layer
        //for each layer in layers
        instances.push(Instance {
            model: cgmath::Matrix4::from_translation(cgmath::Vector3::new(0.0, 0.0, 0.0)).into(),
            tex_index: 1,
            atlas_index: 0,
        });

        let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Instance Buffer"),
            contents: bytemuck::cast_slice(&instances),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_SRC,
        });

        let num_instances = instances.len() as u32;

        let staging_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Staging Buffer"),
            size: (instances.len() * std::mem::size_of::<Instance>()) as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let paths = vec![
            "landing",
            "pokecenter",
            "pokemart",
            "menu",
            "house_1",
            "battle",
            "gym",
        ];

        let images = image_loader::load_images(&paths, "/home/chris/games/SirSquare/assets").unwrap();

        Self {
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
            vertex_buffer,
            index_buffer,
            num_indices,
            camera,
            camera_uniform,
            config_buffer,
            camera_buffer,
            camera_bind_group,
            camera_controller,
            instance_buffer,
            num_instances,
            staging_buffer,
            texture_manager,
            texture_map,
            window_map_uniform,
            window_map_buffer,
            window_map_bind_group,
            images,
        }
    }

    pub fn create_sprite(
        &self,
        x: f32,
        y: f32,
        tex_x: u32,
        tex_y: u32,
        tex_w: u32,
        tex_h: u32,
        texture_name: &str,
        scale_x: f32,
        scale_y: f32,
    ) -> Result<Sprite> {
        let atlas = self.texture_map.get(texture_name)
            .ok_or_else(|| anyhow!("Texture name not found"))?;

        //println!("Atlas: {:?}", atlas);
        Ok(Sprite::new(
            x,
            y,
            tex_x,
            tex_y,
            tex_w,
            tex_h,
            atlas.index,
            atlas.texture_width,
            atlas.texture_height,
            atlas.tile_width,
            atlas.tile_height,
            scale_x,
            scale_y,
        ))
    }

    pub fn get_atlas(&self, texture_name: &str) -> Result<&Atlas> {
        let atlas = self.texture_map.get(texture_name)
            .ok_or_else(|| anyhow!("Texture name not found"))?;
        Ok(atlas)
    }

    pub fn load_texture(&mut self, texture_path: &str) -> Result<Atlas> {
        let idx = self.texture_manager.load_texture(texture_path, 16, 16, &self.queue, &self.device)?;
        Ok(idx)
    }

    pub fn update_texture(&mut self, atlas_index: u32, name: &str, grid_w: u32, grid_h: u32) -> Result<()> {
        //remove previous texture at atlas index from texture map

        let image = self.images.get(name).ok_or_else(|| anyhow!("Image not found"))?;
        println!("Updating texture: {}", name);
        let atlas = self.texture_manager.update_texture(atlas_index, image, grid_w, grid_h, &self.queue, &self.device).expect("");
        println!("{}", atlas);
        self.texture_map.insert(name.to_string(), atlas);

        Ok(())
    }

    pub fn load_single(&mut self, texture_path: &str) -> Result<()> {
        let _ = self.texture_manager.load_single(texture_path, &self.queue, &self.device);
        Ok(())
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn update(&mut self, position: cgmath::Vector3<f32>) {
        self.camera_controller.update_camera(&mut self.camera, position);
        self.camera_uniform.update_view_proj(&self.camera);
        self.queue.write_buffer(&self.camera_buffer, 0, bytemuck::cast_slice(&[self.camera_uniform]));
        self.queue.write_buffer(&self.window_map_buffer, 0, bytemuck::cast_slice(&[self.window_map_uniform]));
    }

    pub fn render(&mut self, dynamic_instances: &[Instance], use_cam: bool) -> Result<()> {

        let config_uniform = ConfigUniform {
            apply_camera: if use_cam { 1 } else { 0 },
            _padding: 0,
        };
        self.queue.write_buffer(&self.config_buffer, 0, bytemuck::cast_slice(&[config_uniform]));
        // Combine static instances with additional dynamic instances (e.g., player)
        {
            let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Copy Encoder"),
            });
            encoder.copy_buffer_to_buffer(
                &self.instance_buffer,
                0,
                &self.staging_buffer,
                0,
                (self.num_instances * std::mem::size_of::<Instance>() as u32) as wgpu::BufferAddress,
            );
            self.queue.submit(std::iter::once(encoder.finish()));
        }

        // Wait for the copy to complete
        let buffer_slice = self.staging_buffer.slice(..);
        buffer_slice.map_async(wgpu::MapMode::Read, |result| {
            if let Err(e) = result {
                eprintln!("Failed to map buffer: {:?}", e);
            }
        });

        self.device.poll(wgpu::Maintain::Wait);

        let data = buffer_slice.get_mapped_range();
        let static_instances: &[Instance] = bytemuck::cast_slice(&data);

        // Combine static and dynamic instances
        let mut combined_instances = Vec::with_capacity(static_instances.len() + dynamic_instances.len());
        combined_instances.extend_from_slice(static_instances);
        combined_instances.extend_from_slice(dynamic_instances);

        // Unmap the staging buffer
        drop(data);
        self.staging_buffer.unmap();

        // Create a new instance buffer with the combined instances
        let instance_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Combined Instance Buffer"),
            contents: bytemuck::cast_slice(&combined_instances),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });


        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            // Ensure the correct bind group is set
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.texture_manager.bind_group, &[]);
            render_pass.set_bind_group(1, &self.camera_bind_group, &[]);
            render_pass.set_bind_group(2, &self.window_map_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(1, instance_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..self.num_indices, 0, 0..combined_instances.len() as u32);
        }

        self.queue.submit(iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

}

