use std::num::NonZeroU32;
use anyhow::*;
use image::GenericImageView;

pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}

fn get_rgb_dimensions(img_dim: (u32, u32)) -> (u32, u32) {
    let original_elements = img_dim.0 as f64 * img_dim.1 as f64 * 3.0;
    let new_elements = (original_elements / 4.0).sqrt();  // sqrt to get width = height
    let new_dim = new_elements.round() as u32;  // round to nearest integer

    (new_dim, new_dim)
}


// 3840x2160
impl Texture {
    pub fn from_bytes(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bytes: &[u8],
        label: &str,
    ) -> Result<Self> {
        // let img = image::load_from_memory(bytes)?;
        // println!("Image len: {:?}", img.as_bytes().len());
        Self::_from_bytes(device, queue, bytes, Some(label))
    }


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

    pub fn from_image(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        img: &image::DynamicImage,
        label: Option<&str>,
    ) -> Result<Self> {
        // let rgba = img.to_rgba8();
        let rgba = img.to_rgb8();
        // let dimensions = get_rgb_dimensions(img.dimensions());
        // let dimensions = img.dimensions();
        let dimensions = get_rgb_dimensions(img.dimensions());
        println!("Dimensions: {:?}", dimensions);
        let size = wgpu::Extent3d {
            width: img.width(),
            height: (img.height() as f32 * 1.5) as u32,
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
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        queue.write_texture(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            &rgba,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: NonZeroU32::new(img.width()),
                rows_per_image: NonZeroU32::new((img.height() as f32 * 1.5) as u32),
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
