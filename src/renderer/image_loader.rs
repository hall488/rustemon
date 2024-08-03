use anyhow::{Context, Result};
use image::GenericImageView;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;
use std::collections::HashMap;

pub struct ImageData {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

impl ImageData {
    fn from_png(path: &str) -> Result<Self> {
        let img = image::open(path).context("Failed to open image")?;
        let rgba = img.to_rgba8();
        let dimensions = img.dimensions();
        Ok(Self {
            width: dimensions.0,
            height: dimensions.1,
            data: rgba.into_raw(),
        })
    }

    fn from_bin(path: &str) -> Result<Self> {
        let mut file = File::open(path).context("Failed to open binary file")?;
        let mut buf_reader = BufReader::new(file);

        let mut dimensions = [0u32; 2];
        buf_reader.read_exact(bytemuck::cast_slice_mut(&mut dimensions)).context("Failed to read dimensions")?;
        let (width, height) = (dimensions[0], dimensions[1]);

        let mut data = vec![0u8; (width * height * 4) as usize];
        buf_reader.read_exact(&mut data).context("Failed to read image data")?;

        Ok(Self { width, height, data })
    }

    fn to_bin(&self, path: &str) -> Result<()> {
        let file = OpenOptions::new().write(true).create(true).open(path).context("Failed to create binary file")?;
        let mut buf_writer = BufWriter::new(file);

        buf_writer.write_all(bytemuck::cast_slice(&[self.width, self.height]))?;
        buf_writer.write_all(&self.data)?;

        Ok(())
    }
}


fn load_or_cache_image(base_name: &str) -> Result<ImageData> {
    let png_path = format!("{}.png", base_name);
    let bin_path = format!("{}.bin", base_name);

    if Path::new(&bin_path).exists() && false {
        println!("Loading from binary file: {}", bin_path);
        ImageData::from_bin(&bin_path)
    } else {
        println!("Loading from PNG and creating binary file: {}", png_path);
        let image_data = ImageData::from_png(&png_path)?;
        image_data.to_bin(&bin_path)?;
        Ok(image_data)
    }
}

pub fn load_images(base_names: &[&str], assets_folder: &str) -> Result<HashMap<String, ImageData>> {
    let mut images = HashMap::new();

    for &base_name in base_names {
        //concat the base name with the assets folder
        let path = format!("{}/{}", assets_folder, base_name);
        let image_data = load_or_cache_image(&path)?;
        images.insert(base_name.to_string(), image_data);
    }

    Ok(images)
}
