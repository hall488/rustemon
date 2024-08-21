use anyhow::*;
use image::GenericImageView;
use wgpu::util::DeviceExt;
use std::time::Instant;
use std::fs::File;
use std::io::{BufReader, Read};
use super::image_loader::ImageData;
use std::fmt;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct AtlasInfo {
    atlas_width: u32,
    atlas_height: u32,
    tile_width: u32,
    tile_height: u32,
    texture_width: u32,
    texture_height: u32,
    _padding: u32,
    _padding2: u32,
}

#[derive(Debug, Clone)]
pub struct Atlas {
    pub cols: u32,
    pub rows: u32,
    pub tile_width: u32,
    pub tile_height: u32,
    pub texture_width: u32,
    pub texture_height: u32,
    pub index: u32,
}

impl fmt::Display for Atlas {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Atlas(cols: {}, rows: {}, tile_width: {}, tile_height: {}, texture_width: {}, texture_height: {}, index: {})",
               self.cols, self.rows, self.tile_width, self.tile_height, self.texture_width, self.texture_height, self.index)
    }
}

pub struct TextureManager {
    texture_array: wgpu::Texture,
    texture_array_view: wgpu::TextureView,
    sampler: wgpu::Sampler,
    pub bind_group: wgpu::BindGroup,
    pub bind_group_layout: wgpu::BindGroupLayout,
    next_layer_index: u32,
    atlas_infos: Vec<AtlasInfo>,
    atlas_info_buffer: wgpu::Buffer,
    texture_size: (u32, u32),
}

impl TextureManager {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue, max_textures: u32, texture_size: (u32, u32)) -> Self {
        let texture_array = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Texture Array"),
            size: wgpu::Extent3d {
                width: texture_size.0,
                height: texture_size.1,
                depth_or_array_layers: max_textures,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        let texture_array_view = texture_array.create_view(&wgpu::TextureViewDescriptor::default());

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2Array,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
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
            label: Some("texture_array_bind_group_layout"),
        });

        // Initialize atlas infos with the maximum number of textures
        let atlas_infos = vec![
            AtlasInfo {
                atlas_width: 0,
                atlas_height: 0,
                tile_width: 32, // Fixed texture width
                tile_height: 32, // Fixed texture height
                texture_width: texture_size.0,
                texture_height: texture_size.1,
                _padding: 0,
                _padding2: 0,
            };
            max_textures as usize
        ];

        let atlas_info_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Atlas Info Buffer"),
            contents: bytemuck::cast_slice(&atlas_infos),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture_array_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: atlas_info_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
            label: Some("texture_array_bind_group"),
        });

        Self {
            texture_array,
            texture_array_view,
            sampler,
            bind_group,
            bind_group_layout,
            next_layer_index: 0,
            atlas_infos,
            atlas_info_buffer,
            texture_size,
        }
    }

    pub fn load_single(&mut self, texture_path: &str , queue: &wgpu::Queue, device: &wgpu::Device) -> Result<()> {
        let img = image::open(texture_path).context("Failed to open texture image")?;
        let rgba = img.to_rgba8();
        let dimensions = img.dimensions();

        // Ensure we do not exceed the maximum number of textures in the array
        assert!(self.next_layer_index < self.texture_array.size().depth_or_array_layers);

        // Upload the texture data to the texture array
        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &self.texture_array,
                mip_level: 0,
                origin: wgpu::Origin3d {
                    x: 0,
                    y: 0,
                    z: self.next_layer_index,
                },
                aspect: wgpu::TextureAspect::All,
            },
            &rgba,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions.0),
                rows_per_image: Some(dimensions.1),
            },
            wgpu::Extent3d {
                width: dimensions.0,
                height: dimensions.1,
                depth_or_array_layers: 1,
            },
        );

        // Update the atlas info map and buffer
        // account for texture size of the texture array
        let atlas_info = AtlasInfo {
            atlas_width: dimensions.0,
            atlas_height: dimensions.1,
            tile_width: dimensions.0, // Fixed texture width
            tile_height: dimensions.1, // Fixed texture height
            texture_width: self.texture_size.0,
            texture_height: self.texture_size.1,
            _padding: 0,
            _padding2: 0,
        };

        self.atlas_infos[self.next_layer_index as usize] = atlas_info;

        // Update the buffer with the new atlas info
        queue.write_buffer(&self.atlas_info_buffer, 0, bytemuck::cast_slice(&self.atlas_infos));

        self.next_layer_index += 1;
        Ok(())
    }

    pub fn load_texture(&mut self, texture_path: &str, tile_width: u32, tile_height: u32, queue: &wgpu::Queue, device: &wgpu::Device) -> Result<Atlas> {
        let img = image::open(texture_path).context("Failed to open texture image")?;
        let rgba = img.to_rgba8();
        let dimensions = img.dimensions();

        // Ensure we do not exceed the maximum number of textures in the array
        assert!(self.next_layer_index < self.texture_array.size().depth_or_array_layers);

        // Upload the texture data to the texture array
        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &self.texture_array,
                mip_level: 0,
                origin: wgpu::Origin3d {
                    x: 0,
                    y: 0,
                    z: self.next_layer_index,
                },
                aspect: wgpu::TextureAspect::All,
            },
            &rgba,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions.0),
                rows_per_image: Some(dimensions.1),
            },
            wgpu::Extent3d {
                width: dimensions.0,
                height: dimensions.1,
                depth_or_array_layers: 1,
            },
        );

        // Update the atlas info map and buffer
        // account for texture size of the texture array
        let atlas_info = AtlasInfo {
            atlas_width: dimensions.0,
            atlas_height: dimensions.1,
            tile_width, // Fixed texture width
            tile_height, // Fixed texture height
            texture_width: self.texture_size.0,
            texture_height: self.texture_size.1,
            _padding: 0,
            _padding2: 0,
        };

        self.atlas_infos[self.next_layer_index as usize] = atlas_info;

        // Update the buffer with the new atlas info
        queue.write_buffer(&self.atlas_info_buffer, 0, bytemuck::cast_slice(&self.atlas_infos));

        let atlas = Atlas {
            cols: dimensions.0 / tile_width,
            rows: dimensions.1 / tile_height,
            tile_width,
            tile_height,
            texture_width: dimensions.0,
            texture_height: dimensions.1,
            index: self.next_layer_index,
        };

        self.next_layer_index += 1;
        Ok(atlas)
    }

    pub fn update_texture(&mut self, layer_index: u32, image: &ImageData, tile_width: u32, tile_height: u32, queue: &wgpu::Queue, device: &wgpu::Device) -> Result<Atlas> {

        let rgba = &image.data;
        let dimensions = (image.width, image.height);

        // Ensure we do not exceed the maximum number of textures in the array
        let start_time = Instant::now();
        assert!(layer_index < self.texture_array.size().depth_or_array_layers);
        println!("Assertion check time: {:?}", start_time.elapsed());

        // Update the texture in the texture array
        let start_time = Instant::now();
        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &self.texture_array,
                mip_level: 0,
                origin: wgpu::Origin3d {
                    x: 0,
                    y: 0,
                    z: layer_index,
                },
                aspect: wgpu::TextureAspect::All,
            },
            &rgba,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions.0),
                rows_per_image: Some(dimensions.1),
            },
            wgpu::Extent3d {
                width: dimensions.0,
                height: dimensions.1,
                depth_or_array_layers: 1,
            },
        );
        println!("Texture update time: {:?}", start_time.elapsed());

        let start_time = Instant::now();
        self.atlas_infos[layer_index as usize] = AtlasInfo {
            atlas_width: dimensions.0,
            atlas_height: dimensions.1,
            tile_width: 32, // Fixed texture width
            tile_height: 32, // Fixed texture height
            texture_width: self.texture_size.0,
            texture_height: self.texture_size.1,
            _padding: 0,
            _padding2: 0,
        };
        println!("Atlas info update time: {:?}", start_time.elapsed());

        let start_time = Instant::now();
        queue.write_buffer(&self.atlas_info_buffer, 0, bytemuck::cast_slice(&self.atlas_infos));
        println!("Buffer write time: {:?}", start_time.elapsed());

        let atlas = Atlas {
            cols: dimensions.0 / tile_width,
            rows: dimensions.1 / tile_height,
            tile_width,
            tile_height,
            texture_width: dimensions.0,
            texture_height: dimensions.1,
            index: layer_index,
        };
        Ok(atlas)
    }

    pub fn get_bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }
}
